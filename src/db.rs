use chrono::prelude::*;
use anyhow::Result;
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use tabled::{Table, Tabled};

use crate::punch;
macro_rules! db_connect {
    ($db_file:expr, conn) => {{
        let home_path = match home::home_dir() {
            Some(path) => path,
            _ => panic!("Unable to trash files"),
        };
        Connection::open(home_path.join(Path::new($db_file))).unwrap()
    }};
    ($db_file:expr, pull) => {{
        let home_path = match home::home_dir() {
            Some(path) => path,
            _ => panic!("Unable to trash files"),
        };
        let conn = Connection::open(home_path.join(Path::new($db_file))).unwrap();

        let mut stmt = conn
            .prepare("SELECT name, time, date, path, action FROM files")
            .unwrap();

        let x = stmt
            .query_map([], |row| {
                Ok(Files {
                    _name: row.get(0)?,
                    _time: row.get(1)?,
                    _date: row.get(2)?,
                    _path: row.get(3)?,
                    _action: row.get(4)?,
                })
            })
            .unwrap();

        let mut table_vector: Vec<Files> = Vec::new();
        for file in x {
            table_vector.push(file.unwrap() as Files);
        }

        table_vector
    }};
}

use crate::error::PunchError;
//database struct
#[derive(Debug, Tabled)]
struct Files {
    _name: String,
    _time: String,
    _date: String,
    _path: String,
    _action: String,
}

pub fn push(paths: &Vec<String>, action: &str, current_dir: &Path) -> Result<()> {

    let _home_path = match home::home_dir() {
        Some(path) => path,
        _ => return Err(PunchError::TrashCanError.into()).into(),
    }; 
    //accessing current date & time
    let conn = db_connect!(".punch/punch.db", conn);

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
    )
    .unwrap();

    //deletes items from table if date is less than current(table resets every day to save space)
    conn.execute("DELETE FROM files where date < (?1)", [&date])
        .unwrap();

    for i in 0..paths.len() {
        conn.execute(
            "INSERT INTO files (name, time, date, path, action) values (?1, ?2, ?3, ?4, ?5)",
            &[
                &paths[i],
                &time,
                &date,
                current_dir.to_str().unwrap(), //retrns full path of object
                action,
            ],
        )
        .expect("sql query failed");
    }
    Ok(())
}
//prints db to screen
pub fn show() -> Result<()> {
    let table_vector = db_connect!(".punch/punch.db", pull);

    println!("{}", Table::new(table_vector).to_string());
    Ok(())
}

//if the action preformed on the file was "Trash"
fn u_trash(name: &Path, path: &Path) -> Result<()> {
    //systems home dir
    //check if trashed file is a directory
    let home_path = match home::home_dir() {
        Some(path) => path,
        _ => panic!("Unable to trash files"),
    };
    let trash_file = home_path.join(".punch/trash/").join(name);

    if trash_file.is_dir() {
        let entries = fs::read_dir(home_path.join(".punch/trash/").join(name))
            .expect("unable to parse directory");

        if let Err(_) = punch::create_directory(path) {
             return Err(PunchError::CreateDirectoryError(path.display().to_string()).into()).into()
        }

        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().ok().unwrap();
                if file_type.is_dir() {
                    u_trash(&name.join(entry.file_name()), &path.join(entry.file_name()))?
                } else {
                    let from = &home_path
                            .join(".punch/trash/")
                            .join(&name.join(entry.file_name()));
                    let to = &path.join(entry.file_name());
                   if let Err(_) = punch::move_file(from, to){
                         return Err(PunchError::MoveFielError(from.display().to_string(), to.display().to_string() ).into()).into();
                    }
                }
            }
        }
    } else {
        let from = &home_path.join(".punch/trash/").join(name);
        let to = &path.join(name);
        if let Err(_) = punch::move_file(from, to){
             return Err(PunchError::MoveFielError(from.display().to_string(), to.display().to_string() ).into()).into();
        }
    }
    //delete file in trash after

    if trash_file.is_dir() {
        if let Err(_) = punch::remove_directory(&trash_file) {
                return Err(PunchError::DeleteDirectoryError(trash_file.display().to_string()).into()).into()
            }  
        ;
    } else {
        if let Err(_) = punch::remove_file(&trash_file) { 
            return Err(PunchError::DeleteFileError(trash_file.display().to_string()).into()).into()
        } 
    }
    Ok(())
}
//if the action preformed on the file was "Create"
fn u_create(name: &Path, path: &Path) -> Result<()> {
    if (path.join(name)).is_dir() {
        if let Err(_) = punch::remove_directory(path.join(name).as_path()){
             return Err(PunchError::DeleteDirectoryError(path.join(name).display().to_string()).into()).into();
        }
    } else {
         if let Err(_) =  punch::remove_file(path.join(name).as_path()){
             return Err(PunchError::DeleteFileError(path.join(name).display().to_string()).into()).into();
         }
    }

    Ok(())
}
pub fn undo() -> Result<()> {
    let file_iter = db_connect!(".punch/punch.db", pull);

    let latest_file = file_iter.last().unwrap();

    if latest_file._action == "Create" {
        u_create(Path::new(&latest_file._name), Path::new(&latest_file._path))?;
    } else if latest_file._action == "Trash" {
        u_trash(Path::new(&latest_file._name), Path::new(&latest_file._path))?;
    }
    Ok(())
}

pub fn delete(name: String) -> Result<()> {
    let conn = db_connect!(".punch/punch.db", conn);
    conn.execute("DELETE FROM files WHERE name=(?1)", [&name])
        .unwrap();
        Ok(())
}
