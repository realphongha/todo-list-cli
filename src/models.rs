//use chrono::prelude::DateTime;
use chrono::DateTime;
use chrono::prelude::Local;
use rusqlite::{params, Connection, Result};


struct Task {
    id: i32,
    name: String,
    description: Option<String>,
    done: bool,
    done_at: Option<DateTime<Local>>,
}

pub fn list() {
    println!("list");
}

pub fn add() {
    println!("add");
}

pub fn edit() {
    println!("edit");
}

pub fn done() {
    println!("done");
}

pub fn delete() {
    println!("delete");
}
