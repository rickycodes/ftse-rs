use crate::constants::{
  BASE_URL,
  RISERS,
  FALLERS,
  get_matches
};

pub fn get_url(path: String) -> String {
  let matches = get_matches();
  let mut risers_or_fallers = "";
  if matches.occurrences_of(RISERS) != 0 {
    risers_or_fallers = RISERS
  } else if matches.occurrences_of(FALLERS) != 0 {
    risers_or_fallers = FALLERS
  }
  return format!("{}{}/{}", BASE_URL, path, risers_or_fallers);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_url() {
    let cats = get_url("cats".to_string());
    let dogs = get_url("dogs".to_string());
    assert!(cats == "http://www.hl.co.uk/shares/stock-market-summary/cats/".to_string());
    assert!(dogs == "http://www.hl.co.uk/shares/stock-market-summary/dogs/".to_string());
  }
}
