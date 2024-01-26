mod list;
mod serve;

use crate::basics::Result;
use lib::{cli, cli::{Command, Console}};
use std::io::stdout;

pub fn run() -> Result<()> {
    let command = Command::from_args();
    let mut console = Console::Real(stdout());

    dispatch(&command, &mut console)
}

#[cfg(test)]
pub mod fake {
    use lib::cli::{Command, Console, Fake};
    
    pub fn run(args: &'static str) -> Fake {
        let command = Command::from_str(args);
        let mut console = Console::Fake(Vec::new());
    
        super::dispatch(&command, &mut console).unwrap();
    
        Fake::new(console.output())
    }    
}

fn dispatch(command: &Command, console: &mut Console) -> Result<()> {
    match command.name.as_str() {
        "list" => list::run(console)?,
        "serve" => serve::run(console)?,
        _ => cli::not_found(command, console)?,
    };

    Ok(())
}
