use std::path::PathBuf;
use dirs_next::home_dir;

use clap::{Parser, Subcommand};

mod models;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Use local database `./.todo.sqlite3`
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    local: bool,

    /// Use global database `~/.todo/db.sqlite` 
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    global: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all tasks, leaves COMMAND empty to use it without typing `list`
    List {},
    /// Add a new task
    Add {
        name: Option<String>,
        description: Option<String>,
    },
    /// Edit a task
    Edit {},
    /// Mark a task as done
    Done {},
    /// Delete a task
    Del {},
}

fn main() {
    let args = Cli::parse();
    let local_db_path = PathBuf::from("./.todo/db.sqlite");
    let home = home_dir().expect("Failed to get home directory!");
    let global_db_path = home.join(".todo/db.sqlite");
    assert!(!args.global || !args.local, "Cannot use global and local database at the same time!");
    let conn = if args.local {
        models::get_db_conn(local_db_path)
    } else if args.global {
        models::get_db_conn(global_db_path)
    } else if local_db_path.exists() {
        models::get_db_conn(local_db_path)
    } else {
        models::get_db_conn(global_db_path)
    };

    match &args.command {
        Some(Commands::Add {name, description})
            => models::add(conn, &name, &description),
        Some(Commands::Edit {}) => models::edit(conn),
        Some(Commands::Done {}) => models::done(conn),
        Some(Commands::Del {}) => models::delete(conn),
        Some(Commands::List {}) => models::list(conn),
        None => models::list(conn),
    }
}
