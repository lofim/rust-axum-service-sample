mod config;
mod error;
mod extractors;
mod handlers;
mod model;
mod server;
mod todo_store;
mod use_cases;

use std::{error::Error, str::FromStr};
use tracing::{debug, error, info, trace, Level};

use config::Config;
use server::init_http_server;

// TODO: move this into service utils
fn init_logger(config: &Config) -> Result<(), Box<dyn Error>> {
    let subscriber =
        tracing_subscriber::fmt().with_max_level(Level::from_str(&config.logger.level)?);

    if config.logger.format == "json" {
        subscriber.json().init();
    } else {
        subscriber.compact().with_ansi(true).init();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;

    init_logger(&config)?;
    info!("Loaded configuration: {:?}", &config);

    // demonstrate logger usage
    debug!("This is a debug log {:?}", &config);
    error!("This is an error log");
    trace!("This is a trace log");

    info!("Initializing http server...");
    init_http_server(&config).await?.await?;

    Ok(())
}
