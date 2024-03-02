extern crate dotenv;
extern crate rppal;

use cron::Schedule;
use log::{debug, error, info};
use reqwest::{Client, Response};
use rppal::gpio::{Gpio, Level};
use rppal::gpio::{Gpio, Level};
use serde_derive::{Deserialize, Serialize};
use std::{env, f32::consts::E};

#[derive(Serialize, Deserialize)]
struct ApiCodeTempoResponse {
    date_jour: String,
    code_jour: i32, // Assuming TarifColor is represented as an integer in Rust
    periode: String,
}

enum TarifColor {
    Unknown = 0,
    Blue = 1,
    White = 2,
    Red = 3,
}

impl From<i32> for TarifColor {
    fn from(item: i32) -> Self {
        match item {
            1 => TarifColor::Blue,
            2 => TarifColor::White,
            3 => TarifColor::Red,
            _ => TarifColor::Unknown,
        }
    }
}
async fn main() {
    // Initialize logging
    dotenv::dotenv().ok();
    env_logger::init();

    let color = getEDFColorOfTheDay().await.unwrap_or(TarifColor::Unknown);

    info!("So today we are in {} color", set_rgb_color(color));
}

async fn getEDFColorOfTheDay() -> Option<TarifColor> {
    let client = Client::new();

    match client
        .get("https://www.api-couleur-tempo.fr/api/jourTempo/today")
        .send()
        .await
    {
        Ok(response) => match response.json::<ApiCodeTempoResponse>().await {
            Ok(json_res) => Some(json_res.code_jour.into()), // Assuming code_jour matches with TarifColor
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn set_rgb_color(color: TarifColor) -> String {
    let red_pin = Gpio::new()
        .unwrap()
        .get(env::var("RED_PIN_NB"))
        .unwrap()
        .into_output();
    let green_pin = Gpio::new()
        .unwrap()
        .get(env::var("GREEN_PIN_NB"))
        .unwrap()
        .into_output();
    let blue_pin = Gpio::new()
        .unwrap()
        .get(env::var("BLUE_PIN_NB"))
        .unwrap()
        .into_output();
    match color {
        TarifColor::Blue => {
            red_pin.set_low();
            green_pin.set_low();
            blue_pin.set_high();
        }
        TarifColor::White => {
            red_pin.set_low();
            green_pin.set_high();
            blue_pin.set_high();
        }
        TarifColor::Unknown | TarifColor::Red => {
            red_pin.set_high();
            green_pin.set_low();
            blue_pin.set_low();
        }
    }
}
