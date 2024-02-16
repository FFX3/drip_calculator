// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
mod macros;

mod get_data;

use get_data::{ 
    dividend::DividendData, 
    morningstar::MorningstarSeriesData, 
    transform::PositionSeries,
};

mod error;
use error::Error;
use serde::Serialize;

fn main() {
  tauri::Builder::default()
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
impl PositionSeries {
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
}

#[derive(Serialize)]
struct FetchDataResponse {
    no_drip: Vec<ChartPoint>,
    drip: Vec<ChartPoint>,
    drip_at_nav: Option<Vec<ChartPoint>>,
}

#[tauri::command]
async fn fetch_data(ticker: String, mic: String, start_date: String, end_date: String) -> Result<FetchDataResponse, Error> {
    println!("{}, {}", &ticker, &mic);

    println!("fetching data");
    let dividend_data: Vec<DividendData> = get_data::dividend::get_data(&ticker).await?;

    let morningstar_series: Vec<MorningstarSeriesData> = get_data::morningstar::get_series(&ticker, &mic, &start_date, &end_date).await?;

    println!("transforming data");

    let series = get_data::transform::build_asset_series(dividend_data, morningstar_series);

    let series = get_data::transform::build_position_series(&series, 1000., true).unwrap();

    Ok(FetchDataResponse {
        no_drip: series.to_no_drip_chart_points(),
        drip: series.to_drip_chart_points(),
        drip_at_nav: series.to_drip_at_nav_chart_points(),
    })
}

