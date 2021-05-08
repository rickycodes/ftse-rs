use select::document::Document;
use crate::select_market::select_market;
use crate::format_url::format_url;
use crate::parse_table::parse_table;
use crate::build_table::build_table;

use crate::constants::{
    MARKET,
    TABLE,
    get_matches
};

#[tokio::main]
pub async fn get_market() -> Result<(), Box<dyn std::error::Error>> {
    let matches = get_matches();
    let market = matches.value_of(MARKET).unwrap();

    let target = select_market(market);

    let url = format_url(target);

    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;

    // println!("{:?}", resp);

    let document = Document::from(resp.as_str());
    let stocks = parse_table(document);

    if matches.occurrences_of(TABLE) != 0 {
        let table = build_table(stocks);
        table.printstd();
    } else {
        let json = serde_json::to_string(&stocks)?;
        println!("{}", json);
    }

    Ok(())
}
