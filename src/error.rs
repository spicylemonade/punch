use thiserror::Error;

#[derive(Error, Debug)]
pub enum PunchError {
    #[error("Unable to create file {0}")]
    CreateFileError(String),
    #[error("Unable to create directory {0}")]
    CreateDirectoryError(String),
    #[error("Unable to delete file {0}")]
    DeleteFileError(String),
    #[error("Unable to delete directory {0}")]
    DeleteDirectoryError(String),
    #[error("Unable to rename file")]
    RenameFileError,
    #[error("Unable to copy file {0}")]
    CopyFileError(String),
    #[error("Unable to Trash file {0}")]
    TrashFileError(String),
    #[error("Unable to Create Trash Can")]
    TrashCanError,
    #[error("Can not find trash can in default path")]
    TrashNotFound,
    #[error("Can not move file from {0} to {1}")]
    MoveFielError(String, String),
    #[error("Failed to read file {0}")]
    ReadFileError(String),
    #[error("Some Db Error Occured")]
    _DbError,
}
