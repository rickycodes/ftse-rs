#[macro_use]
extern crate prettytable;

mod constants;
mod parse_table;
mod select_market;
mod format_url;
mod get_market;

use get_market::get_market;

fn main() {
    match get_market() {
        Ok(_) => {},
        Err(err) => eprintln!("error: {}", err),
    };
}
