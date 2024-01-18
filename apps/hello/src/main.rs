use std::collections::HashMap;
use std::env;

fn main() {
    let mut command_definitions = HashMap::new();

    command_definitions.insert("list", CommandDefiniton {
        description: "List available commands",
        factory: || Box::new(ListCommand {
            //commands: command_definitions.clone(),
        }),
    });

    command_definitions.insert("serve", CommandDefiniton {
        description: "Run HTTP server",
        factory: || Box::new(ServeCommand {}),
    });

    // `args`` is an iterator over the command line arguments. The first argument is always
    // the name of the executable. The second optional argument is the name of the command.
    let args = env::args();

    // `args` iterator returns `String` instances. `command_name` owns the second argument
    // if it exists, or is set to `"list"` otherwise. Note that the type is `String` and not
    // `&str`. If we used `&str`, the `command_name` variable would be a reference to the
    // data of a `String` instance, and that instance would have to live as long as the `command_name`.
    let command_name = args.skip(1).next().unwrap_or("list".to_string());

    let command_definition = command_definitions.get(command_name.as_str()).unwrap_or_else(|| {
        println!("Could not find command {}", command_name);
        std::process::exit(1);
    });

    let command = (command_definition.factory)();

    command.run();
}

trait Command {
    fn run(&self);
}

struct CommandDefiniton<'a> {
    // `'a` is a lifetime specifier. It means that the `CommandDefinition` struct 
    // cannot outlive the `description` string reference it contains. In practice,
    // the `CommandDefiniton` instances live as long as the `main()` function, and
    // the `description` strings are stored within the executable and here live as
    // long as the executable is running. Which is longer than the `main()` function.
    #[allow(dead_code)]
    description: &'a str,

    // `dyn FnOnce() -> Box<dyn Command>` is a pointer to a closure. It returns a `Box` - 
    // a pointer to a heap-allocated object. The object is of type `dyn Command` -
    // a trait object. It means an instance of any type that implements the `Command`
    // trait. To sum up, the `factory` field is a closure pointer that returns a
    // pointer to a heap-allocated object of any type that implements the `Command`
    // trait.
    factory: fn() -> Box<dyn Command>,
}

struct ListCommand {
    //commands: HashMap<&'a str, CommandDefiniton<'a>>,
}

impl Command for ListCommand {
    fn run(&self) {
        println!("List command");
    }    
}

struct ServeCommand {}

impl Command for ServeCommand {
    fn run(&self) {
        println!("Serve command");
    }    
}