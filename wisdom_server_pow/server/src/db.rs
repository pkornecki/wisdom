use std::path::PathBuf;

/// a simple database storing its values in a vector
pub struct Db {
    content: Vec<String>,
}

impl Db {
    pub fn new(_file: PathBuf) -> Self {
        Db {
            content: vec!["first quote".to_string(), "second quote".to_string(), "third one".to_string()],
        }
    }

    pub fn num_quotes(&self) -> usize {
        self.content.len()
    }

    pub fn get_quote(&self, n: usize) -> Option<&String> {
        self.content.get(n)
    }
}
