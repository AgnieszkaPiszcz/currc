use serde::{Deserialize, Serialize};

use super::app_error::AppError;
use super::*;

pub const API_KEY: &str = ""; // paste your api key here

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIResponse {
    CurrencyData(CurrencyData),
    CurrencyCodes(CurrencyCodes),
    Error(ErrResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrResponse {
    #[serde(alias = "error-type")]
    pub error_type: String,
}

pub async fn fetch_currency_data(base: String) -> Result<CurrencyData, AppError> {
    let url = format!("https://v6.exchangerate-api.com/v6/{API_KEY}/latest/{base}");
    let res: APIResponse = reqwest::get(url).await?.error_for_status()?.json().await?;
    match res {
        APIResponse::CurrencyData(c) => {
            Ok(c)
        },
        APIResponse::Error(e) => Err(AppError::ApiAccessError(e.error_type)),
        _=> unreachable!(),
    }
}

pub async fn fetch_supported_codes() -> Result<CurrencyCodes, AppError> {
    let url = format!("https://v6.exchangerate-api.com/v6/{API_KEY}/codes");
    let codes = reqwest::get(url).await?.error_for_status()?.json().await?;
    match codes {
        APIResponse::CurrencyCodes(c) => Ok(c),
        APIResponse::Error(e) => Err(AppError::ApiAccessError(e.error_type)),
        _=> unreachable!(),
    }
}
