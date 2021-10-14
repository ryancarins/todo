# Warning
This is my fork of [sioodomy's](https://github.com/sioodmy/todo) todo program.
I keep all of my branches around feel free to mess with any of them. I use the
personal_changes branch but keep in mind I have implemented many breaking changes
in various locations so backup your todo list if you want to try anything

## So far I've added
* Export to github flavoured markdown
* A toml config file with the option to change various things including:
    * todo file path
    * todo file name
    * markdown path
    * markdown name
    * a global flag that operates in the current directory when set to false
    * a flag to always export as markdown every time the program is run
* Instead of appending to or modifying a text file I now
use [bincode]() to store data and optionally export to markdown
* Coloured priority indicators

# todo
A lightweight and super fast cli todo program written in rust under 200 sloc

![gif](todo.gif)
## installation
[AUR package](https://aur.archlinux.org/packages/todo-bin/): `todo-bin`

use `cargo build --release` to compile todo and copy `target/release/todo` to `/usr/bin`
## note
todo is still really early in development so be careful or sth

btw i know that my code is not the best but im still learing 
## usage
```Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add "buy carrots"
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
```
