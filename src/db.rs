use rusqlite::Connection;
use chrono::prelude::*;
use shellexpand::tilde;


#[derive(Debug)]
struct Files{
    _path: String,
    _time: String,
    _date: String,

}

pub fn push(paths: &Vec<String>) {

    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();
    let local = Local::now().format("%H:%M").to_string();

    let date = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            path    TEXT PRIMARY KEY,
            time    TEXT NOT NULL,
            date  TEXT NOT NULL
        )",
        (),
    ).unwrap();


    conn.execute(
        "DELETE FROM files where date < (?1)",
        [&date],
    ).unwrap();
    for i in 0..paths.len(){
        conn.execute(
            "INSERT INTO files (path, time, date) VALUES (?1, ?2, ?3)",
        (&paths[i],&local, &date)).unwrap();
    }

    show();


}

pub fn show(){
    let conn: Connection = Connection::open(tilde("~/.punch/punch.db").to_string()).unwrap();

    let mut stmt = conn.prepare("SELECT path, time, date FROM files").unwrap();
    let file_iter = stmt.query_map([], |row| {
        Ok(Files {
                _path: row.get(0)?,
                _time: row.get(1)?,
                _date: row.get(2)?,
              })
        }).unwrap();
    for file in file_iter {
        println!("{:#?}", file.unwrap() as Files);
    }
}
