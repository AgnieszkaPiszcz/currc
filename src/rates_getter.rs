use super::*;

pub async fn get_all_rates(base: String, cache: &mut CurrenciesStore) -> Result<HashMap<String, f32>, AppError> {
    if let Some(c) = cache.get(base.clone()) {
        Ok(c.conversion_rates.clone())
    } else {
        let c = fetch_currency_data(base).await?;
        cache.insert(c.clone());
        Ok(c.conversion_rates.clone())
    }
}

pub async fn get_amount(base: String, target: String, amount: f32, cache: &mut CurrenciesStore) -> Result<f32, AppError> {
    let rates = get_all_rates(base, cache).await?;
    if let Some(rate) = rates.get(&target) {
        Ok(rate * amount)
    } else {
        Err(AppError::CurrencyCodeError(target))
    }
}
