//for testing features
use std::fs;
use std::env::current_dir;
use std::fs::rename;
use std::path::{Path, PathBuf};

mod args;

#[test]
fn create_files() {
    let arg_struct = args::Args::default();
    let args = arg_struct.target.clone();
    for i in 0..args.len() {
        if args[i].ends_with("/") {
            fs::create_dir_all(&args[i])
                .expect(format!("error creating folder: {}", args[i]).as_str());
        } else {
            fs::File::create(&args[i]).expect(format!("error creating file: {}", args[i]).as_str());
        }
    }
    fs::remove_dir_all("./test_dir/target_test/").unwrap();
    fs::remove_file("./test_dir/target_test.txt").unwrap();
}
#[test]
fn delete_files() {
    {
        fs::File::create("./test_dir/del_test.txt").unwrap();
        fs::create_dir_all("./test_dir/del_test/").unwrap();
    }
    let arg_struct = args::Args::default();
    let args = arg_struct.del.clone().unwrap();
    for i in 0..args.len() {
        if args[i].ends_with("/") {
            fs::remove_dir_all(&args[i])
                .expect(format!("error deleting folder: {}", args[i]).as_str());
        } else {
            fs::remove_file(&args[i]).expect(format!("error deleting file: {}", args[i]).as_str());
        }
    }
}

#[test]
fn rename_file() {
    {
        fs::File::create("./test_dir/rename_test.txt").unwrap();
    }
    let arg_struct = args::Args::default();
    let args = arg_struct.ren.clone().unwrap();
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

    fs::remove_file("./test_dir/rename_passed.txt").unwrap();
}

#[test]
fn move_file() {
    {
        fs::File::create("./test_dir/move_test.txt").unwrap();
        fs::create_dir_all("./test_dir/move_test/").unwrap();
    }
    let arg_struct = args::Args::default();
    let args = arg_struct.mve.clone().unwrap();

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
    fs::remove_dir_all("./test_dir/move_test/").unwrap();

}
