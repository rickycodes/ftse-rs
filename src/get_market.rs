use select::document::Document;
use crate::select_market::select_market;
use crate::format_url::format_url;
use crate::parse_table::parse_table;

use crate::constants::{
    get_matches
};

#[tokio::main]
pub async fn get_market() -> Result<(), Box<dyn std::error::Error>> {
    let matches = get_matches();
    let market = matches.value_of("market").unwrap();

    let target = select_market(market);

    let url = format_url(target);

    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let document = Document::from(resp.as_str());
    let stocks = parse_table(document);

    let json = serde_json::to_string(&stocks)?;
    println!("{}", json);

    Ok(())
}
