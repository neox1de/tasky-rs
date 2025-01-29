use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::BufReader;
use dirs;
use serde_json;
use colored::*;

fn get_categories() -> HashMap<String, String> {
    let config_dir = dirs::config_dir().unwrap().join("tasky-rs");
    let colors_file = config_dir.join("categories.json");
    
    if colors_file.exists() {
        let file = File::open(&colors_file).expect("Unable to open categories file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_categories(categories: &HashMap<String, String>) {
    let config_dir = dirs::config_dir().unwrap().join("tasky-rs");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    let colors_file = config_dir.join("categories.json");
    
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&colors_file)
        .expect("Unable to open categories file");
        
    serde_json::to_writer_pretty(file, &categories).expect("Unable to write categories");
}

pub fn create(name: &str, color: &str) {
    let color = if color.starts_with('#') {
        color.to_string()
    } else {
        format!("#{}", color)
    };

    if color.len() != 7 || !color.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
        eprintln!("Invalid color format. Must be a 6-digit hex color (e.g. ff0000 or #ff0000)");
        return;
    }

    let mut categories = get_categories();
    categories.insert(name.to_string(), color.clone());
    save_categories(&categories);
    println!("Category '{}' created with color {}", name, color);
}

pub fn show() {
    let categories = get_categories();
    
    if categories.is_empty() {
        println!("No categories defined.");
        return;
    }

    println!("Categories:");
    for (name, color) in categories {
        println!("  {} - {}", name, color.truecolor(
            u8::from_str_radix(&color[1..3], 16).unwrap_or(255),
            u8::from_str_radix(&color[3..5], 16).unwrap_or(255),
            u8::from_str_radix(&color[5..7], 16).unwrap_or(255)
        ));
    }
}

pub fn remove(name: &str) {
    let mut categories = get_categories();
    
    if categories.remove(name).is_none() {
        println!("Category '{}' not found.", name);
        return;
    }
    
    save_categories(&categories);
    println!("Category '{}' removed successfully.", name);
}
