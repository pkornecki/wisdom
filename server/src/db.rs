use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// a simple database storing its values in a vector
pub struct Db {
    content: Vec<String>,
}

impl Db {
    /// creates a new database from the file provided
    ///
    /// # arguments
    ///
    /// * `filename` - a path to the database file
    pub fn new(filename: impl AsRef<Path>) -> Self {
        Db {
            content: Self::read(filename).expect("error reading the contents of db"),
        }
    }

    /// returns the number of quotes in the database
    pub fn num_quotes(&self) -> usize {
        self.content.len()
    }

    /// returns the quote specified by the index
    ///
    /// # arguments
    ///
    /// * `n` - an index
    pub fn get_quote(&self, n: usize) -> Option<&String> {
        self.content.get(n)
    }

    fn read(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
        BufReader::new(File::open(filename)?).lines().collect()
    }
}
