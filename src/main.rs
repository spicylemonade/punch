
use std::fs;
use std::path::Path;

use clap::{Parser};

mod db;
mod in_directory;
mod punch;
mod trash;

#[derive(Debug, Parser)]
#[clap(trailing_var_arg = true)]
pub struct Args {
    /// to create file
    #[clap(value_parser, multiple_values = true)]
    target: Vec<String>,
    /// to delete
    #[clap(short, long, value_parser)]
    del: Option<Vec<String>>,
    /// creates files inside target directory-first arguement is target
    #[clap(short, long, value_parser, multiple_values = true)]
    r#in: Option<Vec<String>>,
    /// deletes files inside target directory-first arguement is target
    #[clap(long, value_parser, multiple_values = true)]
    din: Option<Vec<String>>,

    /// send the file to trash can
    #[clap(short, long, value_parser, multiple_values = true)]
    trash: Option<Vec<String>>,

    #[clap(short,long)]
    undo: bool,

    #[clap(short,long)]
    show: bool,


}


impl Args {
    fn input_type(&self) -> InputType {
        
        if let Some(_) = self.din {
            return InputType::DeleteIn;
        } else if let Some(_) = self.del {
            return InputType::Del;
        } else if let Some(_) = self.r#in {
            return InputType::CreateIn;
        } else if let Some(_) = self.trash {
            return InputType::Trash;
        } else if self.target.len() > 0 {
            return InputType::Create;
        }else if let true = self.undo{
            return InputType::Undo;
        }else if let true = self.show{
            return InputType::Show;
        }  
        else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
enum InputType {
    DeleteIn,
    CreateIn,
    Del,
    Create,
    Trash,
    Undo,
    Show,
}


fn create_files(args: &Args) {
    let args = args.target.clone();
    for i in 0..args.len() {
        if args[i].contains("/") && args[i].ends_with("/") {
            fs::create_dir_all(&args[i])
                .expect(format!("error creating folder: {}", args[i]).as_str());
        } else {
            fs::File::create(&args[i]).expect(format!("error creating file: {}", args[i]).as_str());
        }
    }
}
fn delete_files(args: &Args) {
    let args = args.del.clone().unwrap();
    for i in 0..args.len() {
        if args[i].contains("/") && args[i].ends_with("/") {
            fs::remove_dir_all(&args[i])
                .expect(format!("error deleting folder: {}", args[i]).as_str());
        } else {
            fs::remove_file(&args[i]).expect(format!("error deleting file: {}", args[i]).as_str());
        }
    }
}

fn trash_files(args: &Args){
    let args = args.trash.clone().unwrap();
    // Check if the .ptrash/ directory exist in ~
    let home_path  = match  home::home_dir() {
        Some(path) => path,
        _ => panic!("Unable to trash files")
    };

    let trash_path = home_path.join(Path::new(".punch/trash"));
    let trash = trash::Trash::new(&trash_path);

    if !trash.trash_path.exists(){ // Path Does not Exists
        // Create the Directory
        fs::create_dir(trash.trash_path).expect(format!("error creating trash can").as_str())
    } 
    // Move files for directories to crash
    for i in 0..args.len(){
        let file = Path::new(&args[i]); 
        trash.move_to_trash(file); // First Part
        trash.remove_from_source(file); // Second Part
    }
}


fn main() {
    
    let args = Args::parse();

    match args.input_type() {
        InputType::DeleteIn => {
            in_directory::delete_files_dir(&args); 
            db::push(&&args.din.clone().unwrap(), "DeleteIn")
        },

        InputType::CreateIn => {
            in_directory::create_in_dir(&args); 
            db::push(&&args.r#in.clone().unwrap(), "CreateIn")
        },

        InputType::Del => {
            db::push(&&args.del.clone().unwrap(), "Delete");
            delete_files(&args); 
            },

        InputType::Create => {
            create_files(&args); 
            db::push(&&args.target, "Create")
        },

        InputType::Trash => { 
            trash_files(&args);
            db::push(&&args.trash.clone().unwrap(), "Trash");
        },

        InputType::Undo => { db::undo()},
      

        InputType::Show => { db::show()},
    }
}
