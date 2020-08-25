mod constants;

use select::document::Document;
use select::predicate::{Class, Name};
use constants::{
    Stock,
    Markets,
    BASE_URL,
    AIM,
    HUNDRED,
    FALLERS
};

fn parse_table (document: Document) -> Vec<Stock> {
    let mut stocks = Vec::new();

    for node in document.find(Class("stockTable")) {
        for tbody in node.find(Name("tbody")) {
            for tr in tbody.find(Name("tr")) {
                let tds = tr.find(Name("td")).take(5)
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markets = Markets {
        aim: AIM.to_string(),
        hundred: HUNDRED.to_string()
    };

    let url = format!("{}{}/{}", BASE_URL, markets.aim, FALLERS);

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
