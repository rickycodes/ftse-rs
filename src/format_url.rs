use crate::constants::{
  BASE_URL,
  FALLERS
};

pub fn format_url(target: String) -> String {
  return format!("{}{}/{}", BASE_URL, target, FALLERS);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_format_url() {
    let cats = format_url("cats".to_string());
    let dogs = format_url("dogs".to_string());
    assert!(cats == "http://www.hl.co.uk/shares/stock-market-summary/cats/fallers".to_string());
    assert!(dogs == "http://www.hl.co.uk/shares/stock-market-summary/dogs/fallers".to_string());
  }
}
