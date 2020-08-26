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
      return markets.aim;
  } else {
      return markets.hundred;
  }
}
