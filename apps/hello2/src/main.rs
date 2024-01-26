mod basics;
mod cli;
mod http;

use lib::env;
use crate::basics::Result;

fn main() -> Result<()> {
    env::load(".env");

    cli::run()
}
