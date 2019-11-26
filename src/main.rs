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

use clap::{ Arg, App };

const VERSION: &str = "0.1.0";

fn main() {
    // TODO: add -d/--directories option to only list directories
    let matches = App::new("branch")
        .version(VERSION)
        .author("Kied Llaentenn")
        .about("recursively get paths quickly.")
        .arg(Arg::with_name("PATH")
            .help("Input directory to use. Default is current directory.")
            .required(false)
            .index(1))
        .arg(Arg::with_name("all")
             .help("Print hidden items.")
             .required(false)
             .short("a")
             .long("all")
             .takes_value(false))
        .arg(Arg::with_name("sort")
             .help("Sort files. Decreases performance.")
             .short("s")
             .long("sort"))
        .arg(Arg::with_name("level")
             .help("Maximum directory level to recurse into.")
             .short("l")
             .long("level")
             .takes_value(true))
        .arg(Arg::with_name("count")
             .help("Print count of dirs and files.")
             .short("c")
             .long("count"))
        .arg(Arg::with_name("dirs")
             .help("Print only directories.")
             .short("d")
             .long("dirs"))
        .get_matches();
    prog::branch(&matches);
}
