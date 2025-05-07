use chrono::{Duration, Utc};
use jsonwebtoken::{
  Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
  errors::{Error, ErrorKind},
};
use rocket::{
  Response, async_trait,
  http::{Cookie, SameSite},
  request::{FromRequest, Outcome, Request},
  response::Responder,
  tokio::sync::Mutex,
};
use rocket_okapi::{
  r#gen::OpenApiGenerator,
  okapi::{
    Map,
    openapi3::{ParameterValue, RefOr, Responses, SecurityScheme, SecuritySchemeData},
  },
  request::{OpenApiFromRequest, RequestHeaderInput},
  response::OpenApiResponderInner,
};
use rsa::{
  RsaPrivateKey, RsaPublicKey,
  pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
  pkcs8::LineEnding,
  rand_core::OsRng,
};
use schemars::schema::{InstanceType, SchemaObject, SingleOrVec};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use uuid::Uuid;

use crate::db::DBTrait;

use super::utils::jwt_from_request;

pub static COOKIE_NAME: &str = "token";

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
  pub exp: i64,
  pub iss: String,
  pub sub: Uuid,
}

#[derive(Default)]
pub struct JwtInvalidState {
  pub count: Mutex<i32>,
}

pub struct JwtState {
  header: Header,
  encoding_key: EncodingKey,
  decoding_key: DecodingKey,
  validation: Validation,
  pub iss: String,
  exp: i64,
}

impl JwtState {
  pub fn create_token<'c>(&self, uuid: Uuid) -> Result<Cookie<'c>, Error> {
    let exp = Utc::now()
      .checked_add_signed(Duration::seconds(self.exp))
      .ok_or(Error::from(ErrorKind::ExpiredSignature))?
      .timestamp();

    let claims = JwtClaims {
      exp,
      iss: self.iss.clone(),
      sub: uuid,
    };

    let token = encode(&self.header, &claims, &self.encoding_key)?;

    Ok(self.create_cookie(token, true))
  }

  pub fn create_cookie<'c>(&self, value: String, http: bool) -> Cookie<'c> {
    Cookie::build((COOKIE_NAME, value))
      .http_only(http)
      .max_age(rocket::time::Duration::seconds(self.exp))
      .same_site(SameSite::Lax)
      .secure(true)
      .build()
  }

  pub fn validate_token<C: DeserializeOwned>(&self, token: &str) -> Result<C, Error> {
    Ok(decode::<C>(token, &self.decoding_key, &self.validation)?.claims)
  }

  pub async fn init(db: &DatabaseConnection) -> Self {
    let iss = std::env::var("AUTH_ISSUER").expect("Failed to load JwtIssuer");
    let exp = std::env::var("AUTH_JWT_EXPIRATION")
      .expect("Failed to load JwtExpiration")
      .parse()
      .expect("Failed to parse JwtExpiration");

    let (key, kid) = if let Ok(key) = db.tables().key().get_key_by_name("jwt".into()).await {
      (key.private_key, key.id.to_string())
    } else {
      let mut rng = OsRng {};
      let private_key = RsaPrivateKey::new(&mut rng, 4096).expect("Failed to create Rsa key");
      let key = private_key
        .to_pkcs1_pem(LineEnding::CRLF)
        .expect("Failed to export private key")
        .to_string();

      let uuid = Uuid::new_v4();

      db.tables()
        .key()
        .create_key("jwt".into(), key.clone(), uuid)
        .await
        .expect("Failed to save key");

      (key, uuid.to_string())
    };

    let private_key = RsaPrivateKey::from_pkcs1_pem(&key).expect("Failed to load public key");
    let public_key = RsaPublicKey::from(private_key);
    let public_key_pem = public_key
      .to_pkcs1_pem(LineEnding::CRLF)
      .expect("Failed to export public key");

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.clone());

    let encoding_key =
      EncodingKey::from_rsa_pem(key.as_bytes()).expect("Failed to create encoding key");
    let decoding_key =
      DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).expect("Failed to create decoding key");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[iss.as_str()]);
    validation.validate_aud = false;

    Self {
      header,
      encoding_key,
      decoding_key,
      validation,
      iss,
      exp,
    }
  }
}

#[async_trait]
impl<'r> FromRequest<'r> for JwtClaims {
  type Error = ();

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    jwt_from_request::<JwtClaims>(req).await
  }
}

impl<'a> OpenApiFromRequest<'a> for JwtClaims {
  fn from_request_input(
    _: &mut OpenApiGenerator,
    _: String,
    _: bool,
  ) -> rocket_okapi::Result<RequestHeaderInput> {
    Ok(RequestHeaderInput::Security(
      "apiKey".into(),
      SecurityScheme {
        data: SecuritySchemeData::ApiKey {
          name: "token".into(),
          location: "cookie".into(),
        },
        description: Some("JWT token".to_string()),
        extensions: Map::new(),
      },
      Map::new(),
    ))
  }
}

#[derive(Default)]
pub struct TokenRes;

#[async_trait]
impl<'r, 'o: 'r> Responder<'r, 'o> for TokenRes {
  fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
    Ok(
      Response::build()
        .header(rocket::http::Header::new("Cache-Control", "no-store"))
        .header(rocket::http::Header::new("Pragma", "no-cache"))
        .finalize(),
    )
  }
}

impl OpenApiResponderInner for TokenRes {
  fn responses(_: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
    let mut responses = Map::new();
    responses.insert(
      "200".to_string(),
      RefOr::Object(rocket_okapi::okapi::openapi3::Response {
        description: "Successfully authenticated. The JWT token is set in the cookie.".to_string(),
        headers: Map::from([(
          "Set-Cookie".to_string(),
          RefOr::Object(rocket_okapi::okapi::openapi3::Header {
            allow_empty_value: false,
            required: true,
            deprecated: false,
            value: ParameterValue::Schema {
              style: None,
              explode: None,
              allow_reserved: false,
              schema: SchemaObject {
                instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                format: Some("string".to_string()),
                ..Default::default()
              },
              examples: None,
              example: Some(Value::String(format!(
                "{}={}; HttpOnly; Max-Age={}; SameSite=Lax; Secure",
                COOKIE_NAME, "{token}", "{max_age}"
              ))),
            },
            extensions: Map::new(),
            description: Some("JWT token".to_string()),
          }),
        )]),
        ..Default::default()
      }),
    );

    Ok(Responses {
      responses,
      ..Default::default()
    })
  }
}
