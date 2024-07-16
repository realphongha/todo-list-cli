use std::path::PathBuf;
use std::usize;
use chrono::{DateTime, Duration};
use chrono::prelude::Local;
use rusqlite::{params, Connection};
use colored::Colorize;
use rustyline::DefaultEditor;
use crate::config::Config;

struct Task {
    id: i32,
    name: String,
    description: String,
    status: i32,  // 0 - not done, 1 - done, 2 - removed
    done_at: Option<DateTime<Local>>,
}

impl Task {
    fn print(&self, i: usize) {
        if self.status == 1 {
            println!("{:<3}) {} {:<50} | {} {}",
                i, "[✓]".green(), self.name,
                "Done at:".green(),
                self.done_at.as_ref().unwrap().to_rfc2822().green()
            );
        } else if self.status == 2 {
            println!("{:<3}) {} {:<50} | {} {}",
                i, "[✗]".red(), self.name,
                "Removed at:".red(),
                self.done_at.as_ref().unwrap().to_rfc2822().red()
            );
            
        } else {
            println!("{:<3}) {} {:<50}", i, "[ ]".yellow(), self.name);
        }
        if !self.description.is_empty() {
            println!("     {}", self.description);
        }
    }
}

pub fn datetime_to_sql_string(datetime: DateTime<Local>) -> String {
    datetime.to_rfc3339()
}

pub fn sql_string_to_datetime(datetime: Option<String>) -> Option<DateTime<Local>> {
    if datetime.is_none() {
        return None;
    }
    Some(DateTime::parse_from_rfc3339(&datetime.unwrap()).unwrap().with_timezone(&Local))
}

pub fn get_db_conn(path: PathBuf) -> Connection {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create directories!");
        }
    }
    let conn = Connection::open(path.as_path()).expect(
        format!("Failed to open database: {:?}", path.display()).as_str());
    let res = conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            status INTEGER DEFAULT 0,
            done_at DATETIME
        )", []
    );
    res.expect("Failed to create table!");
    conn
}

fn get_list(conn: &Connection, cfg: &Config) -> Vec<Task> {
    // clean old tasks (that are done or removed for a long time)
    clean_by_datetime(conn, cfg);
    let mut stmt = conn.prepare(
        "SELECT id, name, description, status, done_at FROM tasks
            ORDER BY status, done_at"
    ).expect("Failed to prepare statement!");
    let results = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            done_at: Some(sql_string_to_datetime(row.get(4)?)).unwrap_or(None),
            //done_at: row.get(4)?
        })
    }).expect("Failed to query tasks!");

    let mut tasks = Vec::new();
    for task in results {
        tasks.push(task.unwrap());
    }
    tasks
}

pub fn list(conn: Connection, cfg: Config) {
    println!("TODO cli app. Run `todo --help` for more info.\n");
    let tasks = get_list(&conn, &cfg);
    if tasks.is_empty() {
        println!("{}", "No tasks to show!".red());
        return;
    }
    for (i, task) in tasks.iter().enumerate() {
        task.print(i+1);
    }
}

pub fn add(conn: Connection, cfg: Config, name_: &Option<String>, description_: &Option<String>) {
    let mut name = String::new();
    let mut description = String::new();
    let mut rl = DefaultEditor::new().expect("Can't create readline");
    if name_.is_none() {
        loop {
            name.clear();
            let readline = rl.readline("Enter task name (required): ");
            match readline {
                Ok(line) => {
                    name = line.trim().to_string();
                    let _ = rl.add_history_entry(line.as_str());
                },
                Err(err) => println!("Error reading line: {:?}", err),
            };
            if name.len() >= 50 {
                println!("{}", "Name should be less than 50 characters!".red());
                continue;
            }
            if name.len() <= 3 {
                println!("{}", "Name should be more than 3 characters!".red());
                continue;
            }
            if !name.is_empty() {
                break;        
            }
        }
    } else {
        name = name_.clone().unwrap();
    }
    if description_.is_none() {
        let readline = rl.readline("Enter task description (optional): ");
        match readline {
            Ok(line) => {
                description = line.trim().to_string();
                let _ = rl.add_history_entry(line.as_str());
            },
            Err(err) => println!("Error reading line: {:?}", err),
        };
    } else {
        description = description_.clone().unwrap();
    }
    let res = conn.execute(
        "INSERT INTO tasks (name, description) VALUES (?1, ?2)",
        params![name, description]
    );
    res.expect("Failed to add task!");
    println!();
    list(conn, cfg);
}

