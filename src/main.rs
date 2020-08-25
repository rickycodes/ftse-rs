use select::document::Document;
use select::predicate::{Class, Name};
use serde::Serialize;

#[derive(Debug)]
struct Markets {
    aim: String,
    hundred: String,
}

#[derive(Serialize, Debug)]
struct Stock {
    epic: String,
    name: String,
    price: f32,
    change_amount: String,
    change_percent: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markets = Markets {
        aim: "ftse-aim-100".to_string(),
        hundred: "ftse-100".to_string()
    };

    let base_url = "http://www.hl.co.uk/shares/stock-market-summary/";
    let url = format!("{}{}", base_url, markets.aim);

    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let document = Document::from(resp.as_str());
    let mut stocks = Vec::new();

    for node in document.find(Class("stockTable")) {
        for tbody in node.find(Name("tbody")) {
            for tr in tbody.find(Name("tr")) {
                let tds = tr.find(Name("td")).take(5)
                    .map(|td| td.text())
                    .collect::<Vec<_>>();

                let price = tds[2].parse::<f32>().unwrap_or(0.00).to_owned();

                let stock = Stock {
                    epic: tds[0].to_string(),
                    name: tds[1].to_string(),
                    price,
                    change_amount: tds[3].to_string(),
                    change_percent: tds[4].to_string()
                };

                stocks.push(stock);
            }
        }
    }

    let json = serde_json::to_string(&stocks)?;
    println!("{}", json);

    Ok(())
}
