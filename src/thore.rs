use chrono;
use dirs;
use std::{
    fs::{self, remove_file, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Error, Write},
};

const DB_NAME: &str = "thore.tmp";
pub const VERSION: &str = "v0.2";

pub struct DB {
    path: String,
}

pub trait DBRepo {
    fn new() -> DB;
    fn opener(&self, folder: String) -> Result<File, Error>;
}

impl DBRepo for DB {
    fn new() -> DB {
        // define cache dir
        let mut dir = dirs::cache_dir().unwrap();
        let dir_str = dir.as_mut_os_str().to_str().unwrap();

        // create a cache dir
        let cache_dir = format!("{dir_str}/thore/");
        let _ = fs::create_dir(&cache_dir);

        // define path and create a file by this
        let file_path = format!("{}{}", cache_dir, DB_NAME);
        let _ = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .create(true)
            .open(file_path);

        // set and return the struct
        DB { path: cache_dir }
    }

    fn opener(&self, folder: String) -> Result<File, Error> {
        let file = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .create(true)
            .open(format!("{}{}.tmp", self.path, folder));
        file
    }
}

pub struct Todo {
    db: DB,
}

pub trait TodoRepo {
    fn init(db: DB) -> Todo;
    fn get(self, folder: String);
    fn set(self, folder: String, todo: TodoModel);
    fn del(self, folder: String, topic_id: u8);
    fn del_all(self, folder: String);
}

impl TodoRepo for Todo {
    fn init(db: DB) -> Todo {
        Todo { db }
    }
    fn get(self, folder: String) {
        let file = match self.db.opener(folder) {
            Ok(file) => file,
            Err(_) => panic!("Error while open file"),
        };
        let buffer = BufReader::new(file).lines();
        for (num, line) in buffer.enumerate() {
            println!("{}. {}", num + 1, line.unwrap())
        }
    }
    fn set(self, folder: String, todo: TodoModel) {
        let todo_str = todo.to_string();
        let mut file = match self.db.opener(folder) {
            Ok(file) => file,
            Err(_) => panic!("Error while open file"),
        };
        let _ = writeln!(file, "{}", todo_str);
    }
    fn del(self, folder: String, topic_id: u8) {
        {
            let file = match self.db.opener(folder) {
                Ok(file) => file,
                Err(_) => panic!("Error while open file"),
            };
            let out_file: File = File::create("./changes.txt").unwrap();

            let reader = BufReader::new(&file);
            let mut writer = BufWriter::new(&out_file);

            for (index, line) in reader.lines().enumerate() {
                let line = line.as_ref().unwrap();
                if index + 1 != usize::from(topic_id) {
                    let _ = writeln!(writer, "{}", line);
                }
            }
        }
        fs::rename("./changes.txt", self.db.path).unwrap();
    }
    fn del_all(self, folder: String) {
        let _ = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(format!("{}{}.tmp", self.db.path, folder));
    }
}

pub struct Folder {
    db: DB,
}

pub trait FolderRepo {
    fn init(db: DB) -> Folder;
    fn get(self);
    fn path(self, name: &String) -> String;
    fn add(self, name: &String);
    fn del(self, name: &String);
}

impl FolderRepo for Folder {
    fn init(db: DB) -> Folder {
        Folder { db }
    }
    // define folder path
    fn path(self, name: &String) -> String {
        let folder_path = format!("{}{}.tmp", self.db.path, name);
        folder_path
    }
    fn get(self) {
        let paths = fs::read_dir(self.db.path).unwrap();

        for (index, folder) in paths.enumerate() {
            println!(
                "{}. {}",
                index + 1,
                folder.unwrap().file_name().to_string_lossy().to_string()
            )
        }
    }
    fn add(self, name: &String) {
        let folder_path = self.path(name);
        let _ = File::create(folder_path);
    }
    fn del(self, name: &String) {
        let folder_path = self.path(name);
        let _ = remove_file(folder_path);
    }
}

pub struct TodoModel {
    text: String,
    time: String,
}

pub trait TodoInterface {
    fn new_todo(text: String) -> TodoModel;
    fn to_string(self) -> String;
}

impl TodoInterface for TodoModel {
    fn new_todo(text: String) -> TodoModel {
        let now = chrono::Local::now();
        let time = format!("{}", now.format("%H:%M"));
        TodoModel { text, time }
    }
    fn to_string(self) -> String {
        let mut todo: String = self.text.to_owned();
        let time: String = self.time.to_owned();
        todo.push_str(" ");
        todo.push_str(&time);
        todo
    }
}
