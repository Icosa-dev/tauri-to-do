// NOTE:
// If a cvs file parsing library exists
// already it would make more sense
// for the tasks to be stored in that
// format.

use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

/// An object representing the
/// data stored in the user's
/// `todo.json` file.
#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<String>,
}

/// The path to the JSON file which
/// holds the user's tasks.
const TODO_PATH: &str = "todo.json";

pub fn init_todo_list() -> () {
    if !fs::exists(TODO_PATH).unwrap() {
        let _ = File::create(TODO_PATH);
    }
}

/// Overwrites the tasks in the user's
/// `todo.json` file with the values
/// supplied to by the `new_tasks`
/// parameter.
pub fn update_todo_list(new_tasks: Vec<String>) -> () {
    overwrite_todo_list(&serde_json::to_string(&TodoList { tasks: new_tasks }).unwrap());
}

/// Returns the contents of the user's
/// `todo.json` file as a String.
pub fn read_todo_list() -> String {
    let mut file: File = match OpenOptions::new().read(true).open(TODO_PATH) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error getting content from todo list: {}", e);
            return String::new();
        }
    };

    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);

    content
}

/// Overwrites the data in the user's `todo.json`
/// file with the text supplied by the `content`
/// parameter.
pub fn overwrite_todo_list(content: &str) -> () {
    let mut file: File = File::create(TODO_PATH).unwrap();

    match file.write(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => eprintln!("Error writing to todo list: {}", e),
    };
}
