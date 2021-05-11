#[macro_use]
extern crate prettytable;

mod constants;
mod parse_table;
mod build_table;
mod fetch_market;
mod get_url;
mod get_market;

use fetch_market::fetch_market;

fn main() {
    match fetch_market() {
        Ok(_) => {},
        Err(err) => eprintln!("error: {}", err),
    };
}
