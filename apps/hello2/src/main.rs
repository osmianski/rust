mod basics;
mod cli;

use lib::env;
use crate::basics::Error;

fn main() -> Result<(), Error> {
    env::load(".env");

    cli::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_description_of_list_command_in_command_list() {
        env::load(".env.testing");

        assert!(cli::fake::run("").see("List available commands"));
    }
}
