use rsa::{
  Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
  pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
  pkcs8::LineEnding,
  rand_core::OsRng,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::db::DBTrait;

pub struct PasswordState {
  key: RsaPrivateKey,
  pub pub_key: String,
  pub pepper: Vec<u8>,
}

impl PasswordState {
  pub fn decrypt(&self, message: &[u8]) -> Result<Vec<u8>, rsa::errors::Error> {
    self.key.decrypt(Pkcs1v15Encrypt, message)
  }

  pub async fn init(db: &DatabaseConnection) -> Self {
    let key = if let Ok(key) = db.tables().key().get_key_by_name("password".into()).await {
      RsaPrivateKey::from_pkcs1_pem(&key.private_key).expect("Failed to parse private password key")
    } else {
      let mut rng = OsRng {};
      let private_key = RsaPrivateKey::new(&mut rng, 4096).expect("Failed to create Rsa key");
      let key = private_key
        .to_pkcs1_pem(LineEnding::CRLF)
        .expect("Failed to export private key")
        .to_string();

      db.tables()
        .key()
        .create_key("password".into(), key.clone(), Uuid::new_v4())
        .await
        .expect("Failed to save key");

      private_key
    };

    let pub_key = RsaPublicKey::from(&key)
      .to_pkcs1_pem(LineEnding::CRLF)
      .expect("Failed to export Rsa Public Key");

    let pepper = std::env::var("AUTH_PEPPER")
      .expect("Failed to read Pepper")
      .as_bytes()
      .to_vec();
    if pepper.len() > 32 {
      panic!("Pepper is longer than 32 characters");
    }

    Self {
      key,
      pub_key,
      pepper,
    }
  }
}
