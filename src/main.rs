mod args;
mod errors;

use args::Args;
use std::process::exit;

fn main() {
    let args = Args::new();

    match args.name {
        Some(value) => {
            if value.as_str() != "new" {
                eprintln!("Cannot have more than one argument if the first argument is not `new`.");
                exit(1);
            }
        }
        None => {}
    }
    match args.command.as_str() {
        "help" => display_available_commands(),
        "new" => add_new_todo(),
        "list" => list_all_todos(),
        "reminder" => {}
        _ => {
            eprintln!("Wrong command name. Use `todo help` to display all available commands.");
            exit(1);
        }
    }
}

fn display_available_commands() {
    let commands = vec!["help", "new", "list", "reminder"];
    println!("\nUsage: todo [COMMAND] [TEXT]\n\n");
    println!("Available commands: \n");

    for command in commands {
        match command {
            "help" => println!("help - Displays usage info and available commands.\n"),
            "new" => println!("new - Adds a new todo. Opens the system editor if argument [TEXT] is not specified.\n"),
            "list" => println!("list - Lists all available todos.\n"),
            "reminder" => println!("reminder - TBA\n"),
            // Impossible case.
            _ => ()
        }
    }
    println!();
}

fn add_new_todo() {}

fn list_all_todos() {}
