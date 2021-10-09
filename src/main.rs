use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};
use todo_bin::{help, Todo};

#[derive(Deserialize, Serialize)]
struct Config {
    todo_file_path: Option<String>,
}

fn get_config(config_path: PathBuf) -> Config {
    let config: Config;
    if config_path.exists() {
        config =
            toml::from_str(&fs::read_to_string(config_path).expect("Could not read config file"))
                .unwrap();
    } else {
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        config = Config {
            todo_file_path: None,
        };
        fs::write(
            config_path,
            "#Path to TODO file, defaults to /home/user/TODO\n#todo_file_path = \"/path/to/TODO\"",
        )
        .expect("Failed to write config file");
    }
    config
}

fn main() {
    let user_dirs = BaseDirs::new().expect("Home directory could not be found");
    let config_path = user_dirs.config_dir().join("todo/config.toml");

    let todo_path: PathBuf;
    let config = get_config(config_path);

    if config.todo_file_path.is_none() {
        let home = user_dirs.home_dir().to_path_buf();

        todo_path = home.join("TODO");
    } else {
        todo_path = PathBuf::from(config.todo_file_path.unwrap());
    }

    let mut todo = Todo::new(todo_path).expect("Couldn't create the todo instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" => todo.remove(&args[2..]),
            "done" => todo.done(&args[2..]),
            "raw" => todo.raw(&args[2..]),
            "sort" => todo.sort(),
            "help" | "--help" | "-h" | _ => help(),
        }
    } else {
        todo.list();
    }
}
