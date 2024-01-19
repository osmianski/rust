use std::collections::HashMap;
use std::env;

pub struct Command<'a> {
    // `'a` is a lifetime specifier. It means that the `CommandDefinition` struct 
    // cannot outlive the `description` string reference it contains. In practice,
    // the `CommandDefiniton` instances live as long as the `main()` function, and
    // the `description` strings are stored within the executable and here live as
    // long as the executable is running. Which is longer than the `main()` function.
    pub description: &'a str,

    // `Box<dyn Fn(&CommandMap)>` is a pointer to a closure having 1 parameter and 
    // returning no result. It returns a `Box` - 
    // a pointer to a heap-allocated object. The object is of type `dyn Command` -
    // a trait object. It means an instance of any type that implements the `Command`
    // trait. To sum up, the `factory` field is a closure pointer that returns a
    // pointer to a heap-allocated object of any type that implements the `Command`
    // trait.
    pub run_fn: fn(commands: &CommandMap),
}

pub type CommandMap<'a> = HashMap<&'a str, Command<'a>>;

pub fn run<'a, F>(f: F) where F: FnOnce() -> CommandMap<'a> {
    // The `f` callback creates and fills in a command hash map and returns it.
    // The ownership of the hash map is moved to the `commands` variable.
    let commands = f();

    // `args`` is an iterator over the command line arguments. The first argument is always
    // the name of the executable. The second optional argument is the name of the command.
    let args = env::args();

    // `args` iterator returns `String` instances. `command_name` owns the second argument
    // if it exists, or is set to `"list"` otherwise. Note that the type is `String` and not
    // `&str`. If we used `&str`, the `command_name` variable would be a reference to the
    // data of a `String` instance, and that instance would have to live as long as the `command_name`.
    let command_name = args.skip(1).next().unwrap_or("list".to_string());

    let command = commands.get(command_name.as_str()).unwrap_or_else(|| {
        println!("Could not find command {}", command_name);
        std::process::exit(1);
    });

    (command.run_fn)(&commands);
}