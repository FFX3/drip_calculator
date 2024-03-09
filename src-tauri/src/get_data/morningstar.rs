use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, AUTHORIZATION, CONNECTION, COOKIE, ORIGIN, REFERER};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use chrono::serde::ts_seconds_option;

use crate::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MorningstarSeriesData {
    pub close: f32,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub nav: f32,
    pub previous_close: f32,
    pub total_return: f32,
    pub volume: u32,
    #[serde(with = "ts_seconds_option")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
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

async fn get_mic(ticker: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    //let url = "https://www.morningstar.com/";
    //let response = client
    //    .get(url)
    //    .send()
    //    .await?;
    //let session_cookie = response.headers()["set-cookie"].to_owned();
    let url = format!("https://www.morningstar.com/api/v1/search/entities?q={}&limit=1&autocomplete=false", ticker);

    let response = client
        .get(url)
        //.header(COOKIE, session_cookie)
        .header("x-api-key", "Nrc2ElgzRkaTE43D5vA7mrWj2WpSBR35fvmaqfte")
        .header(REFERER, "https://www.morningstar.com/")
        .header(ORIGIN, "https://www.morningstar.com")
        .header(CONNECTION, "keep-alive")
        .header(ACCEPT_ENCODING, "gzip, deflate, br")
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .header(ACCEPT, "application/json, text/plain, */*")
        .send()
        .await?;
    println!("{}", response.status());
    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    println!("{}", json);

    let mic: String = serde_json::from_value(json["results"][0]["exchange"].to_owned())?;
    return Ok(mic);
}

async fn get_morningstar_metadata(ticker: &str, mic: &str) -> Result<(String, String), Error> {
    let url = format!("https://www.morningstar.com/api/v2/cefs/{}/{}/chart", mic, ticker);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let session_token = json["chart"]["payload"]["token"].as_str().unwrap().to_string();
    let security_id = json["security"]["securityID"].as_str().unwrap().to_string();

    Ok((session_token, security_id))
}


pub async fn get_series(ticker: &str, mic: &str, start_date: &str, end_date: &str) -> Result<Vec<MorningstarSeriesData>, Error> {

    let mic = get_mic(ticker).await?;
    let (session_token, security_id) = get_morningstar_metadata(ticker, &mic).await?;

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
    
    let mut series: Vec<MorningstarSeriesData> = vec![];
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
    Ok(series)
}
