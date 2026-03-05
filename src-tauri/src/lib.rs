/*
 * Copyright (c) 2026 LJC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

/* TODO:
 * - remove boilerplate
 * - handle errors better with less .unwrap()
 */

use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<String>,
}

const TODO_PATH: &str = "todo.json";

fn init_todo_list() -> () {
    if !fs::exists(TODO_PATH).unwrap() {
        let _ = File::create(TODO_PATH);
    }
}

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

fn overwrite_todo_list(content: &str) -> () {
    let mut file: File = File::create(TODO_PATH).unwrap();

    match file.write(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => eprintln!("Error writing to todo list: {}", e),
    };
}

#[tauri::command]
fn submit_task(task: &str) -> () {
    init_todo_list();

    let mut tasks: Vec<String> = get_tasks();
    tasks.append(&mut vec![String::from(task)]);

    let todo_list: TodoList = TodoList { tasks: tasks };
    overwrite_todo_list(&serde_json::to_string(&todo_list).unwrap());
}

#[tauri::command]
fn remove_task(task: &str) -> () {
    init_todo_list();

    let mut tasks: Vec<String> = get_tasks();
    let index = tasks.iter().position(|x| *x == task).unwrap();
    tasks.remove(index);

    let todo_list: TodoList = TodoList { tasks: tasks };
    overwrite_todo_list(&serde_json::to_string(&todo_list).unwrap());
}

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
