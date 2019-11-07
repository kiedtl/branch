use std::fs::{ DirEntry };

// tree statistics
pub struct TreeStatistics {
    pub directories: u64,
    pub files: u64,
}

// check if a file is hidden
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
