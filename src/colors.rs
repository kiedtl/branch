use std::vec::Vec;

// escape character (U+001B)
pub const E: char = 0x1B as char;

#[derive(Clone, Copy, Debug)]
pub enum ColorAttribute {
    Reset,
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,      // lol
    Swap,
    Hidden,
    Strikethrough,
}

impl ColorAttribute {
    pub fn get(&self) -> &'static str {
        match *self {
            ColorAttribute::Reset           => 0,
            ColorAttribute::Bold            => 1,
            ColorAttribute::Faint           => 2,
            ColorAttribute::Italic          => 3,
            ColorAttribute::Underline       => 4,
            ColorAttribute::Blink           => 5,
            ColorAttribute::Swap            => 7,
            ColorAttribute::Hidden          => 8,
            ColorAttribute::Strikethrough   => 8,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ColorFormat {
    pub Foreground:     usize,
    pub Background:     usize,
    pub Attributes:     Vec<ColorAttribute>,     
}

#[derive(Clone, Copy, Debug)]
pub struct ColorConfig {
    pub file:       ColorFormat,
    pub dir:        ColorFormat,
    pub link:       ColorFormat,
    pub pipe:       ColorFormat,
    pub blockdev:   ColorFormat,
    pub chardev:    ColorFormat,
    pub socket:     ColorFormat,
    pub special:    ColorFormat,
    pub exe:        ColorFormat,
}
