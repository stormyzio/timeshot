mod snapshot;
mod goback;

use std::fs::{remove_file, File};
use std::io::{read_to_string};
use colored::Colorize;

pub fn info() {
    print!("\x1B[2J\x1B[1;1H"); // clear console

    println!("{}", "Timeshot is a simple CLI tool to quickly go back in your code.".bright_blue());
    println!("{}", " > It's useful when you're trying to find a solution to a bug or refactor your code but you're not sure if the changes will work.".italic());

    println!(" ");

    println!("{}", "Commands:".bold());
    println!("{}", "- `timeshot create`: Creates a new snapshot for your directory.");
    println!("{}", "  > only one snapshot can be created at a time.".italic().bright_black());
    println!("{}", "- `timeshot back`: Goes back to your snapshot.");
    println!("{}", "- `timeshot reset`: Delete your snapshot.");

    println!(" ");

    println!("{}", ".tsignore:".bold());
    println!("{}", "- You can create a `.tsignore` file in your project to ignore directories from being snapshotted.");
    println!("{}", "  > Each line in the file should be a directory name or a file to ignore.".italic().bright_black());
}

pub fn create() {
    let ignore_file = File::open(".tsignore");
    if ignore_file.is_ok() {
        let content = read_to_string(ignore_file.unwrap()).unwrap(); // read the content of the ignore file

        let mut dirs: Vec<String> = content.split("\n").map(|s| -> String { // split the content line by line, and map to convert it to a String and remove the end slash
            let mut s_str = String::from(s);

            if s_str.starts_with("#") { // ignore comments
                return "".to_string();
            }
            if s_str.ends_with("/") { // remove the end slash if it exists
                s_str.pop();
            }

            s_str
        }).collect(); // transform the iterator into a vector

        dirs.push(".timeshot".to_string()); // always add the .timeshot directory to the ignore list
        dirs.push(".git".to_string()); // always add the .git directory to the ignore list
        dirs.push(".vscode".to_string()); // always add the .git directory to the ignore list
        dirs.push(".idea".to_string()); // always add the .git directory to the ignore list
        snapshot::snap(Some(dirs));
        return;
    }

    snapshot::snap(None);
}

pub fn back() {
    goback::goback();
}

pub fn reset() {
    let save_file = File::open(".timeshot/save");
    if save_file.is_ok() {
        remove_file(".timeshot/save").unwrap();
        println!("{}", "Snapshot deleted".green());
    } else {
        println!("{}", "No snapshot found".red());
    }
}









