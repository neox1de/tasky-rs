mod commands;
mod config;

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
            let status = sub_m.get_one::<String>("STATUS").map(|s| s.as_str()).unwrap_or("To-Do");
            commands::add::execute(title, description, status);
        },
        Some(("remove", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            commands::remove::execute(id);
        },
        Some(("edit", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let field = sub_m.get_one::<String>("FIELD").unwrap();
            let value = sub_m.get_one::<String>("VALUE").unwrap();
            commands::edit::execute(id, field, value);
        },
        Some(("show", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            commands::show::execute(id);
        },
        Some(("set", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let status = sub_m.get_one::<String>("STATUS").unwrap();
            commands::set::execute(id, status);
        },
        None => {
            println!("Missing argument, use --help to get help.");
        },
        _ => {
            let _ = config::build_app().print_help();
        },
    }
}
