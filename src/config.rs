use clap::{Arg, Command};
use dirs;
use std::path::PathBuf;

const ASCII_BANNER: &str = r#"
  _____         _
 |_   _|_ _ ___| | ___   _
   | |/ _` / __| |/ / | | |
   | | (_| \__ \   <| |_| |
   |_|\__,_|___/_|\_\\__, |
                     |___/
"#;

pub fn build_app() -> Command {
    Command::new("tasky")
        .bin_name("tasky")
        .version("0.1.0")
        .about("A todo(task) manager written in Rust")
        .before_help(ASCII_BANNER)
        .subcommand(Command::new("list").about("List all tasks"))
        .subcommand(Command::new("add")
            .about("Add a new task")
            .arg(Arg::new("TITLE").required(true).help("The title of the task"))
            .arg(Arg::new("DESCRIPTION").required(true).help("The description of the task"))
            .arg(Arg::new("STATUS").required(false).help("The status of the task (default: To-Do)")))
        .subcommand(Command::new("remove")
            .about("Remove a task by ID")
            .arg(Arg::new("ID").required(true).help("The ID of the task to remove")))
        .subcommand(Command::new("edit")
            .about("Edit a task by ID")
            .arg(Arg::new("ID").required(true).help("The ID of the task to edit"))
            .arg(Arg::new("FIELD").required(true).value_parser(["title", "description", "status"]).help("The field to edit"))
            .arg(Arg::new("VALUE").required(true).help("The new value for the field")))
        .subcommand(Command::new("show")
            .about("Show details of a task by ID")
            .arg(Arg::new("ID").required(true).help("The ID of the task to show")))
        .subcommand(Command::new("set")
            .about("Set the status of a task by ID")
            .arg(Arg::new("ID").required(true).help("The ID of the task"))
            .arg(Arg::new("STATUS").required(true).help("The new status of the task")))
}

pub fn get_data_dir() -> PathBuf {
    let dir = if cfg!(target_os = "linux") {
        dirs::data_local_dir().unwrap().join("tasky-rs")
    } else if cfg!(target_os = "macos") {
        dirs::data_dir().unwrap().join("tasky-rs")
    } else if cfg!(target_os = "windows") {
        dirs::data_dir().unwrap().join("tasky-rs")
    } else {
        panic!("Unsupported OS");
    };
    dir
}
