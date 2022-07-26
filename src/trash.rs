use std::path::Path;
use std::fs;
use crate::error::PunchError;
use crate::punch;
use anyhow::Result;

pub struct Trash<'a>{
    pub trash_path: &'a Path,
}

impl<'a> Trash<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self {
            trash_path: path
        }
    }

    pub fn update(&self, path: &'a Path) -> Self {
        Self {
            trash_path: path
        }
    }
    // This is to ensure scenarios of punch -t folder/file.txt can be handled
    pub fn move_(&self, path: &Path) -> Result<()>{
        let file_name = path.file_name().unwrap();
        let trash_path = &Path::new(&self.trash_path).join(file_name);
      
        self.update(trash_path);
        self.move_to_trash(path)?; 
        Ok(())
    }
    pub fn move_to_trash(&self, path: &Path) -> Result<()>{
        if path.is_dir(){
            let entries = fs::read_dir(path).expect("unable to parse directory");
            fs::create_dir_all(Path::new(self.trash_path).join(path)).unwrap(); 
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            // if it is a directory we need to copy the things in the directory . so call again with the new path
                            if let Err(_) = self.move_to_trash(&path.join(entry.file_name())){}
                        } else {
                            let from = path.join(entry.file_name());
                            let to = Path::new(self.trash_path).join(path.join(entry.file_name()));
                            punch::move_file(&from, &to)?; 
                        }
                    }
                }
            }
        } else {
             
            let to = Path::new(self.trash_path).join(path); 
            punch::move_file(path, &to)?;
        }


        Ok(())
    }  
    
    pub fn remove_from_source(&self, path: &Path) ->Result<()>{
         if Path::new(path).is_dir() {
            //Iterate the directory and move it
             if let Err(_) = fs::remove_dir_all(path){
                return Err(PunchError::DeleteDirectoryError(path.display().to_string()).into()).into();
             }
        } else { 
             if let Err(_) = fs::remove_file(path){
                return Err(PunchError::DeleteFileError(path.display().to_string()).into()).into();
             }  
        }
        Ok(())
    }
}
