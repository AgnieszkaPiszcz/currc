use super::*;

#[tokio::test]
async fn test_get_rates_from_api() {
    let url = format!("https://v6.exchangerate-api.com/v6/{API_KEY}/latest/USD");
    let mut cache = Cache::new().unwrap();
    let rates1 = get_rates_from_api(url, &mut cache).await.unwrap();
    let rates2 = cache.get("USD".to_string()).ok_or("USD not saved to cache").unwrap();
    assert_eq!(rates1, rates2.conversion_rates)
}

#[tokio::test]
async fn test_cache_get() {
    let url = format!("https://v6.exchangerate-api.com/v6/{API_KEY}/latest/USD");
    let mut cache = Cache::new().unwrap();
    let rates1 = get_rates_from_api(url, &mut cache).await.unwrap();
    let rates2 = cache.get("USD".to_string()).ok_or("USD not saved to cache").unwrap();
    assert_eq!(rates1, rates2.conversion_rates)
}

