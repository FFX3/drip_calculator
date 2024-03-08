// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

#[macro_use]
mod macros;

mod get_data;

use chrono::Datelike;
use get_data::{ 
    dividend::DividendData, 
    morningstar::MorningstarSeriesData, 
    transform::{PositionSeries, PositionSeriesEntry},
};

mod error;
use error::Error;
use serde::Serialize;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
        let app_config_path = app.path_resolver().app_config_dir().expect("can't access appconfig");
        if !app_config_path.is_dir() {
            fs::create_dir(&app_config_path).expect("couldn't create config directory");
        }
        let path = app_config_path.join("ticker-confd");
        if !path.is_dir() {
            fs::create_dir(&path).expect("couldn't create config directory");
        }
        let path = path.join("default");
        println!("{:?}", path);
        let read_result = fs::read_to_string(&path);
        if read_result.is_err() {
            println!("creating default");
            fs::write(path, 
"{\
    \"qqq\": {\
        \"ticker\": \"qqq\",\
        \"mic\": \"xnas\",\
        \"color\": \"magenta\",\
        \"dripAtNav\": false\
    },\
    \"clm\": {\
        \"ticker\": \"clm\",\
        \"mic\": \"xase\",\
        \"color\": \"#33d17a\",\
        \"dripAtNav\": true\
    },\
    \"crf\": {\
        \"ticker\": \"crf\",\
        \"mic\": \"xase\",\
        \"color\": \"red\",\
        \"dripAtNav\": true\
    }\
}").expect("couldn't create ticker.conf");
        }
        std::result::Result::Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        fetch_data,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



#[derive(Serialize)]
struct ChartPoint {
    x: i64,
    y: f32,
}

type OnlyDividend<'a> = std::iter::Filter<
    std::slice::Iter<'a, PositionSeriesEntry>, 
    fn(&'_ &'_ PositionSeriesEntry) -> bool
>;

impl PositionSeries {
    /*  Position values */
    
    fn to_no_drip_chart_points(&self) -> Vec<ChartPoint> {
        self.data.iter().map(|entry| -> ChartPoint {
            ChartPoint {
                y: entry.position_value,
                x: entry.date.timestamp() * 1000, //seconds to ms
            }
        }).collect()
    }

    fn to_drip_chart_points(&self) -> Vec<ChartPoint> {
        self.data.iter().map(|entry| -> ChartPoint {
            ChartPoint {
                y: entry.drip.position_value,
                x: entry.date.timestamp() * 1000, //seconds to ms
            }
        }).collect()
    }

    fn to_drip_at_nav_chart_points(&self) -> Option<Vec<ChartPoint>> {
        if self.data[0].drip_at_nav.is_none() {
            return None
        }

        Some(self.data.iter().map(|entry| -> ChartPoint {
            ChartPoint {
                y: entry.drip_at_nav.unwrap().position_value,
                x: entry.date.timestamp() * 1000, //seconds to ms
            }
        }).collect())
    }

    /*  Total returns */

    fn to_no_drip_total_return_chart_points(&self) -> Vec<ChartPoint> {
        self.data.iter().map(|entry| -> ChartPoint {
            ChartPoint {
                y: entry.total_return,
                x: entry.date.timestamp() * 1000, //seconds to ms
            }
        }).collect()
    }

    fn to_drip_total_return_chart_points(&self) -> Vec<ChartPoint> {
        self.data.iter().map(|entry| -> ChartPoint {
            ChartPoint {
                y: entry.drip.total_return,
                x: entry.date.timestamp() * 1000, //seconds to ms
            }
        }).collect()
    }

    fn to_drip_at_nav_total_return_chart_points(&self) -> Option<Vec<ChartPoint>> {
        if self.data[0].drip_at_nav.is_none() {
            return None
        }

        Some(self.data.iter().map(|entry| -> ChartPoint {
            ChartPoint {
                y: entry.drip_at_nav.unwrap().total_return,
                x: entry.date.timestamp() * 1000, //seconds to ms
            }
        }).collect())
    }

    /*  CSV */
    fn to_csv(&self) -> String {
        let lines: Vec<String> = self.data.iter()
            .map(|entry| -> String {entry.to_csv_row()})
            .collect();
        PositionSeriesEntry::csv_header() + &lines.join("")
    }

