use std::fs::File;
use std::io::Read;
use serde_json;
use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: String,
}

pub fn execute() {
    let mut path = dirs::data_local_dir().unwrap().join("tasky-rs");
    path.push("todos.json");

    if !path.exists() {
        println!("No tasks found.");
        return;
    }

    let mut file = File::open(&path).expect("Unable to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");

    let tasks: Vec<Task> = if data.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&data).expect("Unable to parse JSON")
    };

    let tasks: Vec<Task> = tasks.into_iter().filter(|task| task.status != "Deleted").collect();

    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    println!("{}", "ID   | Title                  | Status".bold().underline());
    for task in tasks {
        let status_color = match task.status.as_str() {
            "To-Do" => "yellow",
            "In-Progress" => "blue",
            "Done" => "green",
            "Deferred" => "magenta",
            "Cancelled" => "red",
            _ => "white",
        };
        let status_dot = "●".color(status_color);
        println!(
            "{:<4} | {:<22} | {} {}",
            task.id.to_string().bold(),
            task.title.bold(),
            status_dot,
            task.status.color(status_color).bold()
        );
    }
}