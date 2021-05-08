use prettytable::{Table, Row};
use ansi_term::Colour::{Red, Green, White};

use crate::constants::{
  Stock,
  EPIC,
  NAME,
  CHANGE,
  CHANGE_PERCENT,
  PRICE,
  PLUS,
  MINUS
};

pub fn paint_row(stock: &Stock) -> Row {
  let first = &stock.change_amount[0..1];
  let color = match first {
    PLUS => Green,
    MINUS => Red,
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
