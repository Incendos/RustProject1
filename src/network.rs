use std::collections::HashMap;
use chrono::NaiveDate;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use crate::model::Rates;

const URL_BASE: &str = "https://data.fixer.io/api";
const API_KEY: &str = env!("FIXER_KEY");
const URL_LATEST_PATH: &str = "latest";

pub fn fetch_latest() -> Result<Rates, String> {
    let url: String = format!("{}/{}?access_key={}", URL_BASE, URL_LATEST_PATH, API_KEY);

    process_response(reqwest::blocking::get(url))
}
pub fn fetch_rates(date: &NaiveDate) -> Result<Rates, String> {
    let url: String = format!("{}/{}?access_key={}",
                              URL_BASE,
                              date.format("%Y-%m-%d").to_string(),
                              API_KEY);

    process_response(reqwest::blocking::get(url))
}

fn process_response(response: Result<Response, reqwest::Error>) -> Result<Rates, String> {
    if response.is_err() {
        return Err(response.unwrap_err().to_string());
    }

    let response_text = response.unwrap().text().unwrap();

    let success: Result<FixerStatus, serde_json::Error> =
        serde_json::from_str(response_text.as_str());

    if success.is_err() {
        return Err(format!("Unexpected response: {}", response_text));
    }

    if !success.unwrap().success {
        let fixer_error: ErrorResponse =
            serde_json::from_str(response_text.as_str()).unwrap();

        return Err(format!("Fixer error, Code: {}, Type: {}, Info: {}",
                           fixer_error.error.code,
                           fixer_error.error.r#type,
                           fixer_error.error.info.unwrap_or("None".to_owned())));
    }

    let fixer_rates: RatesResponse =
        serde_json::from_str(response_text.as_str()).unwrap();

    Ok(fixer_rates.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatesResponse {
    success: bool,
    timestamp: i32,
    base: String,
    date: String,
    rates: HashMap<String, f32>
}

impl RatesResponse {
    pub fn base(&self) -> &String {
        &self.base
    }
    pub fn rates(&self) -> &HashMap<String, f32> {
        &self.rates
    }
    pub fn date(&self) -> &String {
        &self.date
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    success: bool,
    error: FixerError,
}

#[derive(Debug, Serialize, Deserialize)]
struct FixerError {
    code: i32,
    //#[serde(rename = "type")]
    r#type: String,
    info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FixerStatus {
    success: bool,
}