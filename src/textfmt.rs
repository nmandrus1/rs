use std::io::{Write, BufWriter, stdout};

use crossterm::style::Stylize;

use super::entries::{Entries, FType};

pub struct Formatter {
    entries: Entries,
    term_width: u16,
}

// Just what I called the struct 
// that handles formatting the output
impl Formatter {
    pub fn new_from_vec(entries: Entries) -> Self {
        Self { 
            entries,
            term_width: crossterm::terminal::size().unwrap().0,
        }
    }

    pub fn format(self) -> anyhow::Result<()> {
        if self.one_line() {

            self.print()?;
        } else {
            println!("Calculating columns...");
            self.column_print()?;
        }
        Ok(())
    }

    // Fxn to print strings to one line
    fn print(self) -> anyhow::Result<()> {
        let entries = self.entries;
        let mut buf = BufWriter::new(stdout());

        for entry in entries.0 {
            match entry.ftype {
                FType::Dir => write!(buf, "{}  ", entry.name.blue().bold())?,
                FType::File => write!(buf,"{}  ", entry.name.white())?,
                FType::Symlink => write!(buf, "{}  ", entry.name.cyan().bold())?,
            };
        }

        writeln!(buf)?;
        buf.flush()?;
        Ok(())
    }


    fn one_line(&self) -> bool {
        self.entries.0.iter().map(|s| s.name.len() + 2).sum::<usize>() < self.term_width as usize
    }

    // Fxn to print strings in formatted columns
    fn column_print(mut self) -> anyhow::Result<()> {
        let mut buf = BufWriter::new(stdout());

        let lengths: Vec<usize> = self.entries.0.iter().map(|e| e.name.len() + 2).collect();
        let (widths, lines) = calc_lines(&lengths[..], self.term_width);
        let cols = (lengths.len() as f32 / lines as f32).ceil() as usize;

        println!("{:?}", widths);

        for i in 0..lines {
            for j in 0..cols {
                if let Some(entry) = self.entries.0.get(i + (lines * j)){
                    println!("{}", widths[j]);
                    match entry.ftype {
                        FType::Dir => write!(buf, "{:width$} ", entry.name.clone().blue().bold(), width = 5)?,
                        FType::File => write!(buf, "{:w$}", entry.name.clone().white(), w = widths[j])?,
                        FType::Symlink => write!(buf, "{:w$}", entry.name.clone().cyan().bold(), w = widths[j])?,
                    }
                }
            }
            writeln!(buf)?;
        }

        // buf.flush()?;
        Ok(())
    }
}

fn calc_lines(lengths: &[usize], twidth: u16) -> (Vec<usize>, usize) {
    let mut lines = 1;
    loop {
            let iter = lengths.chunks(lines).map(|c| c.iter().max().unwrap().clone());
            if iter.clone().sum::<usize>() < twidth.into() {
                break (iter.collect(), lines);
            } else {
                lines += 1;
            }
        }
}
