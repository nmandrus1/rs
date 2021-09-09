use std::path::PathBuf;

use std::fs::read_dir; 

use anyhow::Result;
use structopt::StructOpt;

mod entries;
use entries::Entries;

mod textfmt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rs", about = "list files and directories")]
struct Rs {
    // list all files and directories (including dotfiles)
    #[structopt(short, long)]
    all: bool,

    // list only directories
    #[structopt(short, long)]
    directory: bool,

    // use a long listing format
    #[structopt(short, long)]
    list: bool,

    // path to directory/file to list
    #[structopt(name = "FILE")]
    user_path: Option<PathBuf>,

    // actual path either cwd or user_path
    #[structopt(skip)]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let mut rs = Rs::from_args();

    // Sets path to current directory if no path was specified
    match rs.user_path {
        Some(p) => rs.path = p,
        None => rs.path = std::env::current_dir()?,
    };

    let mut entries = Entries::get_files_and_dirs(rs.path)?;

    if !rs.all { 
        entries.ignore_dotfiles() 
    } 

    if rs.directory { 
        entries.dirs_only();
    }

    if rs.list {
        todo!()
    }

    entries::print_entries(entries);


    Ok(())
}
