extern crate ta_lib_wrapper;

use ta_lib_wrapper::{
    TA_Integer, TA_MAType, TA_Real, TA_RetCode, TA_MA, TA_MFI, TA_ROCP, TA_RSI, TA_SAR, TA_ATR
};

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
            0,                             // startIdx: ::std::os::raw::c_int,
            close_prices.len() as i32 - 1, // endIdx: ::std::os::raw::c_int,
            close_prices.as_ptr(),         // inReal: *const f64,
            period as i32,                 // optInTimePeriod: ::std::os::raw::c_int,
            &mut out_begin,                // outBegIdx: *mut ::std::os::raw::c_int,
            &mut out_size,                 // outNBElement: *mut ::std::os::raw::c_int,
            out.as_mut_ptr(),              // outReal: *mut f64,
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

fn sar(high: &Vec<TA_Real>, low: &Vec<TA_Real>) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(high.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_SAR(
            0,                     // startIdx: ::std::os::raw::c_int,
            high.len() as i32 - 1, // endIdx: ::std::os::raw::c_int,
            high.as_ptr(),         // inHigh: *const f64,
            low.as_ptr(),          // inLow: *const f64,
            4.0,                   // optInAcceleration: f64,
            9.0,                   // optInMaximum: f64,
            &mut out_begin,        // outBegIdx: *mut ::std::os::raw::c_int,
            &mut out_size,         // outNBElement: *mut ::std::os::raw::c_int,
            out.as_mut_ptr(),      // outReal: *mut f64,
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

fn mfi(
    period: u32,
    close_prices: &Vec<TA_Real>,
    high: &Vec<TA_Real>,
    low: &Vec<TA_Real>,
    vol: &Vec<TA_Real>,
) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_MFI(
            0,                             //  startIdx: ::std::os::raw::c_int,
            close_prices.len() as i32 - 1, //  endIdx: ::std::os::raw::c_int,
            high.as_ptr(),                 //  inHigh: *const f64,
            low.as_ptr(),                  //  inLow: *const f64,
            close_prices.as_ptr(),         //  inClose: *const f64,
            vol.as_ptr(),                  //  inVolume: *const f64,
            period as i32,                 //  optInTimePeriod: ::std::os::raw::c_int,
            &mut out_begin,                //  outBegIdx: *mut ::std::os::raw::c_int,
            &mut out_size,                 //  outNBElement: *mut ::std::os::raw::c_int,
            out.as_mut_ptr(),              //  outReal: *mut f64,
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

fn atr(
    period: u32,
    close_prices: &Vec<TA_Real>,
    high: &Vec<TA_Real>,
    low: &Vec<TA_Real>,
) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_ATR(
            0,                             // startIdx: ::std::os::raw::c_int,
            close_prices.len() as i32 - 1, // endIdx: ::std::os::raw::c_int,
            high.as_ptr(),                 // inHigh: *const f64,
            low.as_ptr(),                  // inLow: *const f64,
            close_prices.as_ptr(),         // inClose: *const f64,
            period as i32,                 // optInTimePeriod: ::std::os::raw::c_int,
            &mut out_begin,                // outBegIdx: *mut ::std::os::raw::c_int,
            &mut out_size,                 // outNBElement: *mut ::std::os::raw::c_int,
            out.as_mut_ptr(),              // outReal: *mut f64,
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

pub fn get_technical_indicators(
    _open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    vol: Vec<f64>,
    ) -> TechnicalAdded {
    let (mut ta_rsi, ta_rsi_offset_len) = rsi(12, &close);
    let (mut ta_sma, ta_sma_offset_len) = sma(2, &close);
    let (mut ta_rocp, ta_rocp_offset_len) = rocp(1, &close);
    let (mut ta_sar, ta_sar_offset_len) = sar(&high, &low);
    let (mut ta_mfi, ta_mfi_offset_len) = mfi(10, &close, &high, &low, &vol);
    let (mut ta_atr, ta_atr_offset_len) = atr(10, &close, &high, &low);

    ta_rsi = pad_indicator(ta_rsi, ta_rsi_offset_len);
    ta_sma = pad_indicator(ta_sma, ta_sma_offset_len);
    ta_rocp = pad_indicator(ta_rocp, ta_rocp_offset_len);
    ta_sar = pad_indicator(ta_sar, ta_sar_offset_len);
    ta_mfi = pad_indicator(ta_mfi, ta_mfi_offset_len);
    ta_atr = pad_indicator(ta_atr, ta_atr_offset_len);

    let results = TechnicalAdded {
        data: vec![ta_rsi, ta_sma, ta_rocp, ta_sar, ta_mfi, ta_atr],
        names: vec!["RSI", "SMA", "ROCP", "SAR", "MFI", "ATR"],
    };
    results
}
