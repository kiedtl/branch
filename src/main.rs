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
            .help("Sets the input file to use.")
            .required(false)
            .index(1))
        .arg(Arg::with_name("threads")
             .help("The number of thread to use to crawl the directory tree.")
             .value_name("THREADS")
             .required(false)
             .short("t")
             .long("threads")
             .takes_value(true))
        .get_matches();
    prog::branch(&matches);
}
