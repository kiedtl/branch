use std::path::Path;

// tree statistics
pub struct TreeStatistics {
    pub directories: u64,
    pub files: u64,
}

// check if a file is hidden
pub fn is_hidden(entry: &Path) -> bool {
    entry.file_name()
        .unwrap()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
