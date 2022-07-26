use std::fs;
use std::fs::rename;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::error::PunchError;
use crate::trash;
use crate::punch;
use crate::Args;


pub fn create_files(args: &Args) -> Result<()> {
    let args = args.target.clone();
    for i in 0..args.len() {
        if args[i].contains("/") && args[i].ends_with("/") {
            if let Err(_) = punch::create_directory(Path::new(&args[i])) {
                return Err(PunchError::CreateDirectoryError(args[i].clone()).into()).into();
            }
        } else {
            if let Err(_) =  punch::create_file(Path::new(&args[i])) {
                return Err(PunchError::CreateFileError(args[i].clone()).into()).into();
            }
        }
    }
    Ok(())
}
pub fn delete_files(args: &Args) ->Result<()>{
    let args = args.del.clone().unwrap();
    for i in 0..args.len() {
        if args[i].contains("/") && args[i].ends_with("/") {
            if let Err(_) = punch::remove_directory(Path::new(&args[i])) {
                return Err(PunchError::DeleteDirectoryError(args[i].clone()).into()).into();
            }  
        } else { 
            if let Err(_) = punch::remove_file(Path::new(&args[i])) {
                return Err(PunchError::DeleteFileError(args[i].clone()).into()).into();
            }
        }
    }
    Ok(())
}

pub fn rename_file(args: &Args) -> Result<()> {
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
    if let Err(_) = rename(source, to) {
        return Err(PunchError::RenameFileError.into()).into();
    }
    Ok(())
}

pub fn move_file(args: &Args) -> Result<()> {
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

            if let Err(_) = fs::File::create(Path::new(&back_str).join(&original_file.file_name().unwrap())) {
                return Err(PunchError::CreateFileError(args[0].clone()).into()).into();
            }

            if let Err(_) = fs::copy(original_file, Path::new(&back_str).join(&original_file.file_name().unwrap())){
                return Err(PunchError::CopyFileError(args[0].clone()).into()).into();
            }
            if let Err(_) = fs::remove_file(&original_file) {
                return Err(PunchError::DeleteFileError(args[0].clone()).into()).into();
            }
            }
        },
        Err(_) => {
            if !new_directory.is_dir() { 
                if let Err(_) = fs::create_dir_all(&new_directory) {
                    return Err(PunchError::CreateFileError(args[0].clone()).into()).into();
                }
                 
            }
            if original_file.exists(){
                if let Err(_) = fs::File::create(&new_directory.join(&original_file.file_name().unwrap())) {
                    return Err(PunchError::CreateFileError(args[0].clone()).into()).into();
                }
                
                if let Err(_) = fs::copy(&original_file, &new_directory.join(original_file.file_name().unwrap())) {
                    return Err(PunchError::CopyFileError(args[0].clone()).into()).into();
                }
                if let Err(_) = fs::remove_file(&original_file) {
                    return Err(PunchError::DeleteFileError(args[0].clone()).into()).into();
                } 
            }
        
        } 
    }
     Ok(())
}

pub fn list_current_directory() -> Result<()> {
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
    Ok(())
}


pub fn trash_files(args: &Args) -> Result<()>{
    let args = args.trash.clone().unwrap();
    // Check if the .ptrash/ directory exist in ~
    let home_path = match home::home_dir() {
        Some(path) => path,
        _ => return Err(PunchError::TrashNotFound.into()).into(),
    };

    let trash_path = home_path.join(Path::new(".punch/trash"));
    let trash = trash::Trash::new(&trash_path);

    if !trash_path.exists() {
        // Path Does not Exists
        // Create the Directory
        if let Err(_) = fs::create_dir(&trash_path) {
            return Err(PunchError::TrashFileError(args[0].clone()).into()).into();
        }
    }
    // Move files for directories to trash
    // TODO: check if the user has the appropriate permission to move the files
    for i in 0..args.len(){
        let file = Path::new(&args[i]); 
        //TODO: handle trashing files/directories in another directory e.g punch -t test/file1.txt -- This should remove the file
        trash.move_(file)?; // First Part
        trash.remove_from_source(file)?; // Second Part
    }
    Ok(())
}


pub fn create_in_dir(args: &Args) -> Result<()> {
    let args = args.r#in.clone().unwrap(); 
    for i in 1..args.len() {
        if args[i].contains("/") {
            if let Err(_) = punch::create_directory(Path::new(&format!("{}{}", args[0], args[i]))){
                return Err(PunchError::CreateDirectoryError(format!("{}{}", args[0], args[i])).into()).into();
            }

        } else {
            if let Err(_) = punch::create_file(Path::new(&format!("{}/{}", args[0], args[i]))) {
                 return Err(PunchError::CreateFileError(format!("{}/{}", args[0], args[i])).into()).into();
            }
        }
    }
    Ok(())

}

pub fn delete_files_dir(args: &Args) ->Result<()> {
    let args = args.din.clone().unwrap();
    for i in 1..args.len() {
        if args[i].contains("/") {
            if let Err(_) = punch::remove_directory(Path::new(&format!("{}{}", args[0], args[i]))){
                return Err(PunchError::DeleteDirectoryError(format!("{}{}", args[0], args[i])).into()).into();
            }
        } else {
            if let Err(_) = punch::remove_file(Path::new(&format!("{}/{}", args[0], args[i]))){
                 return Err(PunchError::DeleteFileError(format!("{}/{}", args[0], args[i])).into()).into();
            }
        }
    }
    Ok(())
}
