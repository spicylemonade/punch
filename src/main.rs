use anyhow::Result;
use clap::{CommandFactory, Parser};
use error::PunchError;

mod db;
mod error;
mod operations;
mod punch;
mod trash;

#[derive(Debug, Parser, Clone)]
#[clap(trailing_var_arg = true, version = "1.7.5")]
pub struct Args {
    /// To create file or directory
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
    ///returns size of file or directory
    #[clap(long, help_heading = "EXTRA", multiple_values = true)]
    sizeof: Option<Vec<String>>,
}

impl Args {
    fn input_type(&self) -> Result<InputType, error::PunchError> {
        if let Some(_) = self.din {
            return Ok(InputType::DeleteIn);
        } else if let Some(_) = self.del {
            return Ok(InputType::Del);
        } else if let Some(_) = self.r#in {
            return Ok(InputType::CreateIn);
        } else if let Some(_) = self.trash {
            return Ok(InputType::Trash);
        } else if self.target.len() > 0 {
            return Ok(InputType::Create);
        } else if let true = self.undo {
            return Ok(InputType::Undo);
        } else if let true = self.show {
            return Ok(InputType::Show);
        } else if let true = self.list {
            return Ok(InputType::List);
        } else if let Some(_) = self.mve {
            return Ok(InputType::Move);
        } else if let Some(ref args) = self.ren {
            assert!(args.len() == 2, "Expected 2 arguments got {}", args.len());
            return Ok(InputType::Rename);
        } else if let Some(_) = self.open {
            return Ok(InputType::Open);
        } else if let true = self.clear {
            return Ok(InputType::Clear);
        } else if let Some(_) = self.db_delete {
            return Ok(InputType::Dbdelete);
        } else if let Some(_) = self.sizeof {
            return Ok(InputType::Sizeof);
        } else {
            Err(PunchError::CliInvalidInputError)
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
        Ok(input_type) => match input_type {
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
        },
        Err(e) => {
            eprintln!("Error: {e}");
            let mut cmd = Args::command();
            let _ = cmd.print_help();
            std::process::exit(1);
        }
    }
    Ok(())
}
