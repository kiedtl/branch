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

const E: char = 27 as char;
const branch_str: &str = "├── ";

fn display(thing: &String, isdir: bool, master_depth: usize) -> String {
    let path = thing.split("/").collect::<Vec<_>>();
    let mut nesting: String = "".to_string();
    let mut nestend: String = "".to_string();

    for _ in 0..path.len() - master_depth {
        nesting = nesting + "│  ";
        nestend = nestend + "──┘";
    }

    if isdir {
        print!(" {}{}{}[1;34m{}/\n{}[0m", nesting, branch_str, E, path[path.len() -2], E);
    } else {
        print!(" {}{}{}\n", nesting, branch_str, path[path.len() -1]);
    }
    
    return nestend
}

// get listing of contents of this
// directory
fn tree(directory: String, threadct: i32, master_depth: usize) -> result::Result<String, io::Error> {
    // we will display this when the program has finished
    let mut nestend: String = "".to_string();

    nestend = display(&directory, true, master_depth);

    // jwalk is more a liability than an asset
    // when only one thread is used, so we shall
    // walk the file tree ourselved in that case.
    if threadct > 1 {
        for thing in WalkDir::new(&*directory)
            .sort(true)
            .num_threads(threadct as usize) {
                let entry = &thing;
                if entry.as_ref().unwrap().path().is_dir() { 
                    nestend = display(&thing.unwrap().path().display().to_string(), true, master_depth);
                } else {
                    nestend = display(&thing.unwrap().path().display().to_string(), false, master_depth);
                }
        }
    } else {
        for thing in fs::read_dir(Path::new(&directory))? {
            let thing = thing?;
            if thing.path().is_dir() {
                tree(thing.path().display().to_string(), threadct, master_depth)?;
            } else {
                nestend = display(&thing.path().display().to_string(), false, master_depth);
            }
        }
    }

    Ok(nestend)
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

    // get number of threads
    if let Some(thread_count) = matches.value_of("threads") {
        threadct = thread_count.parse::<i32>().unwrap();
    }

    // get depth of master directory
    let master_depth = directory.split("/").collect::<Vec<_>>().len();

    // check that directory exists
    if ! fs::metadata(&directory).is_ok() {
        die(format!("directory {} does not exist.", directory));
    }

    // check that the thing is a directory 
    if ! Path::new(&directory).is_dir() {
        die(format!("path {} isn't a directory.", directory));
    }

    // add / to path
    if directory.chars().last().unwrap() != '/' {
        directory = directory + "/"
    }

    // print everything
    let result = tree(directory, threadct, master_depth); 

    // match errors, just in case
    match result {
        Ok(nestend) => print!(" └{}\n", nestend),
        Err(err) => error(format!(" {:?}", err)),
    }
}
