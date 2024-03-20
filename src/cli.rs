use clap::{Parser, Subcommand};

/// Currency conversion tool
#[derive(Debug, Parser)] 
#[command(name = "currc", about, subcommand_required=true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Redownload supporteed currency codes
    #[arg(short, action)]
    pub refresh_codes: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Enter interactive mode
    I,
    /// Get exchange rate from base to target currency, if an amount is provided convert the amount from base to target currency
    Rate {
        /// Base currency 
        #[arg(index = 1)]
        base: String,
        /// Target currency
        #[arg(index = 2)]
        target: String,
        /// Amount to convert
        #[arg(index = 3)]
        amount: Option<f32>,
    },
    /// Get all exchange rates for a currency
    All {
        /// Currency code
        curr: String,
    },

}
