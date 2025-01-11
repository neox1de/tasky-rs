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
    categories: Vec<String>,
}

pub fn execute(id: &str, field: &str, new_value: &str) {
    // Validate status if we're updating the status field, new status must be one of the valid statuses
    let valid_statuses = ["To-Do", "In-Progress", "Done", "Deferred", "Cancelled"];
    if field == "status" && !valid_statuses.contains(&new_value) {
        eprintln!("Invalid status: {}", new_value);
        return;
    }

    if field == "title" && new_value.len() > 20 {
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

    let id: u32 = id.parse().expect("Invalid ID format");
    // Find and modify the task with the given ID
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        // match the specified field and update it with the new value
        match field {
            "title" => task.title = new_value.to_string(),
            "description" => task.description = new_value.to_string(),
            "status" => task.status = new_value.to_string(),
            "categories" => {
                // Split by commas for multiple categories
                task.categories = new_value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            _ => {
                eprintln!("Invalid field: {}", field);
                return;
            }
        }
    } else {
        eprintln!("Task with ID {} not found", id);
        return;
    }
    // Convert tasks into a pretty-printed JSON string and write it back to the file
    let serialized_tasks = serde_json::to_string_pretty(&tasks).expect("Unable to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("Unable to open file");
    file.write_all(serialized_tasks.as_bytes()).expect("Unable to write to file");

    println!("Task with ID {} updated successfully", id);
}