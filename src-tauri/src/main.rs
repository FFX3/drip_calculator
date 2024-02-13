// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::header::{REFERER, ORIGIN, CONNECTION, ACCEPT_ENCODING, ACCEPT_LANGUAGE, ACCEPT, AUTHORIZATION};
use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};
use chrono::prelude::*;
use chrono::serde::ts_seconds_option;


macro_rules! string_to_chrono {
    ($string:expr) => {{
        match dateparser::parse(&$string) {
            Ok(date) => {
                let without_hms = Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 0,0,0).unwrap();
                Some(without_hms)
            },
            _ => None
        }
    }};
}

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Http(#[from] reqwest::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Selector(#[from] scraper::error::SelectorErrorKind<'static>),
  #[error(transparent)]
  Regex(#[from] regex::Error),
  #[error(transparent)]
  Dateparser(#[from] anyhow::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        fetch_morningstar_data,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MorningstarSeriesDataWithStringDate {
    close: f32,
    open: f32,
    high: f32,
    low: f32,
    nav: f32,
    previous_close: f32,
    total_return: f32,
    volume: u32,
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MorningstarSeriesData {
    close: f32,
    open: f32,
    high: f32,
    low: f32,
    nav: f32,
    previous_close: f32,
    total_return: f32,
    volume: u32,
    #[serde(with = "ts_seconds_option")]
    date: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl MorningstarSeriesDataWithStringDate {
    fn to_series_data(self) -> MorningstarSeriesData {
        MorningstarSeriesData {
            close: self.close,
            open: self.open,
            high: self.high,
            low: self.low,
            nav: self.nav,
            previous_close: self.previous_close,
            total_return: self.total_return,
            volume: self.volume,
            date: string_to_chrono!(self.date)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DividendData {
    cash_amount: f32,
    #[serde(with = "ts_seconds_option")]
    ex_dividend_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(with = "ts_seconds_option")]
    record_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(with = "ts_seconds_option")]
    paydate: Option<chrono::DateTime<chrono::offset::Utc>>,
}

#[tauri::command]
async fn fetch_morningstar_data(ticker: String, mic: String, start_date: String, end_date: String) -> Result<(), Error> {
    println!("{}, {}", &ticker, &mic);

    let mut dividend_data: Vec<DividendData> = vec![];

    {
        let url = format!("https://stockanalysis.com/etf/{}/dividend/", ticker);
        let mut response = reqwest::get(url).await?;
        if 200 != response.status() {
            let url = format!("https://stockanalysis.com/stocks/{}/dividend/", ticker);
            response = reqwest::get(url).await?;
        }
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
    }

    let session_token;
    let security_id;

    let mut series: Vec<MorningstarSeriesData> = vec![];

    {
        let url = format!("https://www.morningstar.com/api/v2/cefs/{}/{}/chart", mic, ticker);
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body)?;
        //session_token = json.get("chart").unwrap().get("payload").unwrap().get("token").unwrap().to_string();
        //security_id = json.get("security").unwrap().get("securityID").unwrap().to_string();
        session_token = json["chart"]["payload"]["token"].as_str().unwrap().to_string();
        security_id = json["security"]["securityID"].as_str().unwrap().to_string();
    }


    {
        let url = format!("https://www.us-api.morningstar.com/QS-markets/chartservice/v2/timeseries?query={}:totalReturn,nav,open,high,low,close,volume,previousClose&frequency=d&startDate={}&endDate={}&trackMarketData=3.6.3&instid=DOTCOM", security_id, start_date, end_date);

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(REFERER, "https://www.morningstar.com/cefs/xase/clm/chart")
            .header(ORIGIN, "https://www.morningstar.com")
            .header(CONNECTION, "keep-alive")
            .header(ACCEPT_ENCODING, "gzip, deflate, br")
            .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
            .header(ACCEPT, "application/json, text/plain, */*")
            .header(AUTHORIZATION, format!("Bearer {}", &session_token))
            .send()
            .await?;
        let body = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body)?;

        let series_json_values: Vec<serde_json::Value> = serde_json::from_value(json[0]["series"].to_owned())?;
        
        for day in series_json_values.into_iter() {
            let result: Result<MorningstarSeriesDataWithStringDate, serde_json::Error> = serde_json::from_value(day.to_owned());
            match result {
                Ok(day) => {
                    series.push(day.to_series_data());
                },
                Err(_error) => {
                    // We can never parse today because the day isn't over yet
                    // leaving this here incase we have future parsing issues
                    //println!("couldn't parse: \n{:?} \n{:?}", _error, &day);
                }
                
            }
        }
    }

    println!("series struct: {:?}", series[0]);
    println!("dividend data: {:?}", dividend_data[0]);
    Ok(())
}

