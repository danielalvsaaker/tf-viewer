use crate::models::Unit;
use serde::Deserialize;
use std::{fs::read, net::Ipv4Addr};

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    cookie_key: String,
    #[serde(default)]
    pub secure_cookies: bool,
    #[serde(default)]
    pub disable_registration: bool,
    #[serde(default = "default_address")]
    pub address: Ipv4Addr,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_units")]
    pub units: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            cookie_key: Default::default(),
            secure_cookies: false,
            disable_registration: false,
            address: default_address(),
            port: default_port(),
            units: default_units(),
        }
    }
}

impl Config {
    pub fn get_units(&self) -> Unit {
        match self.units.as_str() {
            "metric" => Unit::Metric,
            "imperial" => Unit::Imperial,
            _ => {
                println!("Failed to read unit: using metric. Valid keywords are metric/imperial.");
                Unit::Metric
            }
        }
    }

    pub fn get_cookie_key(&self) -> Vec<u8> {
        let parsed_key = self.cookie_key.as_bytes().to_vec();
        if parsed_key.len() < 32 {
            println!("Cookie key is missing or shorter than 32 bytes, generating a key...");

            let mut key = [0u8; 32];
            getrandom::getrandom(&mut key).unwrap();
            key.to_vec()
        } else {
            parsed_key
        }
    }
}

fn default_address() -> Ipv4Addr {
    Ipv4Addr::new(127, 0, 0, 1)
}

fn default_units() -> String {
    "metric".into()
}

fn default_port() -> u16 {
    8080
}

pub fn config() -> Config {
    if let Ok(bytes) = read("config.toml") {
        let config = String::from_utf8(bytes).expect("Config file is not valid UTF-8.");

        toml::from_str(&config).expect("Failed to parse config.")
    } else {
        let config = Config::default();

        println!("Config file not found, starting with defaults.");

        config
    }
}
