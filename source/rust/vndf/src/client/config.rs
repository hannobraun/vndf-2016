use std::fs::File;
use std::io::prelude::*;

use toml;

#[derive(Debug,Clone)]
pub struct Config {
    pub scaling_factor: f32,
}

impl Config {
    pub fn load() -> Config {
        let mut config = Config {
            scaling_factor: 1.0,
        };

        let mut config_file = match File::open("client-config.toml") {
            Ok(file) =>
                file,
            Err(error) => {
                warn!(
                    "Could not open config file. Using defaults. Error: {}",
                    error,
                );
                return config;
            },
        };

        let mut toml_config = String::new();
        if let Err(error) = config_file.read_to_string(&mut toml_config) {
            warn!(
                "Could not read from config file. Using defaults. Error: {}",
                error,
            );
            return config;
        }

        let mut parser = toml::Parser::new(&toml_config);
        let table = match parser.parse() {
            Some(table) =>
                table,
            None => {
                warn!("Error parsing config file. Using defaults. Errors:");
                for error in parser.errors {
                    warn!("Parsing error: {}", error);
                }
                return config;
            },
        };

        if let Some(scaling_factor) = table.get("scaling_factor") {
            match *scaling_factor {
                toml::Value::Float(scaling_factor) =>
                    config.scaling_factor = scaling_factor as f32,
                _ =>
                    warn!("Expected float type for scaling_factor. Using default"),
            }
        }

        config
    }
}
