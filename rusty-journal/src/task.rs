use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Error, ErrorKind, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let create_at = Utc::now();
        Task { text, create_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let mut tasks = collection_task(&mut file);

    file.seek(SeekFrom::Start(0))?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let mut tasks = collection_task(&mut file);
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid task position"));
    }
    tasks.remove(task_position - 1);
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let tasks = collection_task(&mut file);
    let mut idx = 0;
    for task in tasks {
        idx += 1;
        println!("{},{:?}", idx, task);
    }
    Ok(())
}

fn collection_task(file: &mut File) -> Vec<Task> {
    let mut tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => panic!("{}", e.to_string().as_str()),
    };
    return tasks;
}
