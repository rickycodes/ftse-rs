use serde::Serialize;
use clap::{App, load_yaml, ArgMatches};

pub fn get_matches() -> ArgMatches {
  let yaml = load_yaml!("cli.yml");
  App::from(yaml).get_matches()
}

pub struct Markets {
  pub aim: String,
  pub hundred: String,
}

#[derive(Serialize, Clone)]
pub struct Stock {
  pub epic: String,
  pub name: String,
  pub price: String,
  pub change_amount: String,
  pub change_percent: String
}

pub const BASE_URL: &str = "http://www.hl.co.uk/shares/stock-market-summary/";

/* url bits (risers and fallers are also flags) */
pub const RISERS: &str = "risers";
pub const FALLERS: &str = "fallers";
pub const AIM: &str = "ftse-aim-100";
pub const HUNDRED: &str = "ftse-100";

/* selectors */
pub const STOCK_TABLE: &str = "stockTable";
pub const TBODY: &str = "tbody";

/* elements */
pub const TR: &str = "tr";
pub const TD: &str = "td";

/* cli flags + options */
pub const TABLE: &str = "table";
pub const MARKET: &str = "market";
pub const LIMIT: &str = "limit";
pub const VERBOSE: &str = "verbose";
pub const COLOR: &str = "color";

/* table headers */
pub const EPIC: &str = "EPIC";
pub const NAME: &str = "NAME";
pub const CHANGE: &str = "CHANGE";
pub const CHANGE_PERCENT: &str = "CHANGE %";
pub const PRICE: &str = "PRICE";

pub const PLUS: &str = "+";
pub const MINUS: &str = "-";

/* td amount */
pub const AMOUNT: usize = 5;
