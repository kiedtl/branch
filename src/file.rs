use std::path::Path;

// tree statistics
pub struct TreeStatistics {
    pub directories: u64,
    pub files: u64,
}

impl TreeStatistics {
    fn print_dirs(&self) {
        println!("{}", self.directories);
    }
    fn print_files(&self) {
        println!("{}", self.files);
    }
    fn print_all(&self) {
        print_dirs();
        print_files();
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
