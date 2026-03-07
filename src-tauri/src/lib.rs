/* TODO:
 * - migrate persistent data handling to a KVDB
 * - add more documentation
 */

use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

// NOTE:
// If a cvs file parsing library exists
// already it would make more sense
// for the tasks to be stored in that
// format.

// TODO:
// Helper code that isn't accessable
// to the frontend should be held in
// a seperate file.

/// An object representing the 
/// data stored in the user's
/// `todo.json` file.
#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<String>,
}

/// The path to the JSON file which 
/// holds the user's tasks.
const TODO_PATH: &str = "todo.json";

fn init_todo_list() -> () {
    if !fs::exists(TODO_PATH).unwrap() {
        let _ = File::create(TODO_PATH);
    }
}

/// Overwrites the tasks in the user's 
/// `todo.json` file with the values 
/// supplied to by the `new_tasks`
/// parameter.
fn update_todo_list(new_tasks: Vec<String>) -> () {
    overwrite_todo_list(&serde_json::to_string(&TodoList { tasks: new_tasks }).unwrap());
}

/// Returns the contents of the user's
/// `todo.json` file as a String.
fn read_todo_list() -> String {
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
fn overwrite_todo_list(content: &str) -> () {
    let mut file: File = File::create(TODO_PATH).unwrap();

    match file.write(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => eprintln!("Error writing to todo list: {}", e),
    };
}

/// Adds a task to tasks in the user's
/// `todo.json` file.
#[tauri::command]
fn submit_task(task: &str) -> () {
    init_todo_list();

    let mut tasks: Vec<String> = get_tasks();
    tasks.append(&mut vec![String::from(task)]);

    update_todo_list(tasks);
}

/// Removes a task from tasks in the user's
/// `todo.json` file.
#[tauri::command]
fn remove_task(task: &str) -> () {
    init_todo_list();

    let mut tasks: Vec<String> = get_tasks();
    let index = tasks.iter().position(|x| *x == task).unwrap();
    tasks.remove(index);

    update_todo_list(tasks);
}

/// Returns all tasks from the user's
/// `todo.json` file.
#[tauri::command]
fn get_tasks() -> Vec<String> {
    init_todo_list();

    let todo_list_string: String = read_todo_list();
    let mut todo_list: TodoList = TodoList { tasks: Vec::new() };
    if todo_list_string != "" {
        todo_list = serde_json::from_str(&todo_list_string).unwrap();
    }

    todo_list.tasks
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            submit_task,
            remove_task,
            get_tasks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
