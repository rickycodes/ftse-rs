use crate::constants::{
  Markets,
  AIM,
  HUNDRED
};

pub fn select_market(market: &str) -> String {
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
  fn test_select_market() {
    assert!(select_market("100") == HUNDRED.to_string());
    assert!(select_market("aim") == AIM.to_string());
  }
}
