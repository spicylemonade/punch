use anyhow::Result;
use clap::Parser;

mod db;
mod error;
mod operations;
mod punch;
mod trash;

#[derive(Debug, Parser, Clone)]
#[clap(trailing_var_arg = true, version = "1.6.0")]
pub struct Args {
    /// To create file
    #[clap(value_parser, multiple_values = true)]
    target: Vec<String>,
    /// To delete
    #[clap(short, long, value_parser)]
    del: Option<Vec<String>>,
    /// Creates files inside target directory-first arguement is target
    #[clap(short, long, value_parser, multiple_values = true)]
    r#in: Option<Vec<String>>,
    /// Deletes files inside target directory-first arguement is target
    #[clap(long, value_parser, multiple_values = true)]
    din: Option<Vec<String>>,

    /// Sends the file to trash can
    #[clap(short, long, value_parser, multiple_values = true)]
    trash: Option<Vec<String>>,
    /// Undoes last create or trash
    #[clap(short, long)]
    undo: bool,
    /// Prints out creation,deletion, and trash history
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
    list: bool,

    /// Opens file with default application
    #[clap(short, long, value_parser, multiple_values = true)]
    open: Option<String>,

    /// Clears the trash Directory
    #[clap(long)]
    clear: bool,
    ///deletes specific file name from database
    #[clap(long, help_heading = "EXTRA")]
    db_delete: Option<String>,

    #[clap(long, help_heading = "EXTRA", multiple_values = true)]
    sizeof: Option<Vec<String>>,
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
        } else if let true = self.undo {
            return InputType::Undo;
        } else if let true = self.show {
            return InputType::Show;
        } else if let true = self.list {
            return InputType::List;
        } else if let Some(_) = self.mve {
            return InputType::Move;
        } else if let Some(ref args) = self.ren {
            assert!(args.len() == 2, "Expected 2 arguments got {}", args.len());
            return InputType::Rename;
        } else if let Some(_) = self.open {
            return InputType::Open;
        } else if let true = self.clear {
            return InputType::Clear;
        } else if let Some(_) = self.db_delete {
            return InputType::Dbdelete;
        } else if let Some(_) = self.sizeof {
            return InputType::Sizeof;
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
    Trash,
    Undo,
    Show,
    Rename,
    Move,
    List,
    Open,
    Clear,
    Dbdelete,
    Sizeof,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let current_dir = &std::env::current_dir().unwrap();
    match args.input_type() {
        //order matters for pushing to the database
        /*for files that result increating in current dir
        push to db after for files resulting in a deletion (trash,move,delete,deletein),
        push before*/
        InputType::DeleteIn => {
            operations::delete_files_dir(&args)?;
            db::push(&&args.din.clone().unwrap(), "DeleteIn", current_dir)?
        }
        InputType::CreateIn => {
            operations::create_in_dir(&args)?;
            db::push(&&args.r#in.clone().unwrap(), "CreateIn", current_dir)?
        }
        InputType::Del => {
            db::push(&&args.del.clone().unwrap(), "Delete", current_dir)?;
            operations::delete_files(&args)?
        }
        InputType::Create => {
            operations::create_files(&args)?;
            db::push(&&args.target, "Create", current_dir)?
        }

        InputType::Trash => {
            operations::trash_files(&args)?;
            db::push(&&args.trash.clone().unwrap(), "Trash", current_dir)?;
        }
        InputType::Undo => db::undo()?,
        InputType::Show => db::show()?,
        InputType::Rename => operations::rename_file(&args)?,
        InputType::List => operations::list_current_directory()?,
        InputType::Move => operations::move_file(&args)?,
        InputType::Open => operations::open_file(&args)?,
        InputType::Clear => operations::clear_trash()?,
        InputType::Dbdelete => db::delete(args.db_delete.unwrap())?,
        InputType::Sizeof => operations::sizeof(&args)?,
    }
    Ok(())
}
