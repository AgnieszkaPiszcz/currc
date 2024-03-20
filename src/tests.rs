use std::collections::HashMap;
use chrono::{Duration, Utc};

use super::*;

#[tokio::test]
async fn test_cache_get() {
    let mut cache = CurrenciesStore::new().unwrap();
    let item = CurrencyData {
        base_code: "USD".to_string(),
        time_next_update_unix: Utc::now() - Duration::try_days(2).unwrap(),
        conversion_rates: HashMap::new(),
    };
    cache.insert(item);
    let actual = cache.get("USD".to_string());
    assert!(actual.is_none());
}

