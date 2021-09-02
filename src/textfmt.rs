use std::io::{Write, BufWriter, stdout};

pub struct Formatter {
    strings: Vec<String>,
    widths: Vec<usize>,
}

// Just what I called the struct 
// that handles formatting the output
impl Formatter {
    pub fn new_from_vec(vec: Vec<String>) -> Self {
        Self { strings: vec, widths: Vec::with_capacity(6) }
    }

    pub fn format(self) -> anyhow::Result<()> {
        use crossterm::terminal::size;
        // Get the width of the terminal
        let (x, _) = size()?;

        let iter = self.strings.iter()
            .map(|string| string.len() - 18 );

        let sum = iter.sum::<usize>();

        // if the strings can fit in one line print them in one line
        // otherwise print with columns
        if sum < x as usize {
            Self::std_print(self.strings)?;
        } else {
            Self::column_print(self.strings, x)?;
        }

        Ok(())
    }

    // Fxn to print strings to one line
    fn std_print(strings: Vec<String>) -> anyhow::Result<()> {
        let mut buf = BufWriter::new(stdout());

        for string in strings {
            write!(buf, "{}", string)?;
        }

        writeln!(buf)?;
        buf.flush()?;
        Ok(())
    }

    fn entries_per_row(&mut self, slice: &[String]) -> anyhow::Result<()> {
        let (x, _) = crossterm::terminal::size()?;

        
    }

    fn row_print(strings: Vec<String>, x: u16) {

    }

    // Fxn to print strings in formatted columns
    fn column_print(strings: Vec<String>, x: u16) -> anyhow::Result<()> {
        let mut buf = BufWriter::new(stdout());
        // Number of columns basically
        let mut divisor: u16 = 1;

        // the most amount of characters that a potential
        // column could hold, compared to the variable x to 
        // determine whether there's room for another column or not
        let mut char_count: u16 = 0;

        // Vec that holds the widths of the longest string 
        // per column so that when i format the output the amount of
        // spacing is known
        let mut widths: Vec<u16> = Vec::with_capacity(6);

        // While the number of chars in a line is < the # of cols available
        // In the terminal divide the list of strings into more and more cols
        while char_count < x {
            divisor += 1;
            char_count = strings
                .chunks(strings.len() / divisor as usize)
                .map(|e| e.iter().max_by_key(|string| string.len()).unwrap())
                .fold(0, |acc, s| acc + s.len() as u16);
        }

        // Puhsing longest string lens to widths
        strings
            .chunks(strings.len() / divisor as usize)
            .map(|e| e.iter().max_by_key(|string| string.len()).unwrap())
            .for_each(|s| widths.push(s.len() as u16));

        // The amount of strings per col might not be perfectly
        // divisble so to account for that check to see if we need to add
        // 1 to s_per_col because at most a col should only have 1 more element 
        // than the others
        let s_per_col = if strings.len() % divisor as usize == 0 {
            strings.len() / divisor as usize
        } else {
            (strings.len() / divisor as usize) + 1
        };

        println!("DEBUG -- divisor: {} widths.len(): {}", divisor, widths.len());

        for i in 0..s_per_col {
            for j in 0..divisor {
                write!(
                    buf, 
                    "{:width$}", 
                    strings[i + (j as usize * s_per_col as usize)], width = widths[j as usize] as usize
                    )?
            }
            writeln!(buf)?;
        }

        Ok(())
    }
}

