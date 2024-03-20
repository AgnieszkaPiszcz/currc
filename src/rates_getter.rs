use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds::serialize as to_ts;
use std::{collections::{HashMap, HashSet}, fs, io::{Read, Write}, path};
use super::err::AppError;

pub const API_KEY: &str = ""; // paste your api key here
pub const CACHE_FILE: &str = "cache.txt";
pub const CODES_FILE: &str = "codes.txt";

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIResponse {
    CurrencyData(CurrencyData),
    CurrencyCodes(CurrencyCodes),
    Error(ErrResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyData {
    pub base_code: String,
    #[serde(deserialize_with = "from_ts")]
    #[serde(serialize_with = "to_ts")]
    pub time_next_update_unix: DateTime<Utc>,
    pub conversion_rates: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyCodes {
    pub supported_codes: HashSet<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrResponse {
    #[serde(alias = "error-type")]
    pub error_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache{
    #[serde(deserialize_with = "from_ts")]
    #[serde(serialize_with = "to_ts")]
    pub time_next_update_unix: DateTime<Utc>,
    pub items: HashMap<String, CurrencyData>,
}

impl Cache {
    pub fn get(&self, code: String) -> Option<&CurrencyData> {
        if let Some(c) = self.items.get(&code) {
            if c.time_next_update_unix < Utc::now() {
                None
            } else {
                Some(c)
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, item: CurrencyData) {
        self.time_next_update_unix = item.time_next_update_unix;
        self.items.insert(item.base_code.clone(), item);
    }

    pub fn load() -> Result<Self, AppError>  {
        let path = path::Path::new(CACHE_FILE);
        let mut file = fs::File::open(&path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let c: Cache = serde_json::from_str(&s)?;
        if c.time_next_update_unix < Utc::now() {
            Cache::new()
        } else {
            Ok(c)
        }
    }

    pub fn save(&self) -> Result<(), AppError> {
        let path = path::Path::new(CACHE_FILE);
        let mut file = fs::File::create(path)?;
        let data = serde_json::to_string_pretty(&self)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn new() -> Result<Self, AppError> {
        Ok(Cache {
            time_next_update_unix: Utc::now(),
            items: HashMap::new(),
        })
    }
}


pub async fn get_rate(base: String, target: String, codes: &HashSet<String>, cache: &mut Cache) -> Result<f32, AppError> {
    if let None = codes.get(&base) {
        return Err(AppError::CurrencyCodeError(base));
    }
    if let None = codes.get(&target) {
        return Err(AppError::CurrencyCodeError(target));
    }
    let base_ = base.clone();
    let rates = get_all_rates(base, codes, cache);
    rates.await?.get(&target).copied().ok_or_else(|| AppError::CurrencyCodeError(format!("{target}. Code not found in {base_} exchange rates")))
}

pub async fn get_amount(base: String, target: String, amount: f32, codes: &HashSet<String>, cache: &mut Cache) -> Result<f32, AppError> {
    if let None = codes.get(&base) {
        return Err(AppError::CurrencyCodeError(base));
    }
    if let None = codes.get(&target) {
        return Err(AppError::CurrencyCodeError(target));
    }
    let rate = get_rate(base, target, codes, cache).await?;
    Ok(amount * rate)
}

pub async fn get_all_rates(base: String, codes: &HashSet<String>, cache: &mut Cache) -> Result<HashMap<String, f32>, AppError> {
    let url = format!("https://v6.exchangerate-api.com/v6/{API_KEY}/latest/{base}");
    if let Some(_) = codes.get(&base) {
        if let Some(curr) = cache.get(base) {
            Ok(curr.conversion_rates.clone())
        } else {
            get_rates_from_api(url, cache).await
        }
    } else {
        Err(AppError::CurrencyCodeError(base))
    }
}

pub async fn get_rates_from_api(url: String, cache: &mut Cache) -> Result<HashMap<String, f32>, AppError> {
    let res:APIResponse = reqwest::get(url).await?.error_for_status()?.json().await?;
        match res {
            APIResponse::CurrencyData(c) => {
                cache.insert(c.clone());
                Ok(c.conversion_rates)
            },
            APIResponse::Error(e) => Err(AppError::ApiAccessError(e.error_type)),
            _=> unreachable!(),
        }
}

pub async fn get_codes(refresh_codes: bool) -> Result<HashSet<String>, AppError> {
    if refresh_codes {
        let url = format!("https://v6.exchangerate-api.com/v6/{API_KEY}/codes");
        get_codes_from_api(url).await
    } else {
        get_codes_from_file()
    }
}

pub fn get_codes_from_file() -> Result<HashSet<String>, AppError> {
    let path = path::Path::new(CODES_FILE);
    let mut file = fs::File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let codes: CurrencyCodes = serde_json::from_str(&s)?;
    let c: HashSet<String> = codes.supported_codes
        .into_iter()
        .map(|cur| cur.0)
        .collect();
    Ok(c)
}

pub async fn get_codes_from_api(url: String) -> Result<HashSet<String>, AppError> {
    let codes = reqwest::get(url).await?.error_for_status()?.json().await?;
    match codes {
        APIResponse::CurrencyCodes(c) => {
            let path = path::Path::new(CODES_FILE);
            let mut file = fs::File::create(path)?;
            let data = serde_json::to_string(&c)?;
            file.write_all(data.as_bytes())?;
            let c: HashSet<String> = c.supported_codes
                .into_iter()
                .map(|cur| cur.0)
                .collect();
            Ok(c)
        },
        APIResponse::Error(e) => Err(AppError::ApiAccessError(e.error_type)),
        _=> unreachable!(),
    }
}

