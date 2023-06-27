use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub logger: LoggerConfig,
}

#[derive(Debug)]
pub struct LoggerConfig {
    pub format: String,
    pub level: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let logger_config = LoggerConfig {
            format: env::var("LOGGER_FORMAT").unwrap_or_else(|_| "kvp".to_owned()),
            level: env::var("LOGGER_LEVEL").unwrap_or_else(|_| "INFO".to_owned()),
        };

        Ok(Config {
            // TODO: make env var extraction more robus and introduce proper error handling
            port: env::var("PORT")
                .map(|p| p.parse::<u16>())
                .unwrap_or(Ok(8080))?,
            logger: logger_config,
        })
    }
}