    /* Dividends */
    pub fn only_with_dividend_iter(&self) -> OnlyDividend {
        fn has_dividend(&entry: &'_ &PositionSeriesEntry) ->bool {
            entry.dividend.is_some() 
        }
        self.data.iter().filter(has_dividend)
    }

    pub fn only_with_dividend(&self) -> Vec<PositionSeriesEntry> {
        self.only_with_dividend_iter().cloned().collect::<Vec<PositionSeriesEntry>>()
    }
}

#[derive(Serialize)]
struct OnlyWithDividendData {
    date: String,
    dividend: f32,
    position_value: f32,
    total_return: f32,
}

#[derive(Serialize)]
struct OnlyWithDividendEntry {
    no_drip: OnlyWithDividendData,
    drip: OnlyWithDividendData,
    drip_at_nav: Option<OnlyWithDividendData>
}

impl OnlyWithDividendEntry {
    fn from_position_series_entry(entry: &PositionSeriesEntry) -> Self {
        OnlyWithDividendEntry { 
            no_drip: OnlyWithDividendData {
                date: format!("{}-{}-{}", entry.date.year(), entry.date.month(), entry.date.day()), 
                dividend: entry.dividend.expect("entry must have dividend"), 
                position_value: entry.position_value, 
                total_return: entry.total_return 
            },
            drip: OnlyWithDividendData {
                date: format!("{}-{}-{}", entry.date.year(), entry.date.month(), entry.date.day()), 
                dividend: entry.drip.dividend.expect("entry must have dividend"), 
                position_value: entry.drip.position_value, 
                total_return: entry.drip.total_return 
            },
            drip_at_nav: match entry.drip_at_nav {
                Some(drip_at_nav) => {
                    Some(OnlyWithDividendData {
                        date: format!("{}-{}-{}", entry.date.year(), entry.date.month(), entry.date.day()), 
                        dividend: drip_at_nav.dividend.expect("entry must have dividend"), 
                        position_value: drip_at_nav.position_value, 
                        total_return: drip_at_nav.total_return 
                    })
                },
                None => None
            },
        }
    }
}

#[derive(Serialize)]
struct FetchDataResponse {
    no_drip: Vec<ChartPoint>,
    drip: Vec<ChartPoint>,
    drip_at_nav: Option<Vec<ChartPoint>>,
    no_drip_total_return: Vec<ChartPoint>,
    drip_total_return: Vec<ChartPoint>,
    drip_at_nav_total_return: Option<Vec<ChartPoint>>,
    csv: String,
    only_with_dividends: Vec<OnlyWithDividendEntry>,
}

#[tauri::command]
async fn fetch_data(ticker: String, mic: String, start_date: String, end_date: String, initial_position_value: f32, drip_at_nav: bool) -> Result<FetchDataResponse, Error> {
    println!("{}, {}", &ticker, &mic);

    println!("fetching data");
    let dividend_data: Vec<DividendData> = get_data::dividend::get_data(&ticker).await?;

    let morningstar_series: Vec<MorningstarSeriesData> = get_data::morningstar::get_series(&ticker, &mic, &start_date, &end_date).await?;

    println!("transforming data");

    let series = get_data::transform::build_asset_series(dividend_data, morningstar_series);

    let series = get_data::transform::build_position_series(&series, initial_position_value, drip_at_nav).unwrap();

    Ok(FetchDataResponse {
        no_drip: series.to_no_drip_chart_points(),
        drip: series.to_drip_chart_points(),
        drip_at_nav: series.to_drip_at_nav_chart_points(),
        no_drip_total_return: series.to_no_drip_total_return_chart_points(),
        drip_total_return: series.to_drip_total_return_chart_points(),
        drip_at_nav_total_return: series.to_drip_at_nav_total_return_chart_points(),
        csv: series.to_csv(),
        only_with_dividends: series.only_with_dividend_iter().map(|entry| -> OnlyWithDividendEntry { 
            OnlyWithDividendEntry::from_position_series_entry(entry)
        }).collect(),
    })
}

