use clap::{Arg, Command, ArgAction};
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
        .version(env!("CARGO_PKG_VERSION"))
        .about("A todo(task) manager written in Rust")
        .before_help(ASCII_BANNER)
        .subcommand(Command::new("list").about("List all tasks"))
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(
                    Arg::new("TITLE")
                        .required(true)
                        .help("The title of the task"),
                )
                .arg(
                    Arg::new("DESCRIPTION")
                        .required(true)
                        .help("The description of the task"),
                )
                .arg(
                    Arg::new("STATUS")
                        .required(false)
                        .value_parser(["To-Do", "In-Progress", "Done", "Deferred", "Cancelled"])
                        .help("The status of the task (default: To-Do)"),
                )
                .arg(
                    Arg::new("CATEGORIES")
                        .long("categories")
                        .required(false)
                        .action(ArgAction::Append)
                        .help("One or more categories for the task"),
                ),
        )
        .subcommand(
            Command::new("remove").about("Remove a task by ID").arg(
                Arg::new("ID")
                    .required(true)
                    .help("The ID of the task to remove"),
            ),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a task by ID")
                .arg(
                    Arg::new("ID")
                        .required(true)
                        .help("The ID of the task to edit"),
                )
                .arg(
                    Arg::new("FIELD")
                        .required(true)
                        .value_parser(["title", "description", "status", "categories"])
                        .help("The field to edit"),
                )
                .arg(
                    Arg::new("VALUE")
                        .required(true)
                        .help("New value / comma-separated list"),
                ),
        )
        .subcommand(
            Command::new("show")
                .about("Show details of a task by ID")
                .arg(
                    Arg::new("ID")
                        .required(true)
                        .help("The ID of the task to show"),
                ),
        )
        .subcommand(
            Command::new("set")
                .about("Set the status of a task by ID")
                .arg(Arg::new("ID").required(true).help("The ID of the task"))
                .arg(
                    Arg::new("STATUS")
                        .required(true)
                        .value_parser(["To-Do", "In-Progress", "Done", "Deferred", "Cancelled"])
                        .help("The status of the task"),
                ),
        )
        .subcommand(
            Command::new("category")
                .about("Manage task categories")
                .arg(
                    Arg::new("NAME")
                        .required(true)
                        .help("Category name"),
                )
                .arg(
                    Arg::new("COLOR")
                        .required(true)
                        .help("Color in hex format (e.g., #ff0000)"),
                ),
        )
}

pub fn get_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap()
        .join("tasky-rs")
}
