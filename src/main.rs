#[macro_use]
mod errors;
mod args;

use args::Args;
use chrono;
use directories::ProjectDirs;
use std::fs::{create_dir_all, read_dir, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

const MAX_FILENAME_LENGTH: usize = 15;

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
    let current_timestamp = chrono::Utc::now().format("%d-%m-%Y.%H:%M:%S").to_string();
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

    match write!(file, "{}", todo_text) {
        Ok(_value) => (),
        Err(error) => panic!("{}", error),
    }
}

fn list_all_todos() {
    if let Some(data_directory) = ProjectDirs::from("", "", "Todo Rust") {
        let directory = data_directory.config_dir();
        let data_children_paths = read_dir(directory).unwrap();

        if read_dir(directory).unwrap().count() == 0 {
            println!("There are no saved todos.");
        } else {
            for (index, path) in data_children_paths.enumerate() {
                let file_name = path
                    .expect("IO error when iterating paths inside the data directory.")
                    .file_name()
                    .into_string()
                    .expect("Failed to parse file name to string.");

                let mut full_path_name: String =
                    String::from(directory.as_os_str().to_str().unwrap());
                full_path_name.push_str("/");
                full_path_name.push_str(&file_name);

                let file = File::open(&full_path_name).unwrap();
                let mut lines = BufReader::new(file).lines();

                let first_line = lines
                    .nth(0)
                    .unwrap()
                    .expect("Cannot read first line of file.");

                let mut truncated_first_line = first_line.clone();

                // Truncate the first line if it has any content
                if first_line != "" {
                    truncated_first_line.truncate(MAX_FILENAME_LENGTH);
                    if first_line.len() > MAX_FILENAME_LENGTH {
                        truncated_first_line.push_str("...");
                    }
                }

                println!("{}. {} - {}", index, file_name, truncated_first_line);
            }
        }
    } else {
        equit!("Cannot find a data directory for your current operating system.");
    }
}
