mod cli;

fn main() {
    cli::run(|| {
        let mut commands = cli::CommandMap::new();

        commands.insert("list", cli::Command {
            description: "List available commands",
            run_fn: |commands: &cli::CommandMap| {
                for (name, command) in commands {
                    println!("{}: {}", name, command.description);
                }
            },
        });
    
        commands.insert("serve", cli::Command {
            description: "Run HTTP server",
            run_fn: |_| {
                println!("Serve command");
            },
        });

        commands
    });
}