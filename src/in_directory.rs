use std::fs;

use crate::Args;

pub fn create_in_dir(args: &Args) {
    let args = args.r#in.clone().unwrap();
    println!("{:?}", args);
    for i in 1..args.len() {
        if args[i].contains("/") {
            fs::create_dir_all(format!("{}{}", args[0], args[i]))
                .expect(format!("error creating folder: {}", args[i]).as_str());
        } else {
            fs::File::create(format!("{}/{}", args[0], args[i]))
                .expect(format!("error creating file: {}", args[i]).as_str());
        }
    }
}

pub fn delete_files_dir(args: &Args) {
    let args = args.din.clone().unwrap();
    for i in 1..args.len() {
        if args[i].contains("/") {
            fs::remove_dir_all(format!("{}{}", args[0], args[i]))
                .expect(format!("error creating folder: {}", args[i]).as_str());
        } else {
            fs::remove_file(format!("{}/{}", args[0], args[i]))
                .expect(format!("error creating folder: {}", args[i]).as_str());
        }
    }
}