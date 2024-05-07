// use chrono;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

pub struct FileDB {
    file: File,
    path: String,
}

impl FileDB {
    pub fn new() -> Self {
        let dir = String::from("./");
        let path_format = format!("{dir}work.txt");

        let file = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .create(true)
            .open(&path_format);

        FileDB {
            file: file.unwrap(),
            path: path_format,
        }
    }

    pub fn get(self) {
        let file = File::open(self.path).unwrap();
        let buffer = BufReader::new(file).lines();

        for (num, line) in buffer.enumerate() {
            println!("{}. {}", num + 1, line.unwrap())
        }
    }

    pub fn set(&mut self, topic: &String) {
        let _ = writeln!(self.file, "{}", topic);
    }

    pub fn del(&mut self, topic_id: u8) {
        {
            let file: File = File::open(&self.path).unwrap();
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
        fs::rename("./changes.txt", &self.path).unwrap();
    }

    pub fn del_all(self) {
        let _ = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.path);
    }
}

// pub struct Todo {
//     text: String,
//     date: chrono::DateTime<chrono::Utc>,
// }
