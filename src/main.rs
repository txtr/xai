use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
struct KeyError;

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined key")
    }
}

impl std::error::Error for KeyError {}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct sideshift_ai_api_v1_pairs_saibal_xlm {
    min: String,
    max: String,
    rate: String,
    estimatedNetworkFeesUsd: String,
}

fn convert_xai_to_xlm(xai_amt: f32) -> Result<f32, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://sideshift.ai/api/v1/pairs/saibal/xlm")?.text()?;
    let deserialized_json: sideshift_ai_api_v1_pairs_saibal_xlm = serde_json::from_str(&resp)?;
    let rate: f32 = deserialized_json.rate.parse::<f32>()?;
    return Ok(rate * xai_amt);
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct sideshift_ai_api_v1_xai_stats {
    totalSupply: u32,
    circulatingSupply: String,
    numberOfStakers: u32,
    latestAnnualPercentageYield: String,
    latestDistributedXai: String,
    totalStaked: String,
    averageAnnualPercentageYield: String,
    totalValueLocked: String,
    totalValueLockedRatio: String,
}

fn get_apy() -> Result<f32, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://sideshift.ai/api/v1/xai/stats")?.text()?;
    let deserialized_json: sideshift_ai_api_v1_xai_stats = serde_json::from_str(&resp)?;
    let apy: f32 = deserialized_json
        .latestAnnualPercentageYield
        .parse::<f32>()?;
    return Ok(apy);
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct _volume {
    max: String,
    min: String,
    volume: f32,
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct _XLM {
    highest_buy_bid: f32,
    lowest_sell_bid: f32,
    last_traded_price: f32,
    yes_price: f32,
    volume: _volume,
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct bitbns_com_order_getTickerWithVolume {
    XLM: _XLM,
}

fn convert_xlm_to_inr(xlm_amt: f32) -> Result<f32, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://bitbns.com/order/getTickerWithVolume")?.text()?;
    let deserialized_json: bitbns_com_order_getTickerWithVolume = serde_json::from_str(&resp)?;
    let rate: f32 = deserialized_json.XLM.last_traded_price;
    return Ok(rate * xlm_amt);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xai_amt: f32 = 100;
    let xlm_amt = convert_xai_to_xlm(xai_amt)?;
    let inr_amt = convert_xlm_to_inr(xlm_amt)?;
    let latest_apy = get_apy()?;
    println!(
        "XAI = {} >>> XLM = {} >>> INR = ₹{}\tLatest Annual Percent Yield = {}%",
        xai_amt, xlm_amt, inr_amt, latest_apy
    );
    Ok(())
}
