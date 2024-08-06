#![deny(clippy::all)]

use serde_json::Value;
use std::{env, error::Error};

const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";
const MANUFACTURER_COMMON_NAME: &str = "Mfr_CommonName";
const MANUFACTURER_NAME: &str = "Mfr_Name";
const MANUFACTURER_COUNTRY: &str = "Country";

struct AutoManufacturer<'a> {
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>,
}

impl<'a> AutoManufacturer<'a> {
    fn display(&self) {
        println!("\t Manufacturer Name: {}", self.name.unwrap_or_default());
        println!(
            "\t Manufacturer Common Name: {}",
            self.common_name.unwrap_or_default()
        );
        println!("\t Country: {}", self.country.unwrap_or_default());
        println!();
    }

    fn contains(&self, needle: &str) -> bool {
        self.name.unwrap_or("").contains(needle)
            || self.common_name.unwrap_or("").contains("needle")
            || self.country.unwrap_or("").contains("needle")
    }
}

fn print_help() {
    println!("Usage: <executable_path> [ARG]");
}

fn print_result(manufacturers: &[AutoManufacturer]) {
    println!("{} manufacturers found", manufacturers.len());
    for (idx, manufacturer) in manufacturers.iter().enumerate() {
        println!("Manufacturer #{}", idx + 1);
        manufacturer.display();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        print_help();
        return Err("help".into());
    }

    let needle = if args.len() == 2 { &args[1] } else { "" };

    let response = reqwest::get(API_URL)
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(obj) = response.as_object() {
        if let Some((_, value)) = obj.iter().find(|(key, _)| key == &"Results") {
            if let Some(manufacturers) = value.as_array() {
                let empty_string = Value::String("".to_string());

                let manufacturers_iter = manufacturers.iter().map(|m| {
                    let manufacturer = m.as_object().unwrap();

                    let common_name = manufacturer
                        .get(MANUFACTURER_COMMON_NAME)
                        .unwrap_or(&empty_string)
                        .as_str();
                    let name = manufacturer
                        .get(MANUFACTURER_NAME)
                        .unwrap_or(&empty_string)
                        .as_str();
                    let country = manufacturer
                        .get(MANUFACTURER_COUNTRY)
                        .unwrap_or(&empty_string)
                        .as_str();

                    AutoManufacturer {
                        common_name,
                        name,
                        country,
                    }
                });

                let result: Vec<AutoManufacturer> = if needle.is_empty() {
                    manufacturers_iter.collect()
                } else {
                    manufacturers_iter.filter(|m| m.contains(needle)).collect()
                };

                if result.is_empty() {
                    return Err("No manufacturer found".into());
                }

                print_result(&result);
            }
        } else {
            return Err("Results not found in response".into());
        }
    } else {
        return Err("Response is not a json object".into());
    }

    Ok(())
}
