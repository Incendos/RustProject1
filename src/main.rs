use std::process::exit;
use chrono::NaiveDate;
use inquire::{CustomType, DateSelect, Select, Text};
use regex::Regex;
use crate::model::Rates;

mod network;
mod model;


fn main() {
    let mut cached_rates: Option<Rates> = None;
    let mut prompt_msg: String = "Currency query>".to_string();

    loop {
        let command = Text::new(&prompt_msg)
            .prompt()
            .unwrap();

        match command.as_str() {
            i if Regex::new("^load$").unwrap().is_match(i) => {
                let date =
                    DateSelect::new("Choose a date to view exchange rates from:").prompt();

                match network::fetch_rates(date.as_ref().unwrap()) {
                    Ok(rates) => {
                        cached_rates = Some(rates);
                        prompt_msg = format!("Query from: {}>", date.unwrap().format("%Y-%m-%d"));
                        println!("Exchange rates loaded successfully!");
                    },
                    Err(error) => {
                        println!("Failed to load exchange rates: {}", error);
                    }
                }
            }
            i if Regex::new("^load [0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap().is_match(i) => {
                let date =
                    NaiveDate::parse_from_str(i.split(" ").last().unwrap(),"%Y-%m-%d");

                if date.is_err() {
                    println!("Please enter a valid date!");
                    continue;
                }

                match network::fetch_rates(date.as_ref().unwrap()) {
                    Ok(rates) => {
                        cached_rates = Some(rates);
                        prompt_msg = format!("Query from: {}>", date.unwrap().format("%Y-%m-%d"));
                        println!("Exchange rates loaded successfully!");
                    },
                    Err(error) => {
                        println!("Failed to load exchange rates: {}", error);
                    }
                }
            }
            i if Regex::new("^latest$").unwrap().is_match(i) => {
                match network::fetch_latest() {
                    Ok(rates) => {
                        cached_rates = Some(rates);
                        prompt_msg = format!("Query from: {}>", cached_rates.as_ref().unwrap().date());
                        println!("Exchange rates loaded successfully!");
                    }
                    Err(error) => {
                        println!("Failed to load exchange rates: {}", error);
                    }
                }
            }
            i if Regex::new("^list$").unwrap().is_match(i) => {
                if cached_rates.is_none() {
                    println!("You need to load the exchange rates first!");
                    continue;
                }

                let options = cached_rates.as_ref().unwrap().rates()
                    .keys()
                    .collect::<Vec<&String>>();

                let new_base =
                    Select::new("Choose a currency as base:", options).prompt();

                cached_rates.as_ref().unwrap().with_base(new_base.unwrap()).unwrap().print();
            }
            i if Regex::new("^list [A-Z]{3}$").unwrap().is_match(i) => {
                if cached_rates.is_none() {
                    println!("You need to load the exchange rates first!");
                    continue;
                }

                let new_base = i.split(" ").last().unwrap();

                match cached_rates.as_ref().unwrap().with_base(new_base) {
                    Ok(rates) => {
                        rates.print();
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
            }
            i if Regex::new("^convert$").unwrap().is_match(i) => {
                if cached_rates.is_none() {
                    println!("You need to load the exchange rates first!");
                    continue;
                }

                let currencies = cached_rates.as_ref().unwrap()
                    .rates()
                    .keys()
                    .collect::<Vec<&String>>();

                let from =
                    Select::new("Choose a currency to convert from:", currencies.clone())
                        .prompt()
                        .unwrap();

                let to =
                    Select::new("Choose a currency to convert to:", currencies)
                        .prompt()
                        .unwrap();

                let amount =
                    CustomType::new("Enter the amount you want to convert:")
                        .with_error_message("Please enter a valid number!")
                        .prompt()
                        .unwrap();

                let result = cached_rates.as_ref().unwrap().convert(
                    from,
                    to,
                    amount,
                );

                match result {
                    Ok(r) => {
                        println!("{} {} equals {} {}.", amount, from, r, to);
                    }
                    Err(e) => {
                        println!("Failed to convert query: {}", e);
                    }
                }
            }
            i if Regex::new("^convert [A-Z]{3} [A-Z]{3} [0-9]+[.]?[0-9]*$").unwrap().is_match(i) => {
                if cached_rates.is_none() {
                    println!("You need to load the exchange rates first!");
                    continue;
                }

                let args = i.split(" ").collect::<Vec<&str>>();

                let amount = args[3].parse::<f32>();

                if amount.is_err() {
                    println!("Please enter a valid number!");
                    continue;
                }

                let result = cached_rates.as_ref().unwrap().convert(
                    args[1],
                    args[2],
                    amount.as_ref().unwrap().clone(),
                );

                match result {
                    Ok(r) => {
                        println!("{} {} equals {} {}.", amount.unwrap(), args[1], r, args[2]);
                    }
                    Err(e) => {
                        println!("Failed to convert query: {}", e);
                    }
                }
            }
            i if Regex::new("^quit$").unwrap().is_match(i) => {
                exit(0);
            }
            i => {
                println!("Unknown command: `{}`", i);
            }
        }
    }
}