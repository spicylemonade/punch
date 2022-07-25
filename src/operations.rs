use std::fs;
use std::fs::rename;
use std::env::current_dir;
use std::path::{Path, PathBuf};


use crate::trash;
use crate::punch;
use crate::Args;

pub fn create_files(args: &Args) {
    let args = args.target.clone();
    for i in 0..args.len() {
        if args[i].contains("/") && args[i].ends_with("/") {
            punch::create_directory(Path::new(&args[i]));
        } else {
            punch::create_file(Path::new(&args[i]));
        }
    }
}
pub fn delete_files(args: &Args) {
    let args = args.del.clone().unwrap();
    for i in 0..args.len() {
        if args[i].contains("/") && args[i].ends_with("/") {
            punch::remove_directory(Path::new(&args[i])); 
        } else {
            punch::remove_file(Path::new(&args[i])); 
        }
    }
}

pub fn rename_file(args: &Args) {
    let args = args.ren.clone().unwrap();
    let mut source;
    if args[0].clone().starts_with('.') {
        source = current_dir().unwrap()
    } else {
        source = PathBuf::new();
    }
    let mut buf = PathBuf::new();
    args[0].clone().split('/').for_each(|path| {
        if path != "." {
            buf.push(path)
        }
    });
    source = source.join(buf);
    let mut to;
    if args[1].clone().starts_with('.') {
        to = current_dir().unwrap()
    } else {
        to = PathBuf::new();
    }
    let mut buf = PathBuf::new();
    args[1].clone().split('/').for_each(|path| {
        if path != "." {
            buf.push(path)
        }
    });
    to = to.join(buf);
    rename(source, to).expect("Unable to rename");
}

pub fn move_file(args: &Args) {
    let args = args.mve.clone().unwrap();

    let original_file = Path::new(&args[0]);
    let new_directory = Path::new(&args[1]);

    //number of directories to go back
    let num_to_back = new_directory.to_str().unwrap().parse::<i8>();

    //if second input is a number
    match num_to_back {
        Ok(number) => {
            let mut back_str = String::new();
            //go back a directory for number of times
            for _i in 0..number {
                back_str.push_str("../");
            }
            
            if original_file.exists() {

                fs::File::create(Path::new(&back_str).join(&original_file.file_name().unwrap()))
                    .expect(format!("Failed to create new file: {}", original_file.display()).as_str());

                fs::copy(original_file, Path::new(&back_str).join(&original_file.file_name().unwrap()))
                    .expect(format!("Failed to copy file contents: {}", original_file.display()).as_str());

                fs::remove_file(&original_file)
                    .expect(format!("Failed to delete old file: {}", original_file.display()).as_str());
            }
        },
        Err(_) => {
            if !new_directory.is_dir() {
                println!("Destination directory does not exist, creating new folder.");
                fs::create_dir_all(&new_directory)
                    .expect(format!("Failed to create new directory: ./{}/", new_directory.display()).as_str());
            }
            if original_file.exists(){

                fs::File::create(&new_directory.join(&original_file.file_name().unwrap()))
                    .expect(format!("Failed to create new file: {}", original_file.display()).as_str());

                fs::copy(&original_file, &new_directory.join(original_file.file_name().unwrap()))
                    .expect(format!("Failed to copy file contents: {}", original_file.display()).as_str());

                fs::remove_file(&original_file)
                    .expect(format!("Failed to delete old file: {}", original_file.display()).as_str());
            }
        
        } 
    }
}

pub fn list_current_directory() {
    let current_dir = std::env::current_dir();
    let paths = fs::read_dir(current_dir.unwrap());
    let paths : Vec<Result<fs::DirEntry, std::io::Error>> = paths.unwrap().collect();

    for path in paths {
        let path = path.unwrap().file_name();
        let path = Path::new(path.to_str().unwrap());
        let mut information = String::new();
        if path.is_dir() {
            information.push_str(&format!("<DIRECTORY>"));
        } else {
            information.push_str(&format!("     <FILE>"));
        }
        println!("{} {}", information, path.to_str().unwrap());
    }
}


pub fn trash_files(args: &Args) {
    let args = args.trash.clone().unwrap();
    // Check if the .ptrash/ directory exist in ~
    let home_path = match home::home_dir() {
        Some(path) => path,
        _ => panic!("Unable to trash files"),
    };

    let trash_path = home_path.join(Path::new(".punch/trash"));
    let trash = trash::Trash::new(&trash_path);

    if !trash_path.exists() {
        // Path Does not Exists
        // Create the Directory
        fs::create_dir(&trash_path).expect(format!("error creating trash can").as_str())
    }
    // Move files for directories to trash
    // TODO: check if the user has the appropriate permission to move the files
    for i in 0..args.len(){
        let file = Path::new(&args[i]); 
        //TODO: handle trashing files/directories in another directory e.g punch -t test/file1.txt -- This should remove the file
        trash.move_(file); // First Part
        trash.remove_from_source(file); // Second Part
    }
}


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
