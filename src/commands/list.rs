use std::fs::File;
use std::io::Read;
use serde_json;
use serde::{Serialize, Deserialize};
use colored::*;
use crate::utils::hex_to_ansi_color;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: String,
    categories: Vec<String>,
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

    println!("{}", "ID   | Title                  | Status       | Categories".bold().underline());
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
        
        // Format categories with their colors
        let categories = if task.categories.is_empty() {
            "-".to_string()
        } else {
            let colors = load_category_colors();
            task.categories
                .iter()
                .map(|cat| {
                    if let Some(hex_color) = colors.get(cat) {
                        format!("{}{}\x1b[0m", hex_to_ansi_color(hex_color), cat)
                    } else {
                        cat.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        };

        println!(
            "{:<4} | {:<22} | {} {:<10} | {}",
            task.id.to_string().bold(),
            task.title.bold(),
            status_dot,
            task.status.color(status_color),
            categories
        );
    }
}

fn load_category_colors() -> std::collections::HashMap<String, String> {
    let config_dir = dirs::config_dir().unwrap().join("tasky-rs");
    let colors_file = config_dir.join("categories.json");
    
    if !colors_file.exists() {
        return std::collections::HashMap::new();
    }

    let file = File::open(colors_file).expect("Unable to open categories file");
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_default()
}