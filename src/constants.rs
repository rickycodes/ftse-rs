use serde::Serialize;

pub struct Markets {
  pub aim: String,
  pub hundred: String,
}

#[derive(Serialize)]
pub struct Stock {
  pub epic: String,
  pub name: String,
  pub price: String,
  pub change_amount: String,
  pub change_percent: String
}

pub const BASE_URL: &str = "http://www.hl.co.uk/shares/stock-market-summary/";

pub const RISERS: &str = "risers";
pub const FALLERS: &str = "fallers";
pub const AIM: &str = "ftse-aim-100";
pub const HUNDRED: &str = "ftse-100";
pub const STOCK_TABLE: &str = "stockTable";
pub const TBODY: &str = "tbody";
pub const TR: &str = "tr";
pub const TD: &str = "td";
