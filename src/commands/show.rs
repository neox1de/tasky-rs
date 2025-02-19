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

pub fn execute(id: &str) {
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

    let id: u32 = id.parse().expect("Invalid ID format");
    if let Some(task) = tasks.iter().find(|task| task.id == id) {
        // Determine color based on status for status dot and text
        let status_color = match task.status.as_str() {
            "To-Do" => "yellow",
            "In-Progress" => "blue",
            "Done" => "green",
            "Deferred" => "magenta",
            "Cancelled" => "red",
            "Deleted" => "red",
            _ => "white",
        };

        // Format description with word wrap and box drawing
        let wrapped_lines = wrap_text(&task.description, 60);
        // Calculate box width based on content
        let max_line_length = wrapped_lines.iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);
        let box_width = max_line_length + 2;

        println!("{}", "Task Details".bold().underline());
        println!();
        println!("{}: {}", "ID".bold(), task.id);
        println!("{}: {}", "Title".bold(), task.title.bold());
        println!("{}:", "Description".bold());
        println!("┌{}┐", "─".repeat(box_width));
        for line in wrapped_lines {
            println!("│ {}{} │", line, " ".repeat(box_width - line.len() - 2));
        }
        println!("└{}┘", "─".repeat(box_width));
        println!();
        println!("{}: {} {}", "Status".bold(), "●".color(status_color), task.status.color(status_color));
        
        // Format categories with their colors
        if !task.categories.is_empty() {
            println!("{}:", "Categories".bold());
            let colors = load_category_colors();
            for cat in &task.categories {
                if let Some(hex_color) = colors.get(cat) {
                    println!("  • {}{}\x1b[0m", hex_to_ansi_color(hex_color), cat);
                } else {
                    println!("  • {}", cat);
                }
            }
        }
    } else {
        eprintln!("Task with ID {} not found", id);
    }
}

// Word wrap function that preserves whole words.
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    // Split text into words and wrap at width boundary
    for word in text.split_whitespace() {
        if !current.is_empty() && current.len() + word.len() + 1 > width {
            lines.push(current);
            current = String::new();
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
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
