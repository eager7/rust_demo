extern crate core;

mod cli;
mod task;

use crate::task::Task;
use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("read file err"))?;
    match action {
        Add { task } => task::add_task(journal_file, Task::new(task)),
        List => task::list_tasks(journal_file),
        Done { position } => task::complete_task(journal_file, position),
    }?;
    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
