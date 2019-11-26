use std::path::Path;

// tree statistics
pub struct TreeStatistics {
    pub directories: u64,
    pub files: u64,
}

impl TreeStatistics {
    pub fn print_dirs(&self) {
        println!("{}", self.directories);
    }
    pub fn print_files(&self) {
        println!("{}", self.files);
    }
    pub fn print_all(&self) {
        self.print_dirs();
        self.print_files();
    }
}

// check if a file is hidden
pub fn is_hidden(entry: &Path) -> bool {
    entry.file_name()
        .unwrap()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
