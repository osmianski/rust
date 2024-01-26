use std::io::stdout;
use crate::basics::Error;
use lib::{cli, cli::{Command, Console}};

pub fn run() -> Result<(), Error> {
    let command = Command::from_args();
    let mut console = Console::Real(stdout());

    dispatch(&command, &mut console)
}

#[cfg(test)]
pub mod fake {
    use super::*;
    use lib::cli::Fake;
    
    pub fn run(args: &'static str) -> Fake {
        let command = Command::from_str(args);
        let mut console = Console::Fake(Vec::new());
    
        dispatch(&command, &mut console).unwrap();
    
        Fake::new(console.output())
    }    
}

fn dispatch(command: &Command, console: &mut Console) -> Result<(), Error> {
    match command.name.as_str() {
        "list" => list(console)?,
        _ => cli::not_found(command, console)?,
    };

    Ok(())
}

fn list(console: &mut Console) -> Result<(), Error> {
    console.write("
list                        List available commands
    
")?;

    Ok(())
}

