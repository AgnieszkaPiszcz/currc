#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("API request failure. Try again.")]
    RequestError(#[from] reqwest::Error),

    #[error("API access error: {0}. See https://www.exchangerate-api.com/docs/supported-codes-endpoint for more info.")]
    ApiAccessError(String),

    #[error("File handling error. Try again")]
    FileError(#[from] std::io::Error),

    #[error("Error serializing data. Try again")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid currency code: {0}. Try again")]
    CurrencyCodeError(String),

    #[error("CLI error. Try again.")]
    CLIError(#[from] inquire::CustomUserError),
}