use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use crate::constants::{
  Stock,
  STOCK_TABLE,
  TBODY,
  TR,
  TD,
  LIMIT,
  AMOUNT,
  get_matches
};

fn parse_price(price: &str) -> f64 {
  price.parse::<f64>().unwrap_or(0.00).to_owned()
}

fn pad_price(price: f64) -> String {
  format!("{:.2}", price)
}

fn parse_stock_from_tds(tds: Vec<String>, padded_price: String) -> Stock {
  Stock {
    epic: tds[0].to_string(),
    name: tds[1].to_string(),
    price: padded_price,
    change_amount: tds[3].to_string(),
    change_percent: tds[4].to_string()
  }
}

fn collect_tds(tr: Node) -> Vec<String> {
  tr.find(Name(TD)).take(AMOUNT)
    .map(|td| td.text())
    .collect::<Vec<_>>()
}

fn collect_stock(tr: Node) -> Stock {
  let tds = collect_tds(tr);
  let price = parse_price(&tds[2]);
  let padded_price = pad_price(price);

  parse_stock_from_tds(
    tds,
    padded_price
  )
}

pub fn parse_table (document: Document) -> Vec<Stock> {
  let mut stocks: Vec<Stock> = Vec::new();

  for node in document.find(Class(STOCK_TABLE)) {
      for tbody in node.find(Name(TBODY)) {
          for tr in tbody.find(Name(TR)) {
            stocks.push(collect_stock(tr));
          }
      }
  }

  // TODO: account for when there are no risers or fallers?

  let matches = get_matches();
  let limit = matches.value_of(LIMIT);

  if let Some(num) = limit {
    let my_int = num.parse::<usize>().unwrap();
    stocks[..my_int].to_vec()
  } else {
    stocks
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_price() {
    assert!(parse_price("10.00") == 10.0);
    assert!(parse_price("123.500000") == 123.5);
  }

  #[test]
  fn test_pad_price() {
    assert!(pad_price(10.0) == "10.00".to_string());
    assert!(pad_price(123.4) == "123.40".to_string());
  }
}
