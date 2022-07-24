use std::{path::Path, fs};

use rusqlite::{Connection};
use chrono::prelude::*;
use shellexpand::tilde;

//database struct
#[derive(Debug)]
struct Files{
    _name: String,
    _time: String,
    _date: String,
    _path: String,
    _action: String

}

pub fn push(paths: &Vec<String>, action: &str) {
    
    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();
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
            fs::canonicalize(&paths[i]).ok().unwrap().to_str().unwrap(),
            &action]
        ).expect("sql query failed");
    } 

}

pub fn show(){
    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();
    
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
}

//if the action preformed on the file was "Trash"
fn u_trash(name: &Path, path: &Path){
    //systems home dir
  let home_path  = match  home::home_dir() {
    Some(path) => path,
    _ => panic!("Unable to trash files")
};

  if home_path.join(".ptrash").join(name).is_dir(){

     let entries = fs::read_dir(home_path.join(".ptrash").join(name)).expect("unable to parse directory");

     fs::create_dir_all(path).unwrap(); 
      
     for entry in entries {
         if let Ok(entry) = entry {
             if let Ok(file_type) = entry.file_type() {
                 if file_type.is_dir() {
                     // if it is a directory we need to copy the things in the directory . so call again with the new path
                     u_trash(&name.join(entry.file_name()), &path.join(entry.file_name()))
                 } else {

                     fs::copy(home_path.join(".ptrash").join(&name.join(entry.file_name())) ,path.join(entry.file_name())).unwrap();
                 }
             }
         }
     } 
  } else {
      fs::copy(home_path.join(".ptrash").join(name) , path).unwrap();
  }
}
//if the action preformed on the file was "Create"
fn u_create(path: &String){
    if (Path::new(path)).is_dir() {
        fs::remove_dir_all(path)
            .expect(format!("error deleting folder: {}", path).as_str());
    } else {
        fs::remove_file(path).expect(format!("error deleting file: {}", path).as_str());
    }
}
pub fn undo(){
    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();
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
        u_create(&latest_file._path);
    }
    else if latest_file._action == "Trash"{
        u_trash(Path::new(&latest_file._name),Path::new(&latest_file._path));
        //delete file in trash after
        let home_path  = match  home::home_dir() {
            Some(path) => path,
            _ => panic!("Unable to trash files")
        };
        let trash_file = home_path.join(".ptrash").join(&latest_file._name);
        if trash_file.is_dir() {
            fs::remove_dir_all(&trash_file)
                .expect(format!("error deleting folder: {}", &trash_file.display()).as_str());
        } else {
            fs::remove_file(&trash_file).expect(format!("error deleting file: {}", &trash_file.display()).as_str());
        }
    }
        
}