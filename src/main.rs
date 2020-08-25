// use std::collections::HashMap;

// use error_chain::error_chain;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

// error_chain! {
//       foreign_links {
//           ReqError(reqwest::Error);
//           IoError(std::io::Error);
//       }
// }

#[derive(Debug)]
struct Markets {
    aim: String,
    hundred: String,
}

#[derive(Debug)]
struct Stock {
    epic: String,
    name: String,
    price: String,
    change_amount: String,
    change_percent: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markets = Markets {
        aim: "ftse-aim-100".to_string(),
        hundred: "ftse-100".to_string()
    };
    println!("{:#?}", markets);
    let base_url = "http://www.hl.co.uk/shares/stock-market-summary/";
    let url = format!("{}{}", base_url, markets.aim);
    // println!("{:#?}", url);
    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let document = Document::from(resp.as_str());

    for node in document.find(Class("stockTable")) {
        for tbody in node.find(Name("tbody")) {
            for tr in tbody.find(Name("tr")) {
                let tds = tr.find(Name("td")).take(5)
                    .map(|td| td.text())
                    .collect::<Vec<_>>();

                let stock = Stock {
                    epic: tds[0].to_string(),
                    name: tds[1].to_string(),
                    price: tds[2].to_string(),
                    change_amount: tds[3].to_string(),
                    change_percent: tds[4].to_string()
                };

                println!("{:#?}", stock);
            }
        }
    }

    Ok(())
}
