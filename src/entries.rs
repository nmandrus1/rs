use std::fs::ReadDir;

use super::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum FileType {
    File,
    Dir,
    Symlink,
}

#[derive(Debug, PartialEq, Clone)]
struct Entry {
    name: String,
    ftype: FileType,
}

#[derive(Debug)]
pub struct Entries {
    fvec: Vec<Entry>,
    dvec: Vec<Entry>,
}

impl Entries {
    pub fn new() -> Self {
        Self {
            fvec: Vec::with_capacity(35),
            dvec: Vec::with_capacity(35),
        }
    }

    pub fn print_entries(self) -> anyhow::Result<()> {
        use crossterm::style::Stylize;
        use crossterm::{execute, terminal::EnableLineWrap};
        use std::{io, io::Write};
        use textfmt::Formatter;

        let mut dvec = self.dvec;
        let mut fvec = self.fvec;

        execute!(io::stdout(), EnableLineWrap,)?;

        let stdout = io::stdout();
        let mut buffer = io::BufWriter::new(stdout);
        
        sort_vec(&mut dvec)?;
        sort_vec(&mut fvec)?;

        // Create one big iter and match on FileType
        let dvec = dvec.into_iter().chain(fvec.into_iter())
            .map(|entry| match entry.ftype {
                FileType::Dir => format!("{:5}  ", entry.name.blue().bold()),
                FileType::File => format!("{:5}  ", entry.name.white()),
                FileType::Symlink => format!("{:5}  ", entry.name.cyan().bold()),
            }).collect();
        
        // let formatter = Formatter::new_from_vec(dvec);
        let formatter = Formatter::new_from_vec(dvec).format();

        Ok(())
    }

    pub fn dirs_only(&mut self) {
        self.fvec.clear();
    }

    pub fn ignore_dotfiles(&mut self) {
        remove_from_vec(&mut self.dvec, |entry: &Entry| entry.name.starts_with('.'));
        remove_from_vec(&mut self.fvec, |entry: &Entry| entry.name.starts_with('.'));
    }

    pub fn get_files_and_dirs(&mut self, iter: ReadDir) -> Result<()> {
        for entry in iter {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                self.dvec.push(Entry {
                    name: entry.file_name().into_string().expect("Not valid UTF-8"),
                    ftype: FileType::Dir,
                })
            } else if entry.file_type()?.is_file() {
                self.fvec.push(Entry {
                    name: entry.file_name().into_string().expect("Not valid UTF-8"),
                    ftype: FileType::File,
                })
            } else {
                self.fvec.push(Entry {
                    name: entry.file_name().into_string().expect("Not valid UTF-8"),
                    ftype: FileType::Symlink,
                })
            }
        }

        Ok(())
    }
}

fn remove_from_vec<P>(vec: &mut Vec<Entry>, mut pattern: P) 
where P: FnMut(&Entry) -> bool
{
    let mut idxs_to_del: Vec<usize> = Vec::with_capacity(vec.len());
    for i in 0..vec.len() {
        if pattern(&vec[i]) { idxs_to_del.push(i) }
    }

    for i in 0..idxs_to_del.len() {
        vec.remove(idxs_to_del[i] - i);
    }
}

fn sort_vec(vec: &mut [Entry]) -> anyhow::Result<()> {
    let mut other_vec: Vec<_> = Vec::with_capacity(vec.len());
    other_vec.extend_from_slice(vec);

    let mut other_vec: Vec<_> = other_vec.iter_mut()
        .map(|entry| {
            if entry.name.starts_with('.') {
                entry.name.strip_prefix('.').unwrap()
            } else { entry.name.as_str() }
        }) 
        .collect();

    for i in 1..other_vec.len() {
        let mut j = i;
        while j > 0 && other_vec[j] < other_vec[j - 1]{
            other_vec.swap(j - 1, j);
            vec.swap(j - 1, j);
            j -= 1;
        }       
    }

    Ok(())
}
