mod actions;

use std::env::args;
use colored::Colorize;
use actions::{info, back, create, reset};

const PATH_SEPARATOR: &str = "<_@@_/-,+?/+@,$-/€>*=>_?</@?=->>-€#??_€=€_=+-;-;€%@;@?+_?€?=/?<?_";

fn main() {
    let action_binding = args().nth(1).unwrap_or_else(|| String::from("info"));
    let action = action_binding.as_str();

    if action == "info" { info(); }
    else if action == "create" { create(); }
    else if action == "reset" { reset(); }
    else if action == "back" { back(); }
    else {
        println!("{} {}",
                 "Invalid action:".red(),
                 action.red().bold()
        );
    }
}


