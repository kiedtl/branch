use std::result;
use std::fs;
use std::io;
use std::env;

use std::vec::Vec;
use std::path::Path;
use clap::ArgMatches;

use crate::outp::*;

const E: char = 27 as char;

const BRANCH_ENTRY_STR: &str = "├── ";
const BRANCH_LINE_STR: &str = "│   ";
const BRANCH_LASTENTRY_STR: &str = "└── ";

#[derive(Debug)]
struct TreeEntry {
    name: String,
    is_dir: bool,
    is_symlink: bool,
    is_last: bool,
}

fn display(things: Vec<TreeEntry>, master_dir: String) {
    // print master directory
    println!("{}", master_dir);

    let mut depth = 0;
    for ctr in 0..things.len() {
        let thing = &things[ctr];
       
        // skip main directory, since we already printed that.
        if thing.name == master_dir {
            continue;
        }
       
        // print a space
        print!(" ");

        let relative_path = &*thing.name.replace(&master_dir, "");
        let relative_name = &*relative_path.split('/').collect::<Vec<_>>();
        depth = relative_name.len();
        for _ in 1..depth { print!("{}", BRANCH_LINE_STR); }
        if thing.is_last {
            println!("{}{}", BRANCH_LASTENTRY_STR, relative_name[relative_name.len()-1]);
        } else {
            println!("{}{}", BRANCH_ENTRY_STR, relative_name[relative_name.len()-1]);
        }
    }
    //print!(" └{}\n", nestend);
}

// get listing of contents of this
// directory
fn tree(directory: String, threadct: i32) -> result::Result<Vec<TreeEntry>, io::Error> {
    let mut entries: Vec<TreeEntry> = Vec::new();
    let mut is_dir_last = false;
    entries.push(TreeEntry {
            name: directory.clone(),
            is_dir: true,
            is_symlink: false,
            is_last: is_dir_last,
        });
   
    // walk file tree
    for thing in fs::read_dir(&*directory)? {
        let entry = &thing;
        let path = entry.as_ref().unwrap().path().display().to_string();
        let mut tree_entry: TreeEntry = TreeEntry {
            name: path.clone(),
            is_dir: false,
            is_symlink: false,
            is_last: false,
        };

        // check if path is directory, and if so, 
        // recursively get contents
        if entry.as_ref().unwrap().path().is_dir() { 
            tree_entry.is_dir = true;
            let newresults = tree(path, threadct)?;
            for newresult in newresults {
                entries.push(newresult);
            }
        }

        debug(format!("found entry {:?}", tree_entry));
        entries.push(tree_entry);
    }

    debug("done".to_owned());
    Ok(entries)
}

pub fn branch(matches: &ArgMatches) {
    let mut directory: String = env::current_dir()
        .unwrap()
        .display()
        .to_string();
    let mut threadct: i32 = 2;

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
    let result = tree(directory.clone(), threadct); 

    // match errors, just in case
    match result {
        Ok(entries) => display(entries, directory),
        Err(err) => error(format!(" {:?}", err)),
    }
}
