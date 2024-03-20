use super::*;
use std::str::FromStr;

use inquire::{error::InquireError, Select, Text};


pub async fn interactive_mode(cache: &mut CurrenciesStore) {
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
                        interactive_print_all_rates(cache).await;
                        loop {
                            let inner_opts = vec!["Again.", "Go back."];
                            let ans: Result<&str, InquireError> = Select::new("What do you want to do?", inner_opts.clone()).prompt();
                            match ans {
                                Ok(ch) => {
                                    match ch {
                                        "Again." => interactive_print_all_rates(cache).await,
                                        "Go back." => break,
                                        _=> unreachable!(),
                                    }
                                },
                                Err(_) => println!("There was an error, please try again."),
                            }
                        }
                    }
                    _ if choice == opts[1] => {
                        interactive_print_rate(cache).await;
                        loop {
                            let inner_opts = vec!["Again.", "Go back."];
                            let ans: Result<&str, InquireError> = Select::new("What do you want to do?", inner_opts.clone()).prompt();
                            match ans {
                                Ok(ch) => {
                                    match ch {
                                        "Again." => interactive_print_rate(cache).await,
                                        "Go back." => break,
                                        _=> unreachable!(),
                                    }
                                },
                                Err(_) => println!("There was an error, please try again."),
                            }
                        }
                    }
                    _ if choice == opts[2] => {
                        interactive_print_amount(cache).await;
                        loop {
                            let inner_opts = vec!["Again.", "Go back."];
                            let ans: Result<&str, InquireError> = Select::new("What do you want to do?", inner_opts.clone()).prompt();
                            match ans {
                                Ok(ch) => {
                                    match ch {
                                        "Again." => interactive_print_amount(cache).await,
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
}

async fn interactive_print_all_rates(cache: &mut CurrenciesStore) {
    let curr = Text::new("Base currency code: ").prompt();
    let curr = match curr {
        Ok(c) => c.to_uppercase(),
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    if !cache.validate_code(curr.clone()) {
        println!("Invalid currency code {curr}");
        return;
    }
    let rates = get_all_rates(curr.clone(), cache).await;
    let rates = match rates {
        Ok(c) => c,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    print_all_rates(curr, rates);
}

async fn interactive_print_rate(cache: &mut CurrenciesStore) {
    let base = Text::new("Base currency code: ").prompt();
    let base = match base {
        Ok(c) => c.to_uppercase(),
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    if !cache.validate_code(base.clone()) {
        println!("Invalid currency code {base}");
        return;
    }
    let target = Text::new("Base currency code: ").prompt();
    let target = match target {
        Ok(c) => c.to_uppercase(),
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    if !cache.validate_code(target.clone()) {
        println!("Invalid currency code {target}");
        return;
    }
    let amount = get_amount(base.clone(), target.clone(), 1.0, cache).await;
    let amount = match amount {
        Ok(a) => a,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    println!("1 {base} is {amount} {target}");
}

pub async fn interactive_print_amount(cache: &mut CurrenciesStore) {
    let base = Text::new("Base currency code: ").prompt();
    let base = match base {
        Ok(c) => c.to_uppercase(),
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    if !cache.validate_code(base.clone()) {
        println!("Invalid currency code {base}");
        return;
    }
    let target = Text::new("Target currency code: ").prompt();
    let target = match target {
        Ok(c) => c.to_uppercase(),
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    if !cache.validate_code(target.clone()) {
        println!("Invalid currency code {target}");
        return;
    }
    let amount = Text::new("Amount to convert: ").prompt();
    let amount = match amount {
        Ok(c) => {
            match f32::from_str(&c) {
                Ok(a) => a,
                Err(e) => {
                    println!("{0}", e.to_string());
                    return;
                }
            }
        } 
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    let converted_amount = get_amount(base.clone(), target.clone(), amount, cache).await;
    let converted_amount = match converted_amount {
        Ok(a) => a,
        Err(e) => {
            println!("{0}", e.to_string());
            return;
        }
    };
    println!("{amount} {base} is {converted_amount} {target}");

}

