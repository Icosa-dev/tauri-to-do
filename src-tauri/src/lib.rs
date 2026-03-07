mod util;
use util::*;

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
