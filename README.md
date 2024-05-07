# thore
Thore - is a simple todo CLI application.

```
$ thore
Thore is a simple todo application

Usage: thore [COMMAND]

Commands:
  list   List your todos for today
  add    Add a new todo
  del    Delete a todo
  clear  Clear all todos
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# Basic functions
Thore support some basic functions as every todo application does.

## Add a note 
```
$ thore add water 
$ thore list
1. water 
```
You can create a todo record this way.\
Also, if you want to create a big note, you can make it this way:

```
$ thore add "milk and cookies"
$ thore list
1. water 
2. milk and cookies
```

## Delete a note
```
$ thore del 1
$ thore list 
1. milk and cookies
```
You can delete unnecessary note by printing the number of it.

## List of notes
```
$ thore list
1. milk and cookies
```
You can watch all your notes using the command "list"

## Clear all notes
```
$ thore clear
$ thore list
<nothing>
```
Clear all notes

# Data Location
You can find Thore database in your cache directory. Depending on the operating system.