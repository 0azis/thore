# thore
Thore - is a simple todo CLI application.

```
$ thore
Thore is a simple todo application

Usage: thore <COMMAND>

Commands:
  todo    Todo commands
  folder  Folder command
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# Installation
Installation from source is only supported for now.\
**You must have a [cargo](https://crates.io/) installed*
```
$ git clone git@github.com:0azis/thore.git
$ cd thore
$ cargo install --path .
```

# Docs
Thore support some basic functions as every todo application does.\
Features divided into 2 parts - todo and folder.

## Todo
The interface of todo object is really simple.
```
$ thore todo
Todo commands

Usage: thore todo <COMMAND>

Commands:
  list, -L, --list    List of your todos
  add, -a, --add      Add a new todo
  del, -d, --del      Delete a todo
  clear, -c, --clear  Clear all todos
  help                Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

* Add a note 
```
$ thore todo add -t water 
$ thore todo list
1. water 12:00
```
You can create a todo record this way.\
Also, if you want to create a big note, you can make it this way:
```
$ thore todo add -t "milk and cookies"
$ thore todo list
1. water 12:00
2. milk and cookies 12:01
```
Also, if you want to create a note in another folder, you can make it this way:
``` 
$ thore todo add -t "water in big bottle" -f food
$ thore todo list -f food
1. water in big bottle 12:00
```
The folder will create automatically

* Delete a note
```
$ thore todo del 1
$ thore todo list 
1. milk and cookies 12:01
```
You can delete unnecessary note by printing the number of it.

* List of notes
```
$ thore todo list
1. milk and cookies 12:01
```
You can watch all your notes using the command "list"

* Clear all notes
```
$ thore todo clear
$ thore todo list
<nothing>
```
All notes was cleared

## Folder
The interface of folder object is really simple.
```
$ thore folder 
Folder commands

Usage: thore folder <COMMAND>

Commands:
  list, -L, --list  List of your folders
  add, -a, --add    Add a new folder
  del, -d, --del    Delete a folder
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

* Add a folder
``` 
$ thore folder add -f work
```
You can create a folder for current group of your notes./
thore.tmp - is a folder by default, you can't delete it.

* List of folders
```
$ thore folder list
1. thore.tmp
2. work.tmp
```

* Delete a folder
``` 
$ thore folder del -f work
$ thore folder list
1. thore.tmp
```

# Data Location
You can find Thore database in your cache directory. Depending on the operating system.