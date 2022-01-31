extern crate directories;

#[macro_use]
mod errors;
mod args;

use args::Args;
use chrono;
use directories::ProjectDirs;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args = Args::new();

    match args.command.as_str() {
        "help" => display_available_commands(),
        "new" => add_new_todo(&args.name),
        "list" => list_all_todos(),
        "reminder" => {}
        _ => {
            equit!("Wrong command name. Use `todo help` to display all available commands.");
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

fn add_new_todo(name_reference: &Option<String>) {
    if let Some(data_directory) = ProjectDirs::from("", "", "Todo Rust") {
        init_folder_if_not_existent(data_directory.config_dir());
        create_todo_file(data_directory.config_dir(), name_reference);
    } else {
        equit!("Cannot find a data directory for your current operating system.");
    }
}

fn init_folder_if_not_existent(path: &Path) {
    if !path.exists() {
        match create_dir_all(path) {
            Ok(_) => (),
            Err(_) => {
                equit!("Cannot create a directory inside {:?}", path.display());
            }
        }
    }
}

fn create_todo_file(path: &Path, name: &Option<String>) {
    let current_timestamp = chrono::Utc::now().format("%d-%m-%Y.%H:%M%S").to_string();
    let file_name = path.join(current_timestamp.clone());
    let todo_text = match name {
        Some(name) => edit::edit(name).expect("Unable to open text editor."),
        None => edit::edit("").expect("Unable to open text editor."),
    };

    let mut file: File = match File::create(file_name) {
        Ok(created_file) => created_file,
        Err(_) => {
            equit!(
                "Cannot create file {}. Please try again, or file a bug report.",
                path.join(current_timestamp).display(),
            );
        }
    };

    write!(file, "{}", todo_text);
}

fn list_all_todos() {}
