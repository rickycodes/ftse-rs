use select::document::Document;
use select::predicate::{Class, Name};
use crate::constants::{
  Stock,
  STOCK_TABLE,
  TBODY,
  TR,
  TD
};

pub fn parse_table (document: Document) -> Vec<Stock> {
  let mut stocks = Vec::new();

  for node in document.find(Class(STOCK_TABLE)) {
      for tbody in node.find(Name(TBODY)) {
          for tr in tbody.find(Name(TR)) {
              let tds = tr.find(Name(TD)).take(5)
                  .map(|td| td.text())
                  .collect::<Vec<_>>();

              let price = tds[2].parse::<f64>().unwrap_or(0.00).to_owned();

              let padded_price = format!("{:.2}", price);

              let stock = Stock {
                  epic: tds[0].to_string(),
                  name: tds[1].to_string(),
                  price: padded_price,
                  change_amount: tds[3].to_string(),
                  change_percent: tds[4].to_string()
              };

              stocks.push(stock);
          }
      }
  }

  stocks
}
