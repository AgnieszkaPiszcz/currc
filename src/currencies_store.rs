use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds::serialize as to_ts;
use std::{collections::{HashMap, HashSet}, fs, io::{Read, Write}, path};
use crate::api_access::fetch_supported_codes;

use super::app_error::AppError;

pub const CACHE_FILE: &str = "cache.json";

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
pub struct CurrenciesStore{
    pub supported_codes: HashSet<String>,
    pub items: HashMap<String, CurrencyData>,
}

impl CurrenciesStore {
    pub fn get(&self, code: String) -> Option<&CurrencyData> {
        if let Some(c) = self.items.get(&code.to_uppercase()) {
            if c.time_next_update_unix < Utc::now() {
                None
            } else {
                Some(c)
            }
        } else {
            None
        }
    }

    pub fn validate_code(&self, code: String) -> bool {
        if let Some(_) = self.supported_codes.get(&code.to_uppercase()) {
            true
        } else {
            false
        }
    }

    pub fn insert(&mut self, item: CurrencyData) {
        self.items.insert(item.base_code.clone(), item);
    }

    pub async fn load() -> Result<Self, AppError>  {
        let path = path::Path::new(CACHE_FILE);
        let mut file = fs::File::open(&path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let mut rates: CurrenciesStore = serde_json::from_str(&s)?;
        let codes = fetch_supported_codes().await?;
        rates.supported_codes = codes.supported_codes
            .into_iter()
            .map(|cur| cur.0)
            .collect();
        Ok(rates)
    }

    pub fn save(&self) -> Result<(), AppError> {
        let path = path::Path::new(CACHE_FILE);
        let mut file = fs::File::create(path)?;
        let data = serde_json::to_string_pretty(&self)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn new() -> Result<Self, AppError> {
        Ok(CurrenciesStore {
            supported_codes: HashSet::new(),
            items: HashMap::new(),
        })
    }
}

