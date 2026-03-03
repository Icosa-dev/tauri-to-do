const { invoke } = window.__TAURI__.core;

/*
 * Note to frontend dev: the functions for
 * use from the backend are:
 * 
 * submit_task: adds a task (string)
 * remove_task: removes a task (string)
 * get_tasks: returns all tasks (array of strings)
 * 
 * To use these functions you must include
 * the following:
 * 
 * `await invoke("some_function", { someArg: value });`
 * 
 * Note that `await` being used means the above must be
 * inside an `async` function. Also note that the case of
 * the parameters of a method go from rust's standard 
 * snake_case to the JS standard camelCase. This shouldn't
 * be a problem now because the only parameter passed is
 * named "task". Here is an example:
 * 
 * async function submitTask(myTask) {
 *     await invoke("submit_task", { task: myTask});
 * }
 * 
 * Good luck!
 */