mod basics;
mod cli;

use crate::basics::Result;
use lib::env;

fn main() -> Result<()> {
    env::load(".env");

    cli::run()
}
