use std::io::{stdout, BufWriter, Write};

pub struct Formatter {
    names: Vec<String>,
    lengths: Vec<usize>,
    term_width: u16,
}


// that handles formatting the output
impl Formatter {
    pub fn from_n_elements(n: usize) -> Self {
        Self {
            names: Vec::with_capacity(n),
            lengths: Vec::with_capacity(n),
            term_width: crossterm::terminal::size().unwrap().0,
        }
    }

    pub fn with_lengths(mut self, lens: Vec<usize>) -> Self {
        self.lengths = lens;
        self
    }

    pub fn push_name(&mut self, string: String) {
        self.names.push(string)
    }

    pub fn format(self) -> anyhow::Result<()> {
        if self.one_line() {
            self.print()?;
        } else {
            self.column_print()?;
        }
        Ok(())
    }

    // Fxn to print strings to one line
    fn print(self) -> anyhow::Result<()> {
        let mut buf = BufWriter::new(stdout());

        for name in self.names {
                write!(buf, "{}  ", name)?
        }

        writeln!(buf)?;
        buf.flush()?;
        Ok(())
    }

    fn one_line(&self) -> bool {
        self.lengths.iter().sum::<usize>() < self.term_width as usize
        
    }

    // Fxn to print strings in formatted columns
    fn column_print(self) -> anyhow::Result<()> {
        let mut buf = BufWriter::new(stdout());
        let lines = calc_lines(&self.lengths[..], self.term_width);
        let cols = (self.lengths.len() as f32 / lines as f32).ceil() as usize;
        let widths = get_widths(&self.names[..], lines);

        // TODO: .color() and .bold() add characters under
        // the hood that make width weird, fix that shit
        for i in 0..lines {
            for j in 0..cols {
                if let Some(name) = self.names.get(i + (lines * j)) {
                    write!(buf, "{:<w$}", name, w = widths[j])?
                }
            }
            writeln!(buf)?;
        }

        // buf.flush()?;
        Ok(())
    }
}

fn get_widths(names: &[String], lines: usize) -> Vec<usize> {
    names.chunks(lines)
        .map(|c| c.iter().map(|s| s.len() + 2).max().unwrap())
        .collect()
}

fn calc_lines(lengths: &[usize], twidth: u16) ->  usize {
    let mut lines = 1;
    loop {
        let iter = lengths
            .chunks(lines)
            .map(|c| c.iter().max().unwrap().to_owned());
        if iter.sum::<usize>() < twidth.into() {
            break lines;
        } else {
            lines += 1;
        }
    }
}
