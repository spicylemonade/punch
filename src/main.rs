use std::fs;
use std::path::Path;

use clap::Parser;

mod in_directory;

#[derive(Debug, Parser)]
#[clap(trailing_var_arg = true)]
pub struct Args {
    /// to create file
    #[clap(value_parser, multiple_values = true)]
    target: Vec<String>,
    /// to delete
    #[clap(short, long, value_parser)]
    del: Option<Vec<String>>,
    /// creates files inside target directory
    #[clap(short, long, value_parser, multiple_values = true)]
    r#in: Option<Vec<String>>,
    /// deletes files inside target directory
    #[clap(long, value_parser, multiple_values = true)]
    din: Option<Vec<String>>,
    /// send the file to trash can
    #[clap(short, long, value_parser, multiple_values = true)]
    trash: Option<Vec<String>>,
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
        } else {
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
    Trash
}
struct Trash<'a>{
    trash_path: &'a Path
}

impl<'a> Trash<'a> {
    fn new(path: &'a Path) -> Self {
        Self {
            trash_path: path
        }
    }

    fn copy_recursively(&self, path: &Path) {
         let entries = fs::read_dir(path).expect("unable to parse directory");
        if path.is_dir(){
        
            fs::create_dir_all(Path::new(self.trash_path).join(path)).unwrap(); 
             
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            // if it is a directory we need to copy the things in the directory . so call again with the new path
                            self.copy_recursively(&path.join(entry.file_name()))
                        } else {
                            fs::copy(path.join(entry.file_name()) ,Path::new(self.trash_path).join(path.join(entry.file_name()))).unwrap();
                        }
                    }
                }
            }
        } else {
            fs::copy(path ,Path::new(self.trash_path).join(path)).unwrap();
        }
    }
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

    let trash_path = home_path.join(Path::new(".ptrash"));
    let trash = Trash::new(&trash_path);

    if !trash.trash_path.exists(){ // Path Does not Exists
        // Create the Directory
        fs::create_dir(trash.trash_path).expect(format!("error creating trash can").as_str())
    } 
    // Move files for directories to crash
    for i in 0..args.len(){
        let file = Path::new(&args[i]);
        
        trash.copy_recursively(file);
        if Path::new(file).is_dir() {
            //Iterate the directory and move it
             fs::remove_dir_all(file)
                .expect(format!("error removing directory: {:?}", file).as_str());
        } else { 
            fs::remove_file(file)
                .expect(format!("error removing directory: {:?}", file).as_str());
     
        }
    }
}
 

fn main() {
    let args = Args::parse();
    match args.input_type() {
        InputType::DeleteIn => in_directory::delete_files_dir(&args),
        InputType::CreateIn => in_directory::create_in_dir(&args),
        InputType::Del => delete_files(&args),
        InputType::Create => create_files(&args),
        InputType::Trash => trash_files(&args)
    }
}
