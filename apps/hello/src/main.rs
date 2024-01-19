mod cli;

fn main() {
    cli::run(|| {
        let mut commands = cli::CommandMap::new();

        commands.insert("list", cli::Command {
            description: "List available commands",
            run_fn: |commands| {
                for (key, value) in commands {
                    println!("{}: {}", key, value.description);
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