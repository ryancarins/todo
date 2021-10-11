use colored::*;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process;

pub struct Todo {
    pub todo: Vec<TodoItem>,
    pub todo_path: PathBuf,
    pub num_colour: Option<(u8, u8, u8)>,
}

pub struct TodoItem {
    content: String,
    finished: bool,
}

impl Todo {
    pub fn new(todo_path: PathBuf, num_colour: Option<(u8, u8, u8)>) -> Result<Self, String> {
        let mut todo: Vec<TodoItem> = Vec::new();
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(todo_path.clone())
            .expect("Couldn't open the todofile");

        //Check if string is a completed task
        let strikethrough_regex = Regex::new(r"~~.*~~$").unwrap();
        //Get the actual task from a task that is complete selecting between
        //~~ and the ~~ at the end of the line
        let finished_content_regex = Regex::new(r"~~(.*)~~$").unwrap();

        let is_valid = Regex::new(r"^[0-9]*\. ").unwrap();
        //Get the actual task from a task that isn't complete
        let regular_content_regex = Regex::new(r"^[0-9]*\. (.*)$").unwrap();

        // Creates a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        // Empty String ready to be filled with TODOs
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();

        // Splits contents of the TODO file into lines
        let mut lines: Vec<String> = contents.to_string().lines().map(str::to_string).collect();

        //Build TodoItems from the strings
        for line in lines.iter_mut() {
            if !is_valid.is_match(line) {
                continue;
            }
            if strikethrough_regex.is_match(line) {
                let task_content = finished_content_regex
                    .captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();

                todo.push(TodoItem {
                    content: task_content.to_string(),
                    finished: true,
                })
            } else {
                let task_content = regular_content_regex
                    .captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();

                todo.push(TodoItem {
                    content: task_content.to_string(),
                    finished: false,
                })
            }
        }

        // Returns todo
        Ok(Self {
            todo,
            todo_path,
            num_colour,
        })
    }

    // Prints every todo saved
    pub fn list(&self) {
        // This loop will repeat itself for each taks in TODO file
        for (number, task) in self.todo.iter().enumerate() {
            // Converts virgin default number into a chad BOLD string
            let mut number = (number + 1).to_string().bold();
            if self.num_colour.is_some() {
                let colour = self.num_colour.unwrap();
                number = number.truecolor(colour.0, colour.1, colour.2);
            }

            // Checks if the current task is completed or not...
            if task.finished {
                println!("{} {}", number, &task.content.strikethrough());
            } else {
                println!("{} {}", number, &task.content);
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
            // This loop will repeat itself for each taks in TODO file
            for task in self.todo.iter() {
                // Checks if the current task is completed or not...
                if (!task.finished && arg[0] == "todo") || (task.finished && arg[0] == "done") {
                    println!("{}", task.content);
                }
            }
        }
    }
    // Adds a new todo
    pub fn add(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            // Appends a new task/s to the file
            // The plus one is because markdown lists start at 1
            self.todo.push(TodoItem {
                content: arg.to_string(),
                finished: false,
            });
        }
    }

    // Removes a task
    pub fn remove(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }

        //Do a sweep over the indicies to mark for removal
        //This will allow us to traverse the marked vector backwards
        //so that indicies remain the same as they are removed
        let mut marked = Vec::new();
        for (pos, task) in self.todo.iter().enumerate() {
            if (args.contains(&"done".to_string()) && task.finished)
                || args.contains(&(pos + 1).to_string())
            {
                marked.push(pos);
            }
        }

        marked.reverse();
        //Remove marked indicies
        for i in marked {
            self.todo.remove(i);
        }
    }

    // Sorts done tasks
    pub fn sort(&mut self) {
        let mut todo = Vec::new();
        let mut done = Vec::new();

        //Create two vectors. One for items that are done and one that aren't
        //Strip the numbering from the start
        //for task in self.todo.iter_mut() {
        while !self.todo.is_empty() {
            //Has O(n) complexity but maintains order. If too slow replace with VecDeque?
            let task = self.todo.remove(0);
            if !task.finished {
                todo.push(task);
            } else {
                done.push(task);
            }
        }

        self.todo.clear();
        self.todo.append(&mut todo);
        self.todo.append(&mut done);
    }

    pub fn done(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }

        for (pos, task) in self.todo.iter_mut().enumerate() {
            if args.contains(&(pos + 1).to_string()) {
                task.finished = !task.finished;
            }
        }
    }

    fn get_content_string(&self) -> String {
        let mut content_string = String::new();
        for (i, task) in self.todo.iter().enumerate() {
            if task.finished {
                content_string.push_str(&format!("{}. ~~{}~~\n", i + 1, task.content));
            } else {
                content_string.push_str(&format!("{}. {}\n", i + 1, task.content));
            }
        }
        content_string
    }

    pub fn write_to_file(&self, global: bool) {
        let todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open(self.todo_path.clone())
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);
        if global {
            buffer.write_all("# TODO: Global\n".as_bytes()).unwrap();
        } else {
            buffer
                .write_all(
                    format!(
                        "# TODO for project: {}\n",
                        self.todo_path
                            .parent()
                            .unwrap()
                            .canonicalize()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        buffer
            .write_all(self.get_content_string().as_bytes())
            .unwrap();
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
