mod constants;
mod parse_table;
mod select_market;
mod format_url;

use select::document::Document;
use select_market::select_market;
use format_url::format_url;
use parse_table::parse_table;

#[tokio::main]
async fn get_market() -> Result<(), Box<dyn std::error::Error>> {
    let target = select_market("100");

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

fn main() {
    match get_market() {
        Ok(_) => {},
        Err(err) => eprintln!("error: {}", err),
    };
}
