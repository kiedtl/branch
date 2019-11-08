extern crate rayon;
extern crate walkdir;
extern crate clap;
#[macro_use] extern crate log;

mod prog;
mod outp;
mod file;

use clap::{ Arg, App };

const VERSION: &str = "0.1.0";

fn main() {
    let matches = App::new("branch(1)")
        .version(VERSION)
        .author("Kied Llaentenn")
        .about("tree(1) implemented in Rust")
        .arg(Arg::with_name("PATH")
            .help("Sets the input file to use.")
            .required(false)
            .index(1))
        .arg(Arg::with_name("all")
             .help("Print ALL items, including hidden files (but not including items ignored by Git")
             .required(false)
             .short("a")
             .long("all")
             .takes_value(false))
        .get_matches();
    prog::branch(&matches);
}
