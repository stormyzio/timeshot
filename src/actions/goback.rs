use std::fs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{read_to_string, Write};
use std::path::PathBuf;
use colored::Colorize;
use crate::PATH_SEPARATOR;

pub fn goback() {
    let Ok(save_file) = File::open(".timeshot/save") else {
        no_save_file();
        return;
    };

    let Ok(content) = read_to_string(save_file) else {
        println!("{}", "Unexpected error reading save file".red());
        return;
    };

    let binding_sep = PATH_SEPARATOR.to_owned() + "path:";
    let mut files = content.split(&*binding_sep); // split the content by the separator
    files.next(); // skip the first element
    for file in files { // file is a string with the path and the content
        let mut file_split = file.splitn(2, "\n");
        if file_split.clone().count() != 2 { // if the file doesn't have 2 parts, skip it ( avoid bugs )
            continue;
        }
        let path = file_split.next().unwrap(); // get the path
        let content = file_split.next().unwrap(); // get the content

        let file_content_before = fs::read_to_string(path).unwrap_or_default();

        if file_content_before == content { continue; } // if the content is the same, skip the file

        let mut file = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
        {
            Ok(f) => f,
            Err(_) => {
                // create the parent directories if they don't exist
                create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
                // create the file if it doesn't exist anymore
                File::create(path).unwrap()
            }
        };

        file.write_all(content.as_bytes()).unwrap(); // write the content to the file
    }

    println!("{}", "Snapshot restored successfully!".green());
    println!("You can delete the snapshot using `timeshot reset`");
}

fn no_save_file() {
    println!("{} {}", "No save file found. Please create a snapshot first using".red(), "`timeshot create`");
}