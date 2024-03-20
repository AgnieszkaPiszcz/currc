#[cfg(test)]
mod tests;
mod cli;
mod rates_getter;
mod err;

use std::collections::HashSet;

use clap::Parser;
use cli::*;
use rates_getter::*;
use inquire::{error::InquireError, Select, Text};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let codes = match get_codes(args.refresh_codes).await {
        Ok(r) => r,
        Err(e) => {
            println!("{0}", e.to_string());
            std::process::exit(1)
        }
    };
    let mut cache = match Cache::load(){
        Ok(r) => r,
        Err(e) => {
            println!("{0}", e.to_string());
            std::process::exit(1)
        }
    };

    match args.command {
        Commands::I => {
            loop {
                let opts = vec![
                "Get all conversion rates for a currency.", 
                "Get conversion rate from base to target currency.",
                "Convert an amount from base to target currency.",
                "Quit.",
                ];
                let ans: Result<&str, InquireError> = Select::new("What do you want to do?", opts.clone()).prompt();
                match ans {
                    Ok(choice) => {
                        match choice {
                            _ if choice == opts[0] => {
                                interactive_print_all_rates(&codes, &mut cache).await;
                                loop {
                                    let inner_opts = vec!["Again.", "Go back."];
                                    let ans: Result<&str, InquireError> = Select::new("What do you want to do?", inner_opts.clone()).prompt();
                                    match ans {
                                        Ok(ch) => {
                                            match ch {
                                                "Again." => interactive_print_all_rates(&codes, &mut cache).await,
                                                "Go back." => break,
                                                _=> unreachable!(),
                                            }
                                        },
                                        Err(_) => println!("There was an error, please try again."),
                                    }
                                }
                            }
                            _ if choice == opts[1] => {
                                interactive_print_rate(&codes, &mut cache).await;
                                loop {
                                    let inner_opts = vec!["Again.", "Go back."];
                                    let ans: Result<&str, InquireError> = Select::new("What do you want to do?", inner_opts.clone()).prompt();
                                    match ans {
                                        Ok(ch) => {
                                            match ch {
                                                "Again." => interactive_print_rate(&codes, &mut cache).await,
                                                "Go back." => break,
                                                _=> unreachable!(),
                                            }
                                        },
                                        Err(_) => println!("There was an error, please try again."),
                                    }
                                }
                            }
                            _ if choice == opts[2] => {
                                interactive_print_amount(&codes, &mut cache).await;
                                loop {
                                    let inner_opts = vec!["Again.", "Go back."];
                                    let ans: Result<&str, InquireError> = Select::new("What do you want to do?", inner_opts.clone()).prompt();
                                    match ans {
                                        Ok(ch) => {
                                            match ch {
                                                "Again." => interactive_print_amount(&codes, &mut cache).await,
                                                "Go back." => break,
                                                _=> unreachable!(),
                                            }
                                        },
                                        Err(e) => println!("{0}", e.to_string()),
                                    }
                                }
                            }
                            _ if choice == opts[3] => {
                                break;
                            }
                            _ => unreachable!(),
                        }
                    },
                    Err(e) => println!("{0}", e.to_string()),
                }
            }

        },
        Commands::Rate{amount, base, target} => {
            print_rate(amount, base, target, &codes, &mut cache).await;
        },
        Commands::All{curr} => {
            print_all_rates(curr, &codes, &mut cache).await;
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

async fn print_rate(amount: Option<f32>, base: String, target: String, codes: &HashSet<String>, cache: &mut Cache) {
    let amount = if let Some(amount) = amount { amount } else { 1.0 };
    let target_amount = match get_amount(base.to_uppercase(), target.to_uppercase(), amount, codes, cache).await {
        Ok(r) => r,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    println!("{amount} {base} = {target_amount} {target}");
}

async fn print_all_rates(curr: String, codes: &HashSet<String>, cache: &mut Cache) {
    let rates = get_all_rates(curr.to_uppercase(), codes, cache).await;
    let rates = match rates {
        Ok(r) => r,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    println!("Currency echange rates for {curr}:");
    for (k, v) in rates {
        println!("{k} : {v}")
    }
}

async fn interactive_print_all_rates(codes: &HashSet<String>, cache: &mut Cache) {
    let curr = Text::new("Base currency code: ").prompt();
    let curr = match curr {
        Ok(c) => c,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }

    };
    print_all_rates(curr, &codes, cache).await;
}

async fn interactive_print_rate(codes: &HashSet<String>, cache: &mut Cache) {
    let base = Text::new("Base currency code: ").prompt();
    let base = match base {
        Ok(c) => c,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    let target = Text::new("Target currency code: ").prompt();
    let target = match target {
        Ok(c) => c,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    print_rate(Some(1.0), base, target, &codes, cache).await;
}

async fn interactive_print_amount(codes: &HashSet<String>, cache: &mut Cache) {
    let base = Text::new("Base currency code: ").prompt();
    let base = match base {
        Ok(c) => c,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    let target = Text::new("Target currency code: ").prompt();
    let target = match target {
        Ok(c) => c,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    let amount = Text::new("Amount to convert: ").prompt();
    let amount = match amount {
        Ok(c) => {
            let a = f32::from_str(&c);
            match a {
                Ok(a) => a,
                Err(e) => {
                    println!("{0}", e.to_string());
                    return;
                }            }
        },
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    print_rate(Some(amount), base, target, &codes, cache).await;
}
