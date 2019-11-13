// escape character (U+001B)
pub const E: char = 0x1B as char;

#[allow(dead_code)]
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
    pub fn get(&self) -> usize {
        match *self {
            ColorAttribute::Reset           => 0,
            ColorAttribute::Bold            => 1,
            ColorAttribute::Faint           => 2,
            ColorAttribute::Italic          => 3,
            ColorAttribute::Underline       => 4,
            ColorAttribute::Blink           => 5,
            ColorAttribute::Swap            => 7,
            ColorAttribute::Hidden          => 8,
            ColorAttribute::Strikethrough   => 9,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ColorFormat {
    pub foreground:     usize,
    pub background:     usize,
    pub attributes:     &'static [ColorAttribute],
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

impl ColorConfig {
    // retrieve a ColorConfig from LS_COLORS
    pub fn parse(&self) {
        // get LS_COLORS
        let ls_colors_env = std::env::var_os("LS_COLORS");
        
        // check if var is null
        if ls_colors_env == None {
            // TODO: return default config
            return;
        }

        

    }
}

