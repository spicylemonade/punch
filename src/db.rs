use std::{path::Path, fs};

use rusqlite::{Connection};
use chrono::prelude::*;
use anyhow::Result;

use crate::error::PunchError;
//database struct
#[derive(Debug)]
struct Files{
    _name: String,
    _time: String,
    _date: String,
    _path: String,
    _action: String

}

pub fn push(paths: &Vec<String>, action: &str) -> Result<()> {

    let home_path = match home::home_dir() {
        Some(path) => path,
        _ => panic!("Unable to trash files"),
    };
    let conn: Connection = Connection::open(home_path.join(Path::new(".punch/punch.db"))).unwrap();
    //accessing current date & time
    let time = Local::now().format("%H:%M").to_string();
    let date = Local::now().format("%Y-%m-%d").to_string();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            name    TEXT,
            time    TEXT,
            date    TEXT,
            path    TEXT,
            action  TEXT
        )",
        (), 
    ).unwrap();

    //deletes items from table if date is less than current(table resets every day to save space)
    conn.execute(
        "DELETE FROM files where date < (?1)",
        [&date], 
    ).unwrap();


    for i in 0..paths.len(){
        conn.execute(
            "INSERT INTO files (name, time, date, path, action) values (?1, ?2, ?3, ?4, ?5)", 
        &[
            &paths[i],
            &time,
            &date,
            fs::canonicalize(&paths[i]).ok().unwrap().to_str().unwrap(), //retrns full path of object
            &action]
        ).expect("sql query failed");
    } 
    Ok(())

}
//prints db to screen
pub fn show() -> Result<()>{
    let home_path = match home::home_dir() {
        Some(path) => path,
        _ => panic!("Unable to trash files"),
    };
    let conn: Connection = Connection::open(home_path.join(Path::new(".punch/punch.db"))).unwrap();
    
    let mut stmt = conn.prepare("SELECT name, time, date, path, action FROM files").unwrap();
    let file_iter = stmt.query_map([], |row| {
        Ok(Files {
                _name: row.get(0)?,
                _time: row.get(1)?,
                _date: row.get(2)?,
                _path: row.get(3)?,
                _action: row.get(4)?
              })
        }).unwrap();
    for file in file_iter {
        println!("{:#?}", file.unwrap() as Files);
    }
    Ok(())
}

//if the action preformed on the file was "Trash"
fn u_trash(name: &Path, path: &Path) -> Result<()>{
    //systems home dir
    let home_path  = match  home::home_dir() {
    Some(path) => path,
    _ => panic!("Unable to trash files")
    };
    //check if trashed file is a directory
    if home_path.join(".ptrash").join(name).is_dir(){

        let entries = fs::read_dir(home_path.join(".ptrash").join(name)).expect("unable to parse directory");

        fs::create_dir_all(path).unwrap(); 
        
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        // if it is a directory we need to copy the things in the directory . so call again with the new path
                        if let Err(_) = u_trash(&name.join(entry.file_name()), &path.join(entry.file_name())) {
                             
                        }
                    } else {

                        fs::copy(home_path.join(".ptrash").join(&name.join(entry.file_name())) ,path.join(entry.file_name())).unwrap();
                    }
                }
            }
        } 
  } else {
      if let Err(_) = fs::copy(home_path.join(".ptrash").join(name) , path) {
        return Err(PunchError::CopyFileError(name.display().to_string()).into()).into();
      }
  }
  Ok(())
}
//if the action preformed on the file was "Create"
fn u_create(path: &String) -> Result<()>{
    if (Path::new(path)).is_dir() {
        if let Err(_) = fs::remove_dir_all(path) {
            return Err(PunchError::DeleteDirectoryError(path.to_string()).into()).into();
        } 
    } else {
        if let Err(_) = fs::remove_file(path) {
            return Err(PunchError::DeleteFileError(path.to_string()).into()).into();
        }  
    }

    Ok(())
}
pub fn undo() -> Result<()>{
    let home_path = match home::home_dir() {
        Some(path) => path,
        _ => return Err(PunchError::TrashCanError.into()).into(),
    };
    let conn: Connection = Connection::open(home_path.join(Path::new(".punch/punch.db"))).unwrap();
    
    let mut stmt = conn.prepare("SELECT name, time, date, path, action FROM files").unwrap();
    let file_iter = stmt.query_map([], |row| {
        Ok(Files {
                _name: row.get(0)?,
                _time: row.get(1)?,
                _date: row.get(2)?,
                _path: row.get(3)?,
                _action: row.get(4)?
              })
        }).unwrap();
    let latest_file = file_iter.last().unwrap().ok().unwrap();

    if latest_file._action == "Create"{
        u_create(&latest_file._path)?;

    }
    else if latest_file._action == "Trash"{
        u_trash(Path::new(&latest_file._name),Path::new(&latest_file._path))?;
        //delete file in trash after
        let home_path  = match  home::home_dir() {
            Some(path) => path,
            _ => return Err(PunchError::TrashCanError.into()).into(),
        };
        let trash_file = home_path.join(".ptrash").join(&latest_file._name);
        if trash_file.is_dir() {
            if let Err(_) = fs::remove_dir_all(&trash_file){
                return Err(PunchError::DeleteDirectoryError(trash_file.display().to_string()).into()).into()
            }  
        } else {
            if let Err(_) = fs::remove_file(&trash_file) { 
                return Err(PunchError::DeleteFileError(trash_file.display().to_string()).into()).into()
            } 
        }
    }
    Ok(())
}