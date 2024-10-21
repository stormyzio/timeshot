use std::env::current_dir;
use std::fs::{create_dir, read_dir, File, OpenOptions};
use std::io::{read_to_string, Write};
use std::path::PathBuf;
use colored::Colorize;
use getch_rs::Getch;
use getch_rs::Key::{Char, Ctrl};
use crate::PATH_SEPARATOR;

pub fn snap(ignore: Option<Vec<String>>) {

    create_dir(".timeshot").unwrap_or_default(); // create the directory if it doesn't exist

    let save_path = File::open(".timeshot/save");

    if save_path.is_ok() {
        println!("{}", "Snapshot already exists!".red());
        eprint!("Do you want to continue and override the other save? (y/N): "); // using eprint, else the line would be printed after the prompt

        let g = Getch::new();

        loop {
            if let Ok(key) = g.getch() {
                match key {
                    Ctrl('c') | Char('n') | Char('N') => { // n, N or ctrl+c
                        println!("Aborted");
                        return; // Stop the function
                    }
                    Char('y') | Char('Y') | Char('\r') => { // y, Y or enter
                        println!("Accepted");
                        break; // Break the loop, continue with the function
                    }
                    _ => {}
                }
            }
        }
    } else {
        File::create(".timeshot/save").unwrap();
    }

    let mut save_content = String::new();

    let current_dir = current_dir().unwrap();
    save_content.push_str("\
-----------------------------------------------------------------------
> THIS IS AN AUTO-GENERATED FILE.
> DO NOT MODIFY IT, OR ELSE YOU MIGHT LOSE YOUR DATA WHEN USING TIMESHOT.
-----------------------------------------------------------------------


");
    check(current_dir.clone(), ignore.unwrap_or_else(|| vec![String::from("")]), &mut save_content); // start iterating the directory

    println!("Snapshot created.");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(".timeshot/save")
        .unwrap();

    file.write("".as_bytes()).unwrap();
    file.write_all(save_content.as_bytes()).unwrap();
}

fn check(path: PathBuf, ignore: Vec<String>, content: &mut String) {
    for entry in read_dir(path.clone()).unwrap() { // read the directory
        let path = entry.unwrap().path(); // get the path

        if path.is_dir() {
            let ignore_files = ignore.clone();
            let dir_name = path.file_name().unwrap().to_str().unwrap().to_string(); // get the directory name
            if ignore_files.contains(&dir_name) { // if the directory is in the ignore list, continue but skip the check()
                continue;
            }
            check(path.clone(), ignore_files, content); // else, check again the directory
        } else {
            let Ok(file) = File::open(&path) else { continue }; // open the file with a let-else syntax (opens the file, if it can't, continue)

            let file_name = path.file_name().unwrap().to_str().unwrap().to_string(); // get the file name
            if ignore.contains(&file_name) { // if the file is in the ignore list, continue
                continue;
            }

            let Ok(file_content) = read_to_string(file) else { continue }; // read the file content with a let-else syntax (reads the file, if it can't, continue)

            content.push_str(&*(PATH_SEPARATOR.to_owned() + "path:")); // add a separator for the path
            content.push_str(&path.to_str().unwrap().to_string()); // add the file path to the content
            content.push_str("\n"); // add a newline
            content.push_str(&file_content); // add the file content to the content
        }
    }
}

