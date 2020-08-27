use crate::constants::{
  BASE_URL,
  RISERS,
  FALLERS
};

use clap::{App, load_yaml};

pub fn format_url(target: String) -> String {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from(yaml).get_matches();
  let ref_to_matches = &matches;
  let mut risers_or_fallers = "";
  if ref_to_matches.occurrences_of("risers") != 0 {
    risers_or_fallers = RISERS
  } else if ref_to_matches.occurrences_of("fallers") != 0 {
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
    assert!(cats == "http://www.hl.co.uk/shares/stock-market-summary/cats/fallers".to_string());
    assert!(dogs == "http://www.hl.co.uk/shares/stock-market-summary/dogs/fallers".to_string());
  }
}
