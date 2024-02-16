use crate::{ DividendData, MorningstarSeriesData };

#[derive(Debug, Clone, Copy)]
pub struct AssetSeriesEntry {
    date: chrono::DateTime<chrono::offset::Utc>,
    close: f32,
    nav: f32,
    dividend: Option<f32>,
}

#[derive(Debug)]
pub struct AssetSeries {
    pub data: Vec<AssetSeriesEntry>
}

type OnlyDividend<'a> = std::iter::Filter<
    std::slice::Iter<'a, AssetSeriesEntry>, 
    fn(&'_ &'_ AssetSeriesEntry) -> bool
>;

impl AssetSeries {
    pub fn only_with_dividend_iter(&self) -> OnlyDividend {
        fn has_dividend(&entry: &'_ &AssetSeriesEntry) ->bool {
            entry.dividend.is_some() 
        }
        self.data.iter().filter(has_dividend)
    }

    pub fn only_with_dividend(&self) -> Vec<AssetSeriesEntry> {
        self.only_with_dividend_iter().cloned().collect::<Vec<AssetSeriesEntry>>()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DripData {
    drip: Option<f32>,
    share_count: f32,
    pub position_value: f32,
    pub total_return: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct PositionSeriesEntry {
    pub date: chrono::DateTime<chrono::offset::Utc>,
    close: f32,
    nav: f32,
    dividend: Option<f32>,
    pub position_value: f32,
    total_dividends: f32,
    pub total_return: f32,
    pub drip: DripData,
    pub drip_at_nav: Option<DripData>
}

impl PositionSeriesEntry {
    pub fn csv_header() -> String {
        "date,close,nav,dividend,position value,total dividends,total return,(drip) drip,(drip) share count, (drip) total return,(drip) position value,(DAN) drip,(DAN) share count,(DAN) total return,(DAN) position value\n".to_string()
    }
    pub fn to_csv_row(&self) -> String {
        format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n", 
            self.date, 
            self.close, 
            self.nav, 
            match self.dividend { None => "".to_string(), Some(amount) => amount.to_string() },
            self.position_value,
            self.total_dividends,
            self.total_return,
            match self.drip.drip { None => "".to_string(), Some(drip) => drip.to_string() },
            self.drip.share_count,
            self.drip.total_return,
            self.drip.position_value,
            match self.drip_at_nav { None => "".to_string(), Some(data) => 
                match data.drip { None => "".to_string(), Some(drip) => drip.to_string() }
            },
            match self.drip_at_nav { None => "".to_string(), Some(data) => data.share_count.to_string() },
            match self.drip_at_nav { None => "".to_string(), Some(data) => data.total_return.to_string() },
            match self.drip_at_nav { None => "".to_string(), Some(data) => data.position_value.to_string() },
        )
    }
}

#[derive(Debug, Clone)]
pub struct PositionSeries {
    pub initial_share_count: f32,
    pub initial_position_value: f32,
    pub data: Vec<PositionSeriesEntry>,
}

pub fn build_asset_series(dividend_data: Vec<DividendData>, morningstar_series: Vec<MorningstarSeriesData>) -> AssetSeries {
    let mut dividend_iter = dividend_data.iter();
    let mut next_dividend_entry = dividend_iter.next();

    while next_dividend_entry.is_some() 
        && morningstar_series[0].date.unwrap() 
        > next_dividend_entry.unwrap().ex_dividend_date.unwrap() {
        next_dividend_entry = dividend_iter.next();
    }

    let mut series: Vec<AssetSeriesEntry> = vec![];

    for morningstar_entry in morningstar_series.iter() {
        let date = morningstar_entry.date.unwrap();
        let mut dividend: Option<f32> = None;
        if let Some(next_dividend) = next_dividend_entry {
            let ex_date = next_dividend.ex_dividend_date.unwrap();
            if date >= ex_date {
                dividend = Some(next_dividend.cash_amount);
                next_dividend_entry = dividend_iter.next();
            }
        }
        series.push(AssetSeriesEntry { 
            close: morningstar_entry.close, 
            nav: morningstar_entry.nav, 
            date, 
            dividend,
        })
    }
    AssetSeries{
        data: series
    }
}

pub fn build_position_series(asset_series: &AssetSeries, initial_position_value: f32, drip_at_nav: bool) -> Result<PositionSeries, &str> {
    let mut asset_series_iter = asset_series.data.iter();
    let mut position_series: Vec<PositionSeriesEntry> =  Vec::new();
    let initial_share_count: f32;
    
    let first_entry = asset_series_iter.next();

    if let Some(entry) = first_entry {
        let drip_data: DripData;
        let drip_at_nav_data: Option<DripData>;
        initial_share_count = initial_position_value / entry.close;

        (drip_data, drip_at_nav_data) = match entry.dividend {
            Some(dividend) => {
                let cash_amount = dividend * initial_share_count;
                let drip = {
                    let drip = cash_amount / entry.close;
                    let share_count = initial_share_count + drip;
                    let position_value = share_count * entry.close;

                    DripData {
                        total_return: (position_value / initial_position_value) - 1.,
                        drip: Some(drip),
                        position_value,
                        share_count,
                    }
                };
                let drip_at_nav = if drip_at_nav {
                    let drip = cash_amount / entry.nav;
                    let share_count = initial_share_count + drip;
                    let position_value = share_count * entry.close;

                    Some(DripData {
                        total_return: (position_value / initial_position_value) - 1.,
                        position_value: share_count * entry.close,
                        drip: Some(drip),
                        share_count,
                    })
                } else {
                    None
                };
                (drip, drip_at_nav)
            },
            None => {
                let share_count = initial_share_count;
                let drip = DripData {
                    total_return: 0.,
                    position_value: share_count * entry.close,
                    drip: None,
                    share_count,
                };
                
                let drip_at_nav = if drip_at_nav {
                    Some(drip)
                } else {
                    None
                };
                (drip, drip_at_nav)
            }
        };
        
        let total_dividend = match entry.dividend {
            Some(dividend) => dividend * initial_share_count,
            None => 0.,
        };

        let (dividend, total_dividends) = match entry.dividend {
            Some(dividend) => {
                let dividend = dividend * initial_share_count;
                (Some(dividend), dividend)
            },
            None => (None, 0.),
        };

        position_series.push(PositionSeriesEntry { 
            date: entry.date, 
            close: entry.close, 
            nav: entry.nav, 
            dividend,
            position_value: initial_position_value, 
            total_dividends,
            total_return: ((initial_position_value + total_dividend) / initial_position_value) - 1.,
            drip: drip_data,
            drip_at_nav: drip_at_nav_data, 
        });

    } else {
         return Result::Err("asset series is empty")
    }

    let mut previous_entry = position_series[0];

    for entry in asset_series_iter {
        let drip_data: DripData;
        let drip_at_nav_data: Option<DripData>;
        (drip_data, drip_at_nav_data) = match entry.dividend {
            Some(dividend) => {
                let drip = {
                    let cash_amount = dividend * previous_entry.drip.share_count;
                    let drip = cash_amount / entry.close;
                    let share_count = previous_entry.drip.share_count + drip;
                    let position_value = share_count * entry.close;

                    DripData {
                        total_return: (position_value / initial_position_value) - 1.,
                        position_value,
                        drip: Some(drip),
                        share_count,
                    }
                };

                let drip_at_nav = if drip_at_nav {
                    let cash_amount = dividend * previous_entry.drip_at_nav.unwrap().share_count;
                    let drip = cash_amount / entry.nav;
                    let share_count = previous_entry.drip_at_nav.unwrap().share_count + drip;
                    let position_value = share_count * entry.close;

                    Some(DripData {
                        total_return: (position_value / initial_position_value) - 1.,
                        drip: Some(drip),
                        position_value,
                        share_count,
                    })
                } else {
                    None
                };

                (drip, drip_at_nav)
            },
            None => {
                let drip = {
                    let share_count = previous_entry.drip.share_count;
                    let position_value = share_count * entry.close;

                    DripData {
                        total_return: (position_value / initial_position_value) - 1.,
                        drip: None,
                        share_count,
                        position_value,
                    }
                };
                
                let drip_at_nav = if drip_at_nav {
                    let share_count = previous_entry.drip_at_nav.unwrap().share_count;
                    let position_value = share_count * entry.close;

                    Some(DripData {
                        total_return: (position_value / initial_position_value) - 1.,
                        position_value: share_count * entry.close,
                        drip: None,
                        share_count,
                    })
                } else {
                    None
                };

                (drip, drip_at_nav)
            }
        };


        let (dividend, total_dividends) = match entry.dividend {
            Some(dividend) => {
                let dividend = dividend * initial_share_count;
                (Some(dividend), dividend + previous_entry.total_dividends)
            },
            None => (None, previous_entry.total_dividends),
        };

        let position_value = initial_share_count * entry.close;

        previous_entry = PositionSeriesEntry { 
            date: entry.date, 
            close: entry.close, 
            nav: entry.nav, 
            dividend, 
            total_dividends,
            position_value,
            total_return: ((position_value + total_dividends) / initial_position_value) - 1.,
            drip: drip_data,
            drip_at_nav: drip_at_nav_data, 
        };
        position_series.push(previous_entry);
    }

    Ok(PositionSeries {
        initial_share_count, 
        initial_position_value,
        data: position_series
    })
}
