use clap::{value_parser, Arg, ArgMatches, Command};
use thore::{FolderRepo, TodoRepo};
mod thore;

fn cli() -> Command {
    Command::new("thore")
        .about("Thore is a simple todo application")
        .arg_required_else_help(true)
        .version(thore::VERSION)
        .subcommand_required(true)
        .subcommand(
            Command::new("todo")
                .about("Todo commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List of your todos")
                        .short_flag('L')
                        .long_flag("list")
                        .arg(folder_flag()),
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a new todo")
                        .short_flag('a')
                        .long_flag("add")
                        .arg(Arg::new("todo").short('t').long("todo").help("Your note"))
                        .arg(folder_flag())
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("del")
                        .about("Delete a todo")
                        .short_flag('d')
                        .long_flag("del")
                        .arg(
                            Arg::new("id")
                                .short('i')
                                .long("id")
                                .help("id/number of your note")
                                .value_parser(value_parser!(u8)),
                        )
                        .arg(folder_flag())
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("clear")
                        .about("Clear all todos")
                        .short_flag('c')
                        .long_flag("clear")
                        .arg(folder_flag()),
                ),
        )
        .subcommand(
            Command::new("folder")
                .about("Folder commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List of your folders")
                        .short_flag('L')
                        .long_flag("list")
                        .arg(folder_flag()),
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a new folder")
                        .short_flag('a')
                        .long_flag("add")
                        .arg(folder_flag())
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("del")
                        .about("Delete a folder")
                        .short_flag('d')
                        .long_flag("del")
                        .arg(folder_flag())
                        .arg_required_else_help(true),
                ),
        )
}

fn folder_flag() -> clap::Arg {
    Arg::new("folder")
        .short('f')
        .long("folder")
        .default_value("thore")
        .default_missing_value("always")
}

fn folder_getter(args: &ArgMatches) -> String {
    let folder = args.get_one::<String>("folder").unwrap();
    return folder.to_string();
}

fn main() {
    let matches = cli().get_matches();
    let store = <thore::DB as thore::DBRepo>::new();
    let todo_repo = <thore::Todo as thore::TodoRepo>::init(store);
    let store = <thore::DB as thore::DBRepo>::new();
    let folder_repo = <thore::Folder as thore::FolderRepo>::init(store);

    match matches.subcommand() {
        Some(("todo", sub_matches)) => match sub_matches.subcommand() {
            Some(("list", args)) => {
                let folder = folder_getter(args);
                todo_repo.get(folder.to_string())
            }
            Some(("add", args)) => {
                let folder = folder_getter(args);
                let text = args.get_one::<String>("todo").unwrap();
                let todo = <thore::TodoModel as thore::TodoInterface>::new_todo(text.to_string());
                todo_repo.set(folder, todo)
            }
            Some(("del", args)) => {
                let folder = folder_getter(args);
                let id = args.get_count("id");
                todo_repo.del(folder, id)
            }
            Some(("clear", args)) => {
                let folder = folder_getter(args);
                todo_repo.del_all(folder)
            }
            _ => println!("Command for todo not found"),
        },
        Some(("folder", sub_matches)) => match sub_matches.subcommand() {
            Some(("list", _)) => {
                folder_repo.get();
            }
            Some(("add", args)) => {
                let folder = folder_getter(args);
                folder_repo.add(&folder);
            }
            Some(("del", args)) => {
                let folder = folder_getter(args);
                folder_repo.del(&folder);
            }
            _ => println!("Command not found"),
        },

        _ => println!("Command not found"),
    }
}
