use clap::{App, SubCommand, Arg};
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

pub fn build_app() -> App<'static> {
    App::new("tasky")
        .bin_name("tasky")
        .version("0.1.0")
        .about("A todo(task) manager written in Rust")
        .before_help(ASCII_BANNER)
        .subcommand(SubCommand::with_name("list").about("List all tasks"))
        .subcommand(SubCommand::with_name("add")
            .about("Add a new task")
            .arg_from_usage("<TITLE> 'The title of the task'")
            .arg_from_usage("<DESCRIPTION> 'The description of the task'")
            .arg_from_usage("[STATUS] 'The status of the task (default: To-Do)'"))
        .subcommand(SubCommand::with_name("remove")
            .about("Remove a task by ID")
            .arg_from_usage("<ID> 'The ID of the task to remove'"))
        .subcommand(SubCommand::with_name("edit")
            .about("Edit a task by ID")
            .arg(Arg::with_name("ID").required(true).help("The ID of the task to edit"))
            .arg(Arg::with_name("FIELD").required(true).possible_values(&["title", "description", "status"]).help("The field to edit"))
            .arg(Arg::with_name("VALUE").required(true).help("The new value for the field")))
        .subcommand(SubCommand::with_name("show")
            .about("Show details of a task by ID")
            .arg_from_usage("<ID> 'The ID of the task to show'"))
        .subcommand(SubCommand::with_name("set")
            .about("Set the status of a task by ID")
            .arg_from_usage("<ID> 'The ID of the task'")
            .arg_from_usage("<STATUS> 'The new status of the task'"))
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
