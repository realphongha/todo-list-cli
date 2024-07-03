use std::path::PathBuf;

use dirs_next::home_dir;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

mod config;
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
    /// Clean all done and removed tasks
    Clean {}
}

fn main() {
    let args = Cli::parse();
    let local_db_path = PathBuf::from("./.todo/db.sqlite");
    let local_cfg_path = PathBuf::from("./.todo/config.toml");
    let home = home_dir().expect("Failed to get home directory!");
    let global_db_path = home.join(".todo/db.sqlite");
    let global_cfg_path = home.join(".todo/config.toml");
    assert!(!args.global || !args.local,
        "Cannot use global and local database at the same time!");
    let conn: Connection;
    let cfg: config::Config;
    if args.local {
        conn = models::get_db_conn(local_db_path);
        cfg = config::read_config(local_cfg_path);
    } else if args.global {
        conn = models::get_db_conn(global_db_path);
        cfg = config::read_config(global_cfg_path);
    } else if local_db_path.exists() {
        conn = models::get_db_conn(local_db_path);
        cfg = config::read_config(local_cfg_path);
    } else {
        conn = models::get_db_conn(global_db_path);
        cfg = config::read_config(global_cfg_path);
    }

    match &args.command {
        Some(Commands::Add {name, description})
            => models::add(conn, cfg, &name, &description),
        Some(Commands::Edit {}) => models::edit(conn, cfg),
        Some(Commands::Done {}) => models::done(conn, cfg),
        Some(Commands::Del {}) => models::delete(conn, cfg),
        Some(Commands::List {}) => models::list(conn, cfg),
        Some(Commands::Clean {}) => models::clean_all(conn),
        None => models::list(conn, cfg),
    }
}
