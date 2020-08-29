use prettytable::{Table};

use crate::constants::{
  Stock,
  EPIC,
  NAME,
  CHANGE,
  CHANGE_PERCENT,
  PRICE
};

pub fn build_table(stocks: Vec<Stock>) -> Table {
  let mut table = table!([EPIC, NAME, CHANGE, CHANGE_PERCENT, PRICE]);
  for stock in &stocks {
      table.add_row(row![
          stock.epic,
          stock.name,
          stock.change_amount,
          stock.change_percent,
          stock.price
      ]);
  }
  table
}
