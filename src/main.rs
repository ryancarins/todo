use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};
use todo_bin::{help, Todo};

#[derive(Deserialize, Serialize)]
struct Config {
    todo_file_path: Option<String>,
    todo_file_name: Option<String>,
    markdown_export_path: Option<String>,
    markdown_export_file_name: Option<String>,
    global: Option<bool>,
    always_export: Option<bool>,
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
            markdown_export_path: None,
            markdown_export_file_name: None,
            global: None,
            always_export: None,
        };
        fs::write(
            config_path,
            "#Path where todo file will be saved, defaults to /home/user/\n\
            #todo_file_path = \"/path/to/TODO\"\n\n\
            #Path where markdown file will be saved, defaults to /home/user/\n\
            #markdown_export_path = \"/path/to/TODO\"\n\n\
            #Name of todo file, defaults to TODO\n\
            #todo_file_name = \"TODO\"\n\n\
            #Name of markdown file, defaults to TODO.md\n\
            #markdown_export_file_name = \"TODO.md\"\n\n\
            #Enable global config file, when enabled all todos are saved in the path above, otherwise it is saved in the directory the command is run in\n\
            #global = true\n\n\
            #If set then always export to markdown when the program is run\n\
            #always_export = true\n\n\
            ",
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
    let markdown_path: PathBuf;

    //If a file name is specified in the config use that otherwise use TODO
    let todo_name = match config.todo_file_name {
        Some(name) => name,
        None => String::from("TODO"),
    };

    let markdown_name = match config.markdown_export_file_name {
        Some(file_name) => file_name,
        None => String::from("TODO.md"),
    };

    //Global defaults to true if not set
    let global = config.global.unwrap_or(true);

    //If global is set use the global path otherwise use the current directory
    if global {
        //If a path is specified in the config use that, otherwise use the users
        //home as a default
        todo_path = match config.todo_file_path {
            Some(todo_file_path) => PathBuf::from(todo_file_path).join(todo_name),
            None => user_dirs.home_dir().to_path_buf().join(todo_name),
        };

        markdown_path = match config.markdown_export_path {
            Some(file_path) => PathBuf::from(file_path),
            None => user_dirs.home_dir().to_path_buf(),
        }
        .join(markdown_name);
    } else {
        todo_path = PathBuf::from(".").join(todo_name);
        markdown_path = PathBuf::from(".").join(markdown_name);
    }

    let mut todo = Todo::new(todo_path.clone()).expect("Couldn't create the todo instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "raw" => todo.raw(&args[2..]),
            "add" => {
                todo.add(&args[2..]);
                todo.save_data(todo_path);
            }
            "rm" => {
                todo.remove(&args[2..]);
                todo.save_data(todo_path);
            }
            "done" => {
                todo.done(&args[2..]);
                todo.save_data(todo_path);
            }
            "sort" => {
                todo.sort();
                todo.save_data(todo_path);
            }
            "export" => {
                todo.export_markdown(markdown_path.clone(), global);
            }
            "priority" => {
                todo.set_priority(&args[2..]);
                todo.save_data(todo_path);
            }
            _ => help(),
        }
    } else {
        todo.list();
    }
    if config.always_export.is_some() && config.always_export.unwrap() {
        todo.export_markdown(markdown_path, global);
    }
}
