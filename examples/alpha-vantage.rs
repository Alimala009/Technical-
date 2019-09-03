extern crate alphavantage;

use std::time::Instant;

use alphavantage::time_series::IntradayInterval::ThirtyMinutes;

extern crate rusty_technical;
use crate::rusty_technical::get_technical_indicators;
use crate::rusty_technical::TechnicalAdded;

fn get_equity_data() -> Vec<f64> {
    let client = alphavantage::Client::new("F3GVWG91RSWV8SAN");
    // let time_series = client.get_time_series_daily("TVIX").unwrap();

    let time_series = client
        .get_time_series_intraday("TVIX", ThirtyMinutes)
        .unwrap();

    let mut prices: Vec<f64> = vec![];

    let mut c = 0;
    for entry in time_series.entries {
        c = c + 1;
        prices.push(entry.close)
    }
    println!("{:#?}", c);
    prices
}

fn main() {
    let mydata = get_equity_data();
    let _start = Instant::now();
    let results = get_technical_indicators(mydata);
    let duration = _start.elapsed();
    println!("{:#?}", results);
    println!("{:#?}", duration);
}
