mod basics;
mod cli;
mod http;

use crate::basics::Result;
use lib::env;

fn main() -> Result<()> {
    env::load(".env");

    cli::run()
}
