use std::fs;
use std::io::Write;
use std::path::PathBuf;

use toml::to_string_pretty;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[allow(dead_code)]
    pub task_life_cycle: TaskLifeCycle,
}

#[derive(Deserialize, Serialize)]
pub struct TaskLifeCycle {
    #[allow(dead_code)]
    pub keep_deleted_tasks: i32,
    #[allow(dead_code)]
    pub keep_done_tasks: i32,
}

impl Default for TaskLifeCycle {
    fn default() -> Self {
        Self {
            keep_deleted_tasks: 3,
            keep_done_tasks: 3,
        }
    }
}

fn write_default_config(path: PathBuf) {
    let config = Config {
        task_life_cycle: TaskLifeCycle::default(),
    };
    let config_str = to_string_pretty(&config).unwrap();
    let mut file = fs::File::create(path).unwrap();
    file.write_all(config_str.as_bytes()).unwrap();
}

pub fn read_config(path: PathBuf) -> Config {
    if !path.exists() {
        write_default_config(path.clone());
    }
    let config_str = fs::read_to_string(path).unwrap();
    toml::from_str(&config_str).unwrap()
}
