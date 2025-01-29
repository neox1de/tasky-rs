mod commands;
mod config;
mod utils;

use std::fs;

fn main() {
    let data_dir = config::get_data_dir();
    let todos_file = data_dir.join("todos.json");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect("Failed to create data directory");
    }

    if !todos_file.exists() {
        fs::File::create(&todos_file).expect("Failed to create todos.json file");
    }

    let matches = config::build_app().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => commands::list::execute(),
        Some(("add", sub_m)) => {
            let title = sub_m.get_one::<String>("TITLE").unwrap();
            let description = sub_m.get_one::<String>("DESCRIPTION").unwrap();
            let status = sub_m
                .get_one::<String>("STATUS")
                .map(|s| s.as_str())
                .unwrap_or("To-Do");
            let categories: Vec<String> = sub_m
                .get_many::<String>("CATEGORIES")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();
            commands::add::execute(title, description, status, categories);
        }
        Some(("remove", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            commands::remove::execute(id);
        }
        Some(("edit", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let field = sub_m.get_one::<String>("FIELD").unwrap();
            let value = sub_m.get_one::<String>("VALUE").unwrap();
            commands::edit::execute(id, field, value);
        }
        Some(("show", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            commands::show::execute(id);
        }
        Some(("set", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let status = sub_m.get_one::<String>("STATUS").unwrap();
            commands::set::execute(id, status);
        }
        Some(("category", sub_m)) => match sub_m.subcommand() {
            Some(("new", new_matches)) => {
                let name = new_matches.get_one::<String>("NAME").unwrap();
                let color = new_matches.get_one::<String>("COLOR").unwrap();
                commands::category::create(name, color);
            }
            Some(("show", _)) => {
                commands::category::show();
            }
            Some(("remove", remove_matches)) => {
                let name = remove_matches.get_one::<String>("NAME").unwrap();
                commands::category::remove(name);
            }
            _ => {
                println!("Invalid category subcommand. Use --help for usage information.");
            }
        },
        None => {
            println!("Missing argument, use --help to get help.");
        }
        _ => {
            let _ = config::build_app().print_help();
        }
    }
}