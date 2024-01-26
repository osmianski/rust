use std::{
    env, io,
    io::{Result, Stdout, Write},
};

pub struct Command {
    pub name: String,
}

impl Command {
    pub fn from_args() -> Command {
        let args = env::args();
        let name = args.skip(1).next().unwrap_or("list".to_string());

        Command { name }
    }

    pub fn from_str(s: &str) -> Command {
        let mut args = s.split_whitespace();
        let name = args.next().unwrap_or("list").to_string();

        Command { name }
    }
}

pub struct Fake {
    output: String,
}

impl Fake {
    pub fn new(output: String) -> Fake {
        Fake { output }
    }

    pub fn see(&self, s: &str) -> bool {
        self.output.contains(s)
    }
}

pub enum Console {
    Real(Stdout),
    Fake(Vec<u8>),
}

impl Console {
    pub fn new() -> Console {
        Console::Real(io::stdout())
    }

    pub fn write(&mut self, s: &str) -> Result<usize> {
        match self {
            Console::Real(stdout) => stdout.write(s.as_bytes()),
            Console::Fake(fake) => fake.write(s.as_bytes()),
        }
    }

    pub fn writeln(&mut self, s: &str) -> Result<usize> {
        self.write(s)?;
        self.write("\n")
    }

    pub fn output(&self) -> String {
        match self {
            Console::Real(_) => panic!("Cannot get output from real console"),
            Console::Fake(fake) => String::from_utf8(fake.clone()).unwrap(),
        }
    }
}

pub fn not_found(command: &Command, console: &mut Console) -> Result<()> {
    console.writeln(format!("Command not found: {}", command.name).as_str())?;

    Ok(())
}
