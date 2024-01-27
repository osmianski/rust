mod basics;
mod cli;
mod http;
mod inertia;

use crate::basics::Result;
use lib::env;

fn main() -> Result<()> {
    env::load(".env");

    cli::run()
}
