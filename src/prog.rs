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
const BRANCH_NESTEND_STR: &str = "──┴";

#[derive(Debug)]
struct TreeEntry {
    name: String,
    is_dir: bool,
    is_symlink: bool,
    is_last: bool,
}

fn display(things: Vec<TreeEntry>, master_dir: String) {
    // print master directory
    println!("{}[1;34m{}{}[0m", E, master_dir, E);

    let mut depth = 0;
    for ctr in 0..things.len() {
        // forward slash
        let mut dirchar = "".to_string();

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

        // print branch character
        if thing.is_last {
            print!("{}", BRANCH_LASTENTRY_STR);
        } else {
            print!("{}", BRANCH_ENTRY_STR);
        }

        // fancy stuff for directories
        if thing.is_dir {
            dirchar = "/".to_string();
            print!("{}[1;34m", E);
        }
        
        // print item
        print!("{}{}", relative_name[relative_name.len()-1], dirchar);

        // newline and color clear
        print!("{}[0m\n", E);
    }

    let mut nestend = "".to_string();
    for _ in 1..depth { nestend = nestend + BRANCH_NESTEND_STR; }
        
    print!(" └{}\n", nestend);
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
