use crate::basics::Result;
use lib::cli::Console;

pub fn run(console: &mut Console) -> Result<()> {
    console.write(
        "
list                        List available commands
serve                       Run HTTP server
    
",
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cli::fake;
    use lib::env;

    #[test]
    fn it_prints_description_of_list_command() {
        env::load(".env.testing");

        assert!(fake::run("").see("List available commands"));
    }
}
