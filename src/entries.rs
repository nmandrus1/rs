use super::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FType {
    File,
    Dir,
    Symlink,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    pub name: String,
    pub ftype: FType,
}

#[derive(Debug)]
pub struct Entries (pub Vec<Entry>);

impl Entries {
    pub fn dirs_only(&mut self) {
        remove_from_vec(&mut self.0, |entry: &Entry| entry.ftype != FType::Dir)
    }
    pub fn ignore_dotfiles(&mut self) {
        remove_from_vec(&mut self.0, |entry: &Entry| entry.name.starts_with('.'));
    }

    // From a path, grab all the items in it and put their info in Entry
    pub fn get_files_and_dirs(path: PathBuf) -> Result<Self> {
        let iter = std::fs::read_dir(path)?;
        let mut entries = Vec::with_capacity(50);

        for entry in iter {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                entries.push(Entry {
                    name: entry.file_name().into_string().expect("Not valid UTF-8"),
                    ftype: FType::Dir,
                })
            } else if entry.file_type()?.is_file() {
                entries.push(Entry {
                    name: entry.file_name().into_string().expect("Not valid UTF-8"),
                    ftype: FType::File,
                })
            } else {
                entries.push(Entry {
                    name: entry.file_name().into_string().expect("Not valid UTF-8"),
                    ftype: FType::Symlink,
                })
            }
        }

        sort_vec(&mut entries)?;

        Ok(Self(entries))
    }
}

// Take the vec and Pattern that yeilds T/F 
// and removes where the Pattern yeilds F
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

// Sort vector as if the '.' didnt exist on some of them
//
// Sort: Insertion Sort
fn sort_vec(slice: &mut [Entry]) -> anyhow::Result<()> {
    let mut other_vec: Vec<_> = Vec::with_capacity(slice.len());
    other_vec.extend_from_slice(slice);

    // Insertion Sort
    for i in 1..other_vec.len() {
        let mut j = i;
        while j > 0 && other_vec[j].name < other_vec[j - 1].name {
            other_vec.swap(j - 1, j);
            slice.swap(j - 1, j);
            j -= 1;
        }       
    }

    Ok(())
}

pub fn print_entries(mut entries: Entries) -> anyhow::Result<()> {
    use textfmt::Formatter;
    use crossterm::style::Stylize;

    sort_vec(&mut entries.0)?;

    let mut fmter = Formatter::from_n_elements(entries.0.len())
        .with_lengths(get_lens(&entries.0));

    for entry in entries.0 {
        match entry.ftype {
            FType::Dir => fmter.push_name(format!("{}", entry.name.blue().bold())),
            FType::File => fmter.push_name(format!("{}", entry.name.white().bold())),
            FType::Symlink => fmter.push_name(format!("{}", entry.name.cyan().bold())),
        }
    }

    fmter.format()?;
    Ok(())
}

fn get_lens(slice: &[Entry]) -> Vec<usize> {
    slice.iter().map(|e| e.name.len() + 2).collect()
}
