use colored::*;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process;

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: PathBuf,
}

impl Todo {
    pub fn new(todo_path: PathBuf) -> Result<Self, String> {
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(todo_path.clone())
            .expect("Couldn't open the todofile");

        // Creates a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        // Empty String ready to be filled with TODOs
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();

        // Splits contents of the TODO file into a todo vector
        let todo = contents.to_string().lines().map(str::to_string).collect();

        // Returns todo
        Ok(Self { todo, todo_path })
    }

    // Prints every todo saved
    pub fn list(&self) {
        //Check if task contains a double ~~ and ends with one
        //technically this isn't part of the markdown spec but github
        //markdown uses this for strikethrough
        let strikethrough_regex = Regex::new(r"~~.*~~$").unwrap();

        //Check that an item starts with the form number. e.g. 123. Some task here
        let valid_item_regex = Regex::new(r"^[0-9]*\. ").unwrap();

        //Get the actual task from a task that isn't complete
        let regular_content_regex = Regex::new(r"^[0-9]*\. (.*)$").unwrap();

        //Get the actual task starting from the first ~~ and ending at the ~~ at
        //the end of the line
        let finished_content_regex = Regex::new(r"~~(.*)~~$").unwrap();
        // This loop will repeat itself for each taks in TODO file
        for (number, task) in self.todo.iter().enumerate() {
            //Skip any invalid line
            if !valid_item_regex.is_match(task) {
                continue;
            }
            // Converts virgin default number into a chad BOLD string
            let number = (number + 1).to_string().bold();

            // Checks if the current task is completed or not...
            if strikethrough_regex.is_match(task) {
                let task_content = finished_content_regex
                    .captures(task)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();
                println!("{} {}", number, task_content.strikethrough());
            } else {
                let task_content = regular_content_regex
                    .captures(task)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();
                println!("{} {}", number, task_content);
            }
        }
    }

    // This one is for yall, dmenu chads <3
    pub fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("todo raw takes only 1 argument, not {}", arg.len())
        } else if arg.is_empty() {
            eprintln!("todo raw takes 1 argument (done/todo)");
        } else {
            //Check that an item starts with the form number. e.g. 123. Some task here
            let valid_item_regex = Regex::new(r"^[0-9]*\. ").unwrap();

            //Check if task contains a double ~~ and ends with one
            //technically this isn't part of the markdown spec but github
            //markdown uses this for strikethrough
            let strikethrough_regex = Regex::new(r"~~.*~~$").unwrap();

            //Get the actual task from a task that isn't complete
            let regular_content_regex = Regex::new(r"^[0-9]*\. (.*)$").unwrap();

            //Get the actual task starting from the first ~~ and ending at the ~~ at
            //the end of the line
            let finished_content_regex = Regex::new(r"~~(.*)~~$").unwrap();

            // This loop will repeat itself for each taks in TODO file
            for task in self.todo.iter() {
                //Skip any invalid lines
                if !valid_item_regex.is_match(task) {
                    continue;
                }
                // Checks if the current task is completed or not...
                if !strikethrough_regex.is_match(task) && arg[0] == "todo" {
                    let task_content = regular_content_regex
                        .captures(task)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str();
                    println!("{}", task_content);
                } else if strikethrough_regex.is_match(task) && arg[0] == "done" {
                    let task_content = finished_content_regex
                        .captures(task)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str();
                    println!("{}", task_content);
                }
            }
        }
    }
    // Adds a new todo
    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }
        // Opens the TODO file with a permission to:
        let todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(self.todo_path.clone())
            .expect("Couldn't open the todofile");

        //Need this because making a writer moves the File reference which can't be cloned
        let temp_files = OpenOptions::new()
            .read(true)
            .open(self.todo_path.clone())
            .expect("Couldn't open the todofile");
        let line_count = BufReader::new(temp_files).lines().count();

        let mut buffer = BufWriter::new(todofile);
        for (i, arg) in args.iter().enumerate() {
            if arg.trim().is_empty() {
                continue;
            }

            // Appends a new task/s to the file
            // The plus one is because markdown lists start at 1
            let line = format!("{}. {}\n", line_count + i + 1, arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    // Removes a task
    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }
        // Opens the TODO file with a permission to:
        let todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open(self.todo_path.clone())
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);

        //Check if task contains a double ~~ and ends with one
        //technically this isn't part of the markdown spec but github
        //markdown uses this for strikethrough
        let strikethrough_regex = Regex::new(r"~~.*~~$").unwrap();

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&"done".to_string()) && strikethrough_regex.is_match(line) {
                continue;
            }
            if args.contains(&(pos + 1).to_string()) {
                continue;
            }

            let line = format!("{}\n", line);

            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    // Sorts done tasks
    pub fn sort(&self) {
        let mut todo = Vec::new();
        let mut done = Vec::new();

        let strikethrough_regex = Regex::new(r"~~.*~~$").unwrap();
        let valid_item_regex = Regex::new(r"^[0-9]*\. ").unwrap();
        let content_regex = Regex::new(r"^[0-9]*\. (.*)").unwrap();

        //Create two vectors. One for items that are done and one that aren't
        //Strip the numbering from the start
        for task in self.todo.iter() {
            if !valid_item_regex.is_match(task) {
                continue;
            }
            let task_content = content_regex
                .captures(task)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            if !strikethrough_regex.is_match(task) {
                todo.push(task_content);
            } else {
                done.push(task_content);
            }
        }
        todo.append(&mut done);
        //Reapply numbering for markdown
        for (i, item) in todo.iter_mut().enumerate() {
            *item = format!("{}. {}", i + 1, item).to_owned();
        }

        //Turn our now complete todo vector into a string
        let newtodo = todo.join("\n");
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open(self.todo_path.clone())
            .expect("Couldn't open the todo file");

        // Writes contents of a newtodo variable into the TODO file
        todofile
            .write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
    }

    pub fn done(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }

        // Opens the TODO file with a permission to overwrite it
        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.todo_path.clone())
            .expect("Couldn't open the todofile");
        let mut buffer = BufWriter::new(todofile);

        let strikethrough_regex = Regex::new(r"~~.*~~$").unwrap();
        let valid_item_regex = Regex::new(r"^[0-9]*\. ").unwrap();
        let content_regex = Regex::new(r"^[0-9]*\. (.*)").unwrap();

        for (pos, line) in self.todo.iter_mut().enumerate() {
            if !valid_item_regex.is_match(line) {
                continue;
            }

            let task_content: String = content_regex
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            if args.contains(&(pos + 1).to_string()) {
                if !strikethrough_regex.is_match(line) {
                    *line = format!("{}. ~~{}~~", pos + 1, task_content);
                } else {
                    //Remove the leading and trailing ~~
                    let mut chars = task_content.chars();
                    chars.next();
                    chars.next();
                    chars.next_back();
                    chars.next_back();
                    *line = format!("{}. {}", pos + 1, chars.as_str());
                }
            }
        }
        buffer.write(&self.todo.join("\n").as_bytes()).unwrap();
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s] 
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX] 
        removes a task
        Example: todo rm 4 
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort 
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";

pub fn help() {
    // For readability
    println!("{}", TODO_HELP);
}
