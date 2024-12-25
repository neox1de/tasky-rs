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

pub fn execute(id: &str) {
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
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.status = "Deleted".to_string();
    } else {
        eprintln!("Task with ID {} not found", id);
        return;
    }

    let serialized_tasks = serde_json::to_string_pretty(&tasks).expect("Unable to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("Unable to open file");
    file.write_all(serialized_tasks.as_bytes()).expect("Unable to write to file");

    println!("Task with ID {} Deleted successfully", id);
}
