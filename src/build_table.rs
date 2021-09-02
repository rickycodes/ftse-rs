use prettytable::{Table, Row};
use ansi_term::Colour::{Red, Green, White};

use crate::constants::{
  Stock,
  EPIC,
  get_matches,
  NAME,
  CHANGE,
  CHANGE_PERCENT,
  COLOR,
  PRICE,
  PLUS,
  MINUS
};

pub fn paint_row(stock: &Stock) -> Row {
  let matches = get_matches();
  let contains_color = matches.occurrences_of(COLOR) != 0;

  let first = &stock.change_amount[0..1];
  let color = match first {
    PLUS => if contains_color { Green } else { White },
    MINUS => if contains_color { Red } else { White },
    _ => White
  };

  row![
    stock.epic,
    stock.name,
    color.paint(&stock.change_amount),
    color.paint(&stock.change_percent),
    color.paint(&stock.price)
  ]
}

pub fn build_table(stocks: Vec<Stock>) -> Table {
  let mut table = table!([EPIC, NAME, CHANGE, CHANGE_PERCENT, PRICE]);
  for stock in &stocks {
    table.add_row(paint_row(stock));
  }
  table
}
