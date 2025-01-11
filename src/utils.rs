pub fn hex_to_ansi_color(hex: &str) -> String {
    if hex.len() != 7 || !hex.starts_with('#') {
        return String::from("white");
    }

    // Convert hex to RGB
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(255);

    format!("\x1b[38;2;{};{};{}m", r, g, b)
}