pub fn edit(conn: Connection, cfg: Config) {
    let tasks = get_list(&conn, &cfg);
    if tasks.is_empty() {
        println!("{}", "No tasks to edit!".red());
        return;
    }
    println!("Task list:");
    for (i, task) in tasks.iter().enumerate() {
        task.print(i+1); 
    }
    println!();
    let mut idx: usize = tasks.len();
    let mut rl = DefaultEditor::new().expect("Can't create readline");
    loop {
        let readline = rl.readline("Select task to edit: ");
        match readline {
            Ok(line) => {
                idx = line.trim().parse::<usize>().unwrap() - 1;
                let _ = rl.add_history_entry(line.as_str());
            },
            Err(err) => println!("Error reading line: {:?}", err),
        };
        if idx < tasks.len() as usize {
            break;
        }
    }
    let id = tasks[idx].id;

    let mut name = String::new();
    let mut description = String::new();
    loop {
        name.clear();
        let readline = rl.readline("Enter task name (required): ");
        match readline {
            Ok(line) => {
                name = line.trim().to_string();
                let _ = rl.add_history_entry(line.as_str());
            },
            Err(err) => println!("Error reading line: {:?}", err),
        };
        if name.len() >= 50 {
            println!("{}", "Name should be less than 50 characters!".red());
            continue;
        }
        if name.len() <= 3 {
            println!("{}", "Name should be more than 3 characters!".red());
            continue;
        }
        if !name.is_empty() {
            break;        
        }
    }

    let readline = rl.readline("Enter task description (optional): ");
    match readline {
        Ok(line) => {
            description = line.trim().to_string();
            let _ = rl.add_history_entry(line.as_str());
        },
        Err(err) => println!("Error reading line: {:?}", err),
    };

    let res = conn.execute(
        "UPDATE tasks SET name = ?1, description = ?2 WHERE id = ?3",
        params![name, description, id]
    );
    res.expect("Failed to edit task!");
    println!();
    list(conn, cfg);
}

pub fn done(conn: Connection, cfg: Config) {
    let tasks = get_list(&conn, &cfg);
    if tasks.is_empty() {
        println!("{}", "No tasks to edit!".red());
        return;
    }
    println!("Task list:");
    for (i, task) in tasks.iter().enumerate() {
        task.print(i+1); 
    }
    println!();
    let mut idx: usize = tasks.len();
    let mut rl = DefaultEditor::new().expect("Can't create readline");
    loop {
        let readline = rl.readline("Select task to mark as done: ");
        match readline {
            Ok(line) => {
                idx = line.trim().parse::<usize>().unwrap() - 1;
                let _ = rl.add_history_entry(line.as_str());
            },
            Err(err) => println!("Error reading line: {:?}", err),
        };
        if idx < tasks.len() as usize {
            break;
        }
    }
    let id = tasks[idx].id;
    let now = Local::now();
    let res = conn.execute(
        "UPDATE tasks SET status = 1, done_at = ?1 WHERE id = ?2",
        params![datetime_to_sql_string(now), id]
    );
    res.expect("Failed to edit task!");
    println!();
    list(conn, cfg);
}

pub fn delete(conn: Connection, cfg: Config) {
    let tasks = get_list(&conn, &cfg);
    if tasks.is_empty() {
        println!("{}", "No tasks to delete!".red());
        return;
    }
    println!("Task list:");
    for (i, task) in tasks.iter().enumerate() {
        task.print(i+1); 
    }
    println!();
    let mut idx: usize = tasks.len();
    let mut rl = DefaultEditor::new().expect("Can't create readline");
    loop {
        let readline = rl.readline("Select task to delete: ");
        match readline {
            Ok(line) => {
                idx = line.trim().parse::<usize>().unwrap() - 1;
                let _ = rl.add_history_entry(line.as_str());
            },
            Err(err) => println!("Error reading line: {:?}", err),
        };
        if idx < tasks.len() as usize {
            break;
        }
    }
    let id = tasks[idx].id;
    let now = Local::now();
    let res = conn.execute(
        "UPDATE tasks SET status = 2, done_at = ?1 WHERE id = ?2",
        params![datetime_to_sql_string(now), id]
    );
    res.expect("Failed to delete task!");
    println!();
    list(conn, cfg);
}

fn clean_by_datetime(conn: &Connection, cfg: &Config) {
    let now = Local::now();
    let datetime = now - Duration::days(cfg.task_life_cycle.keep_done_tasks as i64);
    let res = conn.execute(
        "DELETE FROM tasks WHERE done_at < ?1 AND status == 1",
        params![datetime_to_sql_string(datetime)]
    );
    res.expect("Failed to delete done tasks!");
    let datetime = now - Duration::days(cfg.task_life_cycle.keep_deleted_tasks as i64);

    let res = conn.execute(
        "DELETE FROM tasks WHERE done_at < ?1 AND status == 2",
        params![datetime_to_sql_string(datetime)]
    );
    res.expect("Failed to delete removed tasks!");
}

pub fn clean_all(conn: Connection) {
    let res = conn.execute("DELETE FROM tasks WHERE status > 0", []);
    res.expect("Failed to delete tasks!");
}
