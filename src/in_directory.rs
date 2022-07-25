use std::path::Path;
use crate::Args;
use crate::punch;

pub fn create_in_dir(args: &Args) {
    let args = args.r#in.clone().unwrap(); 
    for i in 1..args.len() {
        if args[i].contains("/") {
            punch::create_directory(Path::new(&format!("{}{}", args[0], args[i])));

        } else {
            punch::create_file(Path::new(&format!("{}/{}", args[0], args[i])));
        }
    }
}

pub fn delete_files_dir(args: &Args) {
    let args = args.din.clone().unwrap();
    for i in 1..args.len() {
        if args[i].contains("/") {
            punch::remove_directory(Path::new(&format!("{}{}", args[0], args[i])));  
        } else {
            punch::remove_file(Path::new(&format!("{}/{}", args[0], args[i])));
        }
    }
}