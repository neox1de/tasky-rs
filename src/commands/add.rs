use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use serde_json;
use serde::{Serialize, Deserialize};

use dirs;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: String,
}

pub fn execute(title: &str, description: &str, status: &str) {
    // Validate status
    let valid_statuses = ["To-Do", "In-Progress", "Done", "Deferred", "Cancelled"];
    if !valid_statuses.contains(&status) {
        eprintln!("Invalid status: {}", status);
        return;
    }
    // validate title length
    if title.len() > 20 {
        eprintln!("Title is too long. Maximum length is 20 characters.");
        return;
    }

    let mut path = dirs::data_local_dir().unwrap().join("tasky-rs");
    path.push("todos.json");

    let mut tasks: Vec<Task> = if path.exists() {
        let mut file = File::open(&path).expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read file");
        if data.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&data).expect("Unable to parse JSON")
        }
    } else {
        Vec::new()
    };

    // Generate new ID based on the current number of tasks + 1
    // Note: remove command does not remove the task, only marks them as "Deleted" to prevent failure in this section
    let new_id = tasks.len() as u32 + 1;
    let new_task = Task {
        id: new_id,
        title: title.to_string(),
        description: description.to_string(),
        status: status.to_string(),
    };

    tasks.push(new_task);

    let serialized_tasks = serde_json::to_string_pretty(&tasks).expect("Unable to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("Unable to open file");
    file.write_all(serialized_tasks.as_bytes()).expect("Unable to write to file");

    println!("Task added successfully with ID: {}", new_id);
}
