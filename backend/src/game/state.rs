pub struct RoomState {
  pub default_starting_cash: i32,
  pub default_small_blind: i32,
}

impl Default for RoomState {
  fn default() -> Self {
    let default_starting_cash = std::env::var("DEFAULT_STARTING_CASH")
      .ok()
      .and_then(|v| v.parse::<i32>().ok())
      .unwrap_or(1000);

    let default_small_blind = std::env::var("DEFAULT_SMALL_BLIND")
      .ok()
      .and_then(|v| v.parse::<i32>().ok())
      .unwrap_or(10);

    assert!(
      default_starting_cash > 0,
      "Default starting cash must be positive"
    );
    assert!(
      default_small_blind > 0,
      "Default small blind must be positive"
    );
    assert!(
      default_small_blind * 2 <= default_starting_cash,
      " Default small blind must be less than or equal to half of the default starting cash"
    );

    Self {
      default_starting_cash,
      default_small_blind,
    }
  }
}
