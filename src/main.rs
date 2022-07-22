use std::fs;

use clap::Parser;

mod in_directory;

#[derive(Debug, Parser)]
#[clap(trailing_var_arg = true)]
pub struct Args {
    /// to create file
    #[clap(value_parser, multiple_values = true)]
    target: Vec<String>,
    /// to delete
    #[clap(short, long, value_parser, multiple_values = true)]
    del: Option<Vec<String>>,
    /// creates files inside target directory
    #[clap(short, long, value_parser, multiple_values = true)]
    r#in: Option<Vec<String>>,
    /// deletes files inside target directory
    #[clap(long, value_parser, multiple_values = true)]
    din: Option<Vec<String>>,
}

impl Args {
    fn input_type(&self) -> InputType {
        if let Some(_) = self.din {
            return InputType::DeleteIn;
        } else if let Some(_) = self.del {
            return InputType::Del;
        } else if let Some(_) = self.r#in {
            return InputType::CreateIn;
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

fn main() {
    let args = Args::parse();
    match args.input_type() {
        InputType::DeleteIn => in_directory::delete_files_dir(&args),
        InputType::CreateIn => in_directory::create_in_dir(&args),
        InputType::Del => delete_files(&args),
        InputType::Create => create_files(&args),
    }
}
