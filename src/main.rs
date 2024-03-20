#[cfg(test)]
mod tests;
mod non_interactive_cli;
mod currencies_store;
mod app_error;
mod api_access;
mod interactive_cli;
use std::collections::HashMap;
mod rates_getter;

use api_access::fetch_currency_data;
use clap::Parser;
use non_interactive_cli::*;
use currencies_store::*;
use app_error::AppError;
use rates_getter::*;
use interactive_cli::*;


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let mut cache = match CurrenciesStore::load().await {
        Ok(r) => r,
        Err(e) => {
            println!("{0}", e.to_string());
            std::process::exit(1)
        }
    };

    match args.command {
        Commands::I => {
            interactive_mode(&mut cache).await;
        },
        Commands::Convert{amount, base, target} => {
            if !cache.validate_code(base.clone()) {
                println!("Invalid currency code {base}");
                return;
            }
            if !cache.validate_code(target.clone()) {
                println!("Invalid currency code {target}");
                return;
            }
            let amount = if let Some(a) = amount {a} else {1.0};
            let converted_amount = get_amount(base.clone(), target.clone(), amount, &mut cache).await;
            let converted_amount = match converted_amount {
                Ok(a) => a,
                Err(e) => {
                    println!("{0}", e.to_string());
                    return;
                }
            };
            println!("{amount} {base} is {converted_amount} {target}");

        },
        Commands::AllRates{curr} => {
            match get_all_rates(curr.clone(), &mut cache).await {
                Ok(rates) => print_all_rates(curr, rates),
                Err(e) => println!("{0}", e.to_string()),
            }
        },
    };

    match cache.save() {
        Ok(_) => (),
        Err(e) => {
            println!("{0}", e.to_string());
            std::process::exit(1)
        },
    }
}

pub fn print_all_rates(base: String, rates: HashMap<String, f32>) {
    println!("Conversion rates for {base}: ");
    for (k, v) in rates {
        println!("{k}: {v}");
    }
}

