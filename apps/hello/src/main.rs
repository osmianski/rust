mod env;

fn main() {
    env::load(".env");

    // `args`` is an iterator over the command line arguments. The first argument is always
    // the name of the executable. The second optional argument is the name of the command.
    let args = std::env::args();

    // `args` iterator returns `String` instances. `command_name` owns the second argument
    // if it exists, or is set to `"list"` otherwise. Note that the type is `String` and not
    // `&str`. If we used `&str`, the `command_name` variable would be a reference to the
    // data of a `String` instance, and that instance would have to live as long as the `command_name`.
    let command_name = args.skip(1).next().unwrap_or("list".to_string());

    // Match the command name against the list of predefined commands and execute the
    // corresponding function. If the command is not found, print an error message and exit
    // with a non-zero exit code.
    if command_name.eq("list") {
        list();
    } 
    else if command_name.eq("serve") {
        serve();
    } 
    else {
        println!("Could not find command {}", command_name);
        std::process::exit(1);
    }
}

fn list() {
    println!("{:20}{}", "list", "List available commands");
    println!("{:20}{}", "serve", "Run HTTP server");
}

fn serve() {
    unimplemented!();
}

