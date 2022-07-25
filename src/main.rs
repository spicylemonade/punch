use clap::Parser;

mod db; 
mod punch;
mod trash;
mod operations;

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

    /// undoes the last create or trash
    #[clap(short, long)]
    undo: bool,
    
    /// shows command history
    #[clap(short, long)]
    show: bool,
    
    /// Renam a file
    #[clap(short, long, value_parser, multiple_values = true)]
    ren: Option<Vec<String>>,

    /// Move a file from one directory to another.
    #[clap(short, long, multiple_values = true)]
    mve: Option<Vec<String>>,

    /// Lists the files and directories in the current working directory.
    #[clap(short, long)]
    list: bool
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
        } else if let Some(_) = self.mve {
            return InputType::Move;
        } else if self.target.len() > 0 {
            return InputType::Create;
        } else if let true = self.undo {
            return InputType::Undo;
        } else if let true = self.show {
            return InputType::Show;
        } else if let true = self.list {
            return InputType::List;
        } else if let Some(ref args) = self.ren {
            assert!(args.len() == 2, "Expected 2 arguments got {}", args.len());
            return InputType::Rename;
        }else {
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
    Rename,
    Move,
    List
}


fn main() {
    let args = Args::parse();

    match args.input_type() {
        //order matters for pushing to the database
        /*for files that result increating in current dir
        push to db after for files resulting in a deletion (trash,move,delete,deletein),
        push before*/
InputType::DeleteIn => {
            operations::delete_files_dir(&args); 
            db::push(&&args.din.clone().unwrap(), "DeleteIn")
        },

        InputType::CreateIn => {
            operations::create_in_dir(&args); 
            db::push(&&args.r#in.clone().unwrap(), "CreateIn")
        },

        InputType::Del => {
            db::push(&&args.del.clone().unwrap(), "Delete");
            operations::delete_files(&args); 
            },

        InputType::Create => {
            operations::create_files(&args); 
            db::push(&&args.target, "Create")
        },

        InputType::Trash => { 
            operations::trash_files(&args);
            db::push(&&args.trash.clone().unwrap(), "Trash");
        },

        InputType::Undo => { db::undo()},

        InputType::Show => { db::show()},
        
        InputType::Rename => operations::rename_file(&args),

        InputType::Move => {
            match &&args.mve.clone().unwrap()[1].parse::<i32>() {
               Ok(_) => (),
               Err(_) => db::push(&&args.mve.clone().unwrap(), "Move"), 
            }  
            operations::move_file(&args)
        },

        InputType::List => { operations::list_current_directory() }
    }
}
