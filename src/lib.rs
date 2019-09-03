extern crate ta_lib_wrapper;

use ta_lib_wrapper::{TA_Integer, TA_MAType, TA_Real, TA_RetCode, TA_MA, TA_ROCP, TA_RSI};

/// Compute SMA(period) on `close_prices`
/// This function returns a tuple containing the list of sma values and the index of the first
/// close to have an associated sma value
fn rocp(period: u32, close_prices: &Vec<TA_Real>) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_ROCP(
            0,                             // index of the first close to use
            close_prices.len() as i32 - 1, // index of the last close to use
            close_prices.as_ptr(),         // pointer to the first element of the vector
            period as i32,                 // period of the sma
            &mut out_begin,                // set to index of the first close to have an sma value
            &mut out_size,                 // set to number of sma values computed
            out.as_mut_ptr(),              // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_MA call
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    (out, out_begin)
}

/// Compute SMA(period) on `close_prices`
/// This function returns a tuple containing the list of sma values and the index of the first
/// close to have an associated sma value
fn sma(period: u32, close_prices: &Vec<TA_Real>) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_MA(
            0,                             // index of the first close to use
            close_prices.len() as i32 - 1, // index of the last close to use
            close_prices.as_ptr(),         // pointer to the first element of the vector
            period as i32,                 // period of the sma
            TA_MAType::TA_MAType_SMA,      // type of the MA, here forced to sma
            &mut out_begin,                // set to index of the first close to have an sma value
            &mut out_size,                 // set to number of sma values computed
            out.as_mut_ptr(),              // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_MA call
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    (out, out_begin)
}

/// Compute RSI(period) on `close_prices`
/// This function returns a tuple containing the list of rsi values and the index of the first
/// close to have an associated rsi value
fn rsi(period: u32, close_prices: &Vec<TA_Real>) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_RSI(
            0,                             // index of the first close to use
            close_prices.len() as i32 - 1, // index of the last close to use
            close_prices.as_ptr(),         // pointer to the first element of the vector
            period as i32,                 // period of the rsi
            &mut out_begin,                // set to index of the first close to have an rsi value
            &mut out_size,                 // set to number of sma values computed
            out.as_mut_ptr(),              // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_RSI call
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    (out, out_begin)
}

fn zeros(size: i32) -> Vec<f64> {
    vec![0.0; size as usize]
}

fn pad_indicator(mut indicator: Vec<f64>, off: i32) -> Vec<f64> {
    let mut vec = zeros(off);
    vec.append(&mut indicator);
    indicator = vec.to_owned();
    return indicator;
}

#[derive(Debug)]
pub struct TechnicalAdded {
    pub data: Vec<Vec<f64>>,
    pub names: Vec<&'static str>,
}

pub fn get_technical_indicators(mydata: Vec<f64>) -> TechnicalAdded {
    let (mut ta_rsi, ta_rsi_offset_len) = rsi(2, &mydata);
    let (mut ta_sma, ta_sma_offset_len) = sma(2, &mydata);
    let (mut ta_rocp, ta_rocp_offset_len) = rocp(1, &mydata);

    ta_rsi = pad_indicator(ta_rsi, ta_rsi_offset_len);
    ta_sma = pad_indicator(ta_sma, ta_sma_offset_len);
    ta_rocp = pad_indicator(ta_rocp, ta_rocp_offset_len);

    let results = TechnicalAdded {
        data: vec![mydata, ta_rsi, ta_sma, ta_rocp],
        names: vec!["OG", "RSI", "SMA", "ROCP"],
    };
    results
}
