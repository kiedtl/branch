extern crate clap;

mod prog;
mod outp;

use clap::{ Arg, App };

const VERSION: &str = "0.1.0";

fn main() {
    let matches = App::new("branch(1)")
        .version(VERSION)
        .author("Kied Llaentenn")
        .about("tree(1) implemented in Rust")
        .arg(Arg::with_name("PATH")
            .help("Sets the input file to use")
            .required(false)
            .index(1))
        .get_matches();
    prog::branch(&matches);
}
