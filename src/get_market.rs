use crate::constants::{
  Markets,
  AIM,
  HUNDRED
};

pub fn get_market(market: &str) -> String {
  let markets = Markets {
      aim: AIM.to_string(),
      hundred: HUNDRED.to_string()
  };

  if market == "aim" {
    markets.aim
  } else {
    markets.hundred
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_market() {
    assert!(get_market("100") == HUNDRED.to_string());
    assert!(get_market("aim") == AIM.to_string());
  }
}
