use std::{fs, path::Path};

use chrono::prelude::*;
use rusqlite::Connection;
use shellexpand::tilde;

//database struct
#[derive(Debug, Clone)]
struct Files {
    _name: String,
    _time: String,
    _date: String,
    _path: String,
    _action: String,
}

pub fn push(paths: &Vec<String>, action: &str) {
    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();
    let time = Local::now().format("%H:%M").to_string();
    let date = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            name    TEXT PRIMARY KEY,
            time    TEXT NOT NULL,
            date  TEXT NOT NULL,
            path   TEXT NOT NULL,
            action TEXT NOT NULL
        )",
        (),
    )
    .unwrap();

    conn.execute("DELETE FROM files where date < (?1)", [&date])
        .unwrap();
    for i in 0..paths.len() {
        conn.execute(
            "INSERT INTO files (name, time, date, path, action) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &paths[i],
                &time,
                &date,
                fs::canonicalize(&paths[i]).ok().unwrap().to_str(),
                &action,
            ),
        )
        .unwrap();
    }
}

pub fn show() {
    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();

    let mut stmt = conn
        .prepare("SELECT name, time, date, path, action FROM files")
        .unwrap();
    let file_iter = stmt
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
    for file in file_iter {
        println!("{:#?}", file.unwrap() as Files);
    }
}
#[allow(unused_variables)]
fn u_delete(path: &String) {
    todo!()
}
fn u_create(path: &String) {
    if (Path::new(path)).is_dir() {
        fs::remove_dir_all(path).expect(format!("error deleting folder: {}", path).as_str());
    } else {
        fs::remove_file(path).expect(format!("error deleting file: {}", path).as_str());
    }
}
pub fn undo() {
    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();
    let mut stmt = conn
        .prepare("SELECT name, time, date, path, action FROM files")
        .unwrap();
    let file_iter = stmt
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
    let latest_file = file_iter.last().unwrap().ok().unwrap();

    if latest_file._action == "Create" {
        u_create(&latest_file._path);
    } else {
        u_delete(&latest_file._path);
    }
}

