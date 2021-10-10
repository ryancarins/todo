use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};
use todo_bin::{help, Todo};

#[derive(Deserialize, Serialize)]
struct Config {
    todo_file_path: Option<String>,
    todo_file_name: Option<String>,
    global: Option<bool>,
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
            todo_file_name: None,
            global: None,
        };
        fs::write(
            config_path,
            "#Path where todo file will be saved, defaults to /home/user/\n\
            #todo_file_path = \"/path/to/TODO\"\n\
            #Name of todo file, defaults to TODO\n\
            #todo_file_name = \"TODO\"\n\
            #Enable global config file, when enabled all todos are saved in the path above, otherwise it is saved in the directory the command is run in\n\
            #global = true",
        )
        .expect("Failed to write config file");
    }
    config
}

fn main() {
    let user_dirs = BaseDirs::new().expect("Home directory could not be found");
    let config_path = user_dirs.config_dir().join("todo/config.toml");

    let config = get_config(config_path);
    let todo_path: PathBuf;

    //If a file name is specified in the config use that otherwise use TODO
    let file_name = match config.todo_file_name {
        Some(name) => name,
        None => String::from("TODO"),
    };

    //Global defaults to true if not set
    let global = config.global.unwrap_or(true);

    //If global is set use the global path otherwise use the current directory
    if global {
        //If a path is specified in the config use that, otherwise use the users
        //home as a default
        todo_path = match config.todo_file_path {
            Some(todo_file_path) => PathBuf::from(todo_file_path).join(file_name),
            None => user_dirs.home_dir().to_path_buf().join(file_name),
        };
    } else {
        todo_path = PathBuf::from(".").join(file_name);
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
            _ => help(),
        }
    } else {
        todo.list();
    }
}
