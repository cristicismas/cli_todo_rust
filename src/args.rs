// path decorator is included so importing from a sibling mod is possible.
// #[path = "errors.rs"]
// mod errors;

use std::process::exit;

const MAX_COMMANDS_ALLOWED: usize = 3;

fn get_nth_arg(n: usize) -> Option<String> {
  std::env::args().nth(n)
}

#[derive(Debug)]
pub struct Args {
  pub command: String,
  pub name: Option<String>,
}

impl Args {
  pub fn new() -> Self {
    if std::env::args().len() > MAX_COMMANDS_ALLOWED {
      eprintln!(
        "Too many arguments. Maximum number of arguments is {}",
        MAX_COMMANDS_ALLOWED - 1,
      );
      exit(1);
    }

    Args {
      command: match get_nth_arg(1) {
        Some(value) => value,
        None => {
          eprintln!("At least one argument is required. Use `todo help` to display usage info.");
          exit(1);
        }
      },
      name: get_nth_arg(2),
    }
  }
}