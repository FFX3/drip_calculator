use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};
use chrono::prelude::*;
use chrono::serde::ts_seconds_option;

use crate::Error;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DividendData {
    pub cash_amount: f32,
    #[serde(with = "ts_seconds_option")]
    pub ex_dividend_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub record_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub paydate: Option<chrono::DateTime<chrono::offset::Utc>>,
}

pub async fn get_data(ticker: &str) -> Result<Vec<DividendData>, Error>  {
    let url = format!("https://stockanalysis.com/etf/{}/dividend/", ticker);
    let mut response = reqwest::get(url).await?;
    if 200 != response.status() {
        let url = format!("https://stockanalysis.com/stocks/{}/dividend/", ticker);
        response = reqwest::get(url).await?;
    }

    let mut dividend_data: Vec<DividendData> = vec![];
    let html = response.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("tr")?;
    for row in document.select(&selector).skip(1) {
        let selector = Selector::parse("td")?;
        let mut iterator = row.select(&selector);
        let ex_dividend_date = iterator.next().unwrap().inner_html();
        let mut cash_amount = iterator.next().unwrap().inner_html();
        let regex = regex::Regex::new(r"<!-- HTML_TAG_START -->\$(\d+\.\d+)<!-- HTML_TAG_END -->")?;
        if let Some(captures) = regex.captures_iter(&cash_amount).next() {
            // Extract the number
            cash_amount = captures.get(1).unwrap().as_str().to_string();
        } else {
            cash_amount = "0".to_string();
            println!("could not find cash_amount using regex look in string: {}", cash_amount);
        }
        let record_date = iterator.next().unwrap().inner_html();
        let paydate = iterator.next().unwrap().inner_html();

        dividend_data.push(DividendData {
            ex_dividend_date: string_to_chrono!(ex_dividend_date),
            record_date: string_to_chrono!(record_date),
            paydate: string_to_chrono!(paydate),
            cash_amount: cash_amount.parse().unwrap()
        });
    }
    dividend_data.reverse();
    Ok(dividend_data)
}
