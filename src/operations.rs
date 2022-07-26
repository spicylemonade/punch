use crate::error::PunchError;
use crate::punch;
use crate::trash;
use crate::Args;
use anyhow::{Ok, Result};
use fs_extra::dir;
use opener::open;
use std::env::current_dir;
use std::fs;
use std::fs::rename;
use std::path::{Path, PathBuf};

pub fn create_files(args: &Args) -> Result<()> {
    let args = args.target.clone();
    for i in 0..args.len() {
        if args[i].ends_with("/") && args[i].ends_with("/") {
            if let Err(_) = punch::create_directory(Path::new(&args[i])) {
                return Err(PunchError::CreateDirectoryError(args[i].clone()).into()).into();
            }
        } else {
            if let Err(_) = punch::create_file(Path::new(&args[i])) {
                return Err(PunchError::CreateFileError(args[i].clone()).into()).into();
            }
        }
    }
    Ok(())
}
pub fn delete_files(args: &Args) -> Result<()> {
    let args = args.del.clone().unwrap();
    for i in 0..args.len() {
        if args[i].ends_with("/") && args[i].ends_with("/") {
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

    let mut destination = String::from(&args[args.len() - 1]); // Getting the destination arg.
    let num_to_back = destination.parse::<i8>(); // If the final arg is a number we are going back.
    match num_to_back {
        std::result::Result::Ok(number) => {
            destination.clear();
            for _i in 0..number {
                destination.push_str("../"); // Depending on the number we go back x amount of times.
            }
        }
        Err(_) => {}
    }

    let destination = Path::new(&destination); // Convert our formatted destination into type Path.

    let mut files: Vec<String> = Vec::new(); // We create a vector of all files/args except for the last one which is the destination.
    for i in 0..args.len() - 1 {
        files.push(String::from(&args[i]));
    }

    for file in files {
        // For every file we have listed we make a new one in the new directory,
        let file = Path::new(&file); // copy the contents form the old to the new, and delete the old file.
        if file.exists() {
            if let Err(_) = fs::File::create(&destination.join(&file.file_name().unwrap())) {
                return Err(PunchError::CreateFileError(args[0].clone()).into()).into();
            }
            if let Err(_) = fs::copy(&file, &destination.join(file.file_name().unwrap())) {
                return Err(PunchError::CopyFileError(args[0].clone()).into()).into();
            }
            if let Err(_) = fs::remove_file(&file) {
                return Err(PunchError::DeleteFileError(args[0].clone()).into()).into();
            }
        }
    }

    Ok(())
}

pub fn list_current_directory() -> Result<()> {
    let current_dir = std::env::current_dir();
    let paths = fs::read_dir(current_dir.unwrap());
    let paths: Vec<Result<fs::DirEntry, std::io::Error>> = paths.unwrap().collect();

    for path in paths {
        let path = path.unwrap().file_name();
        let path = Path::new(path.to_str().unwrap());
        let mut information = String::new();

        if path.is_dir() {
            information.push_str(&format!("<DIRECTORY>"));
        } else {
            information.push_str(&format!("     <FILE>"));
        }
        if let Err(_) = dir::get_size(path) {
            println!("{} {}", information, path.to_str().unwrap());
        } else {
            println!(
                "{} {}-> {} Kb",
                information,
                path.to_str().unwrap(),
                dir::get_size(path)? as f64 / 1000.0
            );
        }
    }
    Ok(())
}

pub fn trash_files(args: &Args) -> Result<()> {
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
    for i in 0..args.len() {
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
            if let Err(_) = punch::create_directory(Path::new(&format!("{}{}", args[0], args[i]))) {
                return Err(
                    PunchError::CreateDirectoryError(format!("{}{}", args[0], args[i])).into(),
                )
                .into();
            }
        } else {
            if let Err(_) = punch::create_file(Path::new(&format!("{}/{}", args[0], args[i]))) {
                return Err(PunchError::CreateFileError(format!("{}/{}", args[0], args[i])).into())
                    .into();
            }
        }
    }
    Ok(())
}

pub fn delete_files_dir(args: &Args) -> Result<()> {
    let args = args.din.clone().unwrap();
    for i in 1..args.len() {
        if args[i].contains("/") {
            if let Err(_) = punch::remove_directory(Path::new(&format!("{}{}", args[0], args[i]))) {
                return Err(
                    PunchError::DeleteDirectoryError(format!("{}{}", args[0], args[i])).into(),
                )
                .into();
            }
        } else {
            if let Err(_) = punch::remove_file(Path::new(&format!("{}/{}", args[0], args[i]))) {
                return Err(PunchError::DeleteFileError(format!("{}/{}", args[0], args[i])).into())
                    .into();
            }
        }
    }
    Ok(())
}

pub fn sizeof(args: &Args) -> Result<()> {
    let args = args.sizeof.clone().unwrap();
    for i in 0..args.len() {
        print!("{}: ", &args[i]);
        if let Err(_) = fs::metadata(&args[i]) {
            return Err(PunchError::ReadFileError(String::from(&args[i])).into()).into();
        } else {
            println!("{:#} Kb", dir::get_size(&args[i])? as f64 / 1000.0);
            if dir::get_size(&args[i])? as f64 / 1000.0 > 1000000.0 {
                println!(
                    "-> {:#} Gb",
                    (dir::get_size(&args[i])? as f64 / 1000.0) / 1000000.0
                );
            }
        }
    }

    Ok(())
}

pub fn clear_trash() -> Result<()> {
    let trash_dir = home::home_dir().unwrap().join(".punch/trash/");
    for entry in std::fs::read_dir(trash_dir)? {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Err(_) = punch::remove_directory(entry.path().as_path()) {
                return Err(PunchError::CreateFileError(entry.path().display().to_string()).into())
                    .into();
            }
        } else {
            if let Err(_) = punch::remove_file(entry.path().as_path()) {
                return Err(PunchError::CreateFileError(entry.path().display().to_string()).into())
                    .into();
            }
        }
    }
    Ok(())
}

#[inline(always)]
pub fn open_file(args: &Args) -> Result<()> {
    open(Path::new(args.open.as_ref().unwrap()))?;
    Ok(())
}
