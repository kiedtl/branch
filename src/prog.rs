extern crate jwalk;

use std::fs;
use std::env;

use std::vec::Vec;
use std::path::Path;
use clap::ArgMatches;
use jwalk::{ WalkDir };

use crate::outp::die;

// get listing of contents of this
// directory
fn tree(directory: String, threadct: i32) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();

    for thing in WalkDir::new(&*directory)
        .sort(true)
        .num_threads(threadct as usize)
    {
        contents.push(thing
                      .unwrap()
                      .path()
                      .display()
                      .to_string());
    }

    return contents
}

pub fn branch(matches: &ArgMatches) {
    let mut directory: String = env::current_dir()
        .unwrap()
        .display()
        .to_string();
    let mut threadct: i32 = 1;

    // get directory
    if let Some(dir) = matches.value_of("PATH") {
        directory = dir.to_string();
    }

    // check that directory exists
    if ! fs::metadata(&directory).is_ok() {
        die(format!("directory {} does not exist.", directory));
    }

    // check that the thing is a directory 
    if ! Path::new(&directory).is_dir() {
        die(format!("path {} isn't a directory.", directory));
    }

    // get list of contents
    let contents: Vec<String> = tree(directory, threadct);

    // and print it out
    for item in contents {
        println!("{}", item);
    }
}
