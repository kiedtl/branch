extern crate jwalk;

use std::result;
use std::fs;
use std::io;
use std::env;

use std::vec::Vec;
use std::path::Path;
use clap::ArgMatches;
use jwalk::{ WalkDir };

use crate::outp::*;

const E: char = 0x1B as char;
const branch_str: &str = "├── ";

fn display(thing: &String, isdir: bool) {
    let path = thing.split("/").collect::<Vec<_>>();
    let mut nesting: String = "".to_string();

    for _ in 0..path.len() {
        nesting = format!("{}│  ", nesting);
    }

    if isdir {
        println!(" {}{}[1;34m{}/\n{}0m", nesting, branch_str, E, path[path.len() -1]);
    } else {
        println!(" {}{}{}\n", nesting, branch_str, path[path.len() -1]);
    }
}

// get listing of contents of this
// directory
fn tree(directory: String, threadct: i32) -> result::Result<(), io::Error> {
    display(&directory, true);

    // jwalk is more a liability than an asset
    // when only one thread is used, so we shall
    // walk the file tree ourselved in that case.
    if threadct > 1 {
        for thing in WalkDir::new(&*directory)
            .sort(true)
            .num_threads(threadct as usize) {
                let entry = &thing;
                if entry.as_ref().unwrap().path().is_dir() { 
                    display(&thing.unwrap().path().display().to_string(), true);
                } else {
                    display(&thing.unwrap().path().display().to_string(), false);
                }
        }
    } else {
        for thing in fs::read_dir(Path::new(&directory))? {
            let thing = thing?;
            if thing.path().is_dir() {
                tree(thing.path().display().to_string(), threadct)?;
            } else {
                display(&thing.path().display().to_string(), false);
            }
        }
    }

    Ok(())
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

    // print everything
    let result = tree(directory, threadct);

    // match errors, just in case
    match result {
        Ok(_) => (),
        Err(err) => error(format!(" {:?}", err)),
    }
}
