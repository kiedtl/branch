// Unicode characters used to format
// the tree view.

#[derive(PartialEq, Debug)]
pub enum TreeChars {
    Entry,
    Line,
    LastEntry,
    Blank,
}

impl TreeChars {
    pub fn get(&self) -> &'static str {
        match *self {
            TreeChars::Entry        => "├── ",
            TreeChars::Line         => "│   ",
            TreeChars::LastEntry    => "└── ",
            TreeChars::Blank        => "    ",
        }
    }
}
