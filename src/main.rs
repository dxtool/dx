mod config;

use crate::config::Config;
use anyhow::Result;
use clap::App;

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> Result<()> {
    // load the configuration
    let _config = Config::load()?;

    // match program arguments
    let _matches = App::new(CARGO_PKG_NAME)
        .version(CARGO_PKG_VERSION)
        .about(CARGO_PKG_DESCRIPTION)
        .get_matches();

    Ok(())
}
