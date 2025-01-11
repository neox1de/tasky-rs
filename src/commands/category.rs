use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::BufReader;
use dirs;
use serde_json;

pub fn execute(name: &str, color: &str) {
    // Format and validate color
    let color = if color.starts_with('#') {
        color.to_string()
    } else {
        format!("#{}", color)
    };

    // Validate color format (should be either #RRGGBB or RRGGBB)
    if color.len() != 7 || !color.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
        eprintln!("Invalid color format. Must be a 6-digit hex color (e.g. ff0000 or #ff0000)");
        return;
    }

    let config_dir = dirs::config_dir().unwrap().join("tasky-rs");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    
    let colors_file = config_dir.join("categories.json");
    let mut colors: HashMap<String, String> = if colors_file.exists() {
        let file = File::open(&colors_file).expect("Unable to open categories file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        HashMap::new()
    };

    colors.insert(name.to_string(), color.to_string());

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&colors_file)
        .expect("Unable to open categories file");
        
    serde_json::to_writer_pretty(file, &colors).expect("Unable to write categories");
    println!("Category '{}' color set to {}", name, color);
}
