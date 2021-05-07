use crate::constants::{
  BASE_URL,
  RISERS,
  FALLERS,
  get_matches
};

pub fn format_url(target: String) -> String {
  let matches = get_matches();
  let mut risers_or_fallers = "";
  if matches.occurrences_of(RISERS) != 0 {
    risers_or_fallers = RISERS
  } else if matches.occurrences_of(FALLERS) != 0 {
    risers_or_fallers = FALLERS
  }
  return format!("{}{}/{}", BASE_URL, target, risers_or_fallers);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_format_url() {
    let cats = format_url("cats".to_string());
    let dogs = format_url("dogs".to_string());
    assert!(cats == "http://www.hl.co.uk/shares/stock-market-summary/cats/".to_string());
    assert!(dogs == "http://www.hl.co.uk/shares/stock-market-summary/dogs/".to_string());
  }
}
