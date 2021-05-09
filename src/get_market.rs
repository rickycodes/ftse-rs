use select::document::Document;
use crate::select_market::select_market;
use crate::format_url::format_url;
use crate::parse_table::parse_table;
use crate::build_table::build_table;

use crate::constants::{
    MARKET,
    TABLE,
    get_matches,
    RISERS,
    FALLERS
};

#[tokio::main]
pub async fn get_market() -> Result<(), Box<dyn std::error::Error>> {
    let matches = get_matches();
    let market = matches.value_of(MARKET).unwrap();

    let target = select_market(market);

    let t_clone = target.clone();
    let url = format_url(target);

    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let document = Document::from(resp.as_str());
    let stocks = parse_table(document);

    if matches.occurrences_of(TABLE) != 0 {
        let table = build_table(stocks);
        table.printstd();
        print!("{}: ", t_clone.replace("-", " ").to_uppercase());
        if matches.occurrences_of(RISERS) != 0 {
            println!("TOP 20 {}", RISERS.to_uppercase());
        } else if matches.occurrences_of(FALLERS) != 0 {
            println!("TOP 20 {}", FALLERS.to_uppercase());
        }

        println!("DATA FETCHED FROM: ");
        println!("{}", url);
    } else {
        let json = serde_json::to_string(&stocks)?;
        println!("{}", json);
    }

    Ok(())
}
