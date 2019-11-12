// for multitasking
// TODO: use custom implementation.
extern crate rayon;

// argument parsing
// TODO: switch to a lighter lib.
extern crate clap;

// for debugging statements via log::debug!()
#[macro_use] extern crate log;

// main program logic
mod prog;

// output functions
// i.e. debug(), error(), die(), etc
mod outp;

// misc file helper functions
mod file;

// icons and glyphs
mod glyf;

// LS_COLORS
mod clrs;

use clap::{ Arg, App };

const VERSION: &str = "0.1.0";

fn main() {
    let matches = App::new("branch(1)")
        .version(VERSION)
        .author("Kied Llaentenn")
        .about("tree(1) implemented in Rust")
        .arg(Arg::with_name("PATH")
            .help("Sets input directory to use. Default is current directory (`.').")
            .required(false)
            .index(1))
        .arg(Arg::with_name("all")
             .help("Print hidden items.")
             .required(false)
             .short("a")
             .long("all")
             .takes_value(false))
        .arg(Arg::with_name("nosort")
             .help("Do not sort files. May improve performance.")
             .short("S")
             .long("nosort"))
        .arg(Arg::with_name("level")
             .help("Maximum directory level to recurse into.")
             .short("L")
             .long("level")
             .takes_value(true))
        .get_matches();
    prog::branch(&matches);
}
