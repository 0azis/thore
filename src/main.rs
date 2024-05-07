use clap::{arg, value_parser, Command};
mod thore;

fn cli() -> Command {
    Command::new("thore")
        .about("Thore is a simple todo application")
        .arg_required_else_help(true)
        .subcommand(Command::new("list").about("List your todos for today"))
        .subcommand(
            Command::new("add")
                .about("Add a new todo")
                .arg(arg!(<TODO> "The todo"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("del")
                .about("Delete a todo")
                .arg(arg!(<ID> "The number of todo").value_parser(value_parser!(u8)))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("clear").about("Clear all todos"))
}

fn main() {
    let matches = cli().get_matches();
    let mut store = thore::FileDB::new();

    match matches.subcommand() {
        Some(("list", _)) => store.get(),
        Some(("add", sub_args)) => store.set(sub_args.get_one::<String>("TODO").unwrap()),
        Some(("del", sub_args)) => store.del(sub_args.get_count("ID")),
        Some(("clear", _)) => store.del_all(),
        _ => println!("Thore is a simple todo application in your terminal"),
    }
}
