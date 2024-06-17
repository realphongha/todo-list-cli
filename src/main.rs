use std::env;

use clap::{Parser, Subcommand};

mod models;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all tasks, leaves COMMAND empty to use it without typing `list`
    List {},
    /// Add new task
    Add {
        name: Option<String>,
        description: Option<String>,
    },
    /// Edit task
    Edit {
        id: Option<i32>,
        name: Option<String>,
        description: Option<String>,
    },
    /// Mark task as done
    Done {
        id: Option<i32>,
    },
    /// Delete task
    Del {
        id: Option<i32>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::List {}) => models::list(),
        Some(Commands::Add {name, description}) => models::add(),
        Some(Commands::Edit {id, name, description}) => models::edit(),
        Some(Commands::Done {id}) => models::done(),
        Some(Commands::Del {id}) => models::delete(),
        None => models::list(),
    }
}
