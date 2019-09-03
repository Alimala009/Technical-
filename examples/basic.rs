extern crate rusty_technical;
use crate::rusty_technical::get_technical_indicators;
use crate::rusty_technical::TechnicalAdded;

use std::time::Instant;

fn main() {
    let mydata = vec![1.0, 5.3, 0.5, 5.3, 1.0, 5.3, 1.0, 9.0];
    let _start = Instant::now();
    let results = get_technical_indicators(mydata);
    let duration = _start.elapsed();
    println!("{:#?}", results);
    println!("{:#?}", duration);
}
