use std::result;
use std::fs;
use std::io;
use std::env;

use std::vec::Vec;
use std::path::Path;
use clap::ArgMatches;
use rayon::prelude::*;
use lscolors::{ LsColors, Style };

use crate::file::*;
use crate::outp::*;
use crate::glyf::*;

const E: char = 27 as char;

// get listing of contents of this
// directory
fn tree(
            matches: &ArgMatches, 
            directory: &str, 
            prefix: &str, 
            mut treestat: &mut TreeStatistics,
            depth: u64
       ) -> result::Result<(), io::Error> 
{
    // walk file tree
    let mut things: Vec<_> = fs::read_dir(&*directory)?.map(|thing| {
        thing.unwrap().path()
    }).collect();
    let mut index = things.len();

    // don't sort if told not to
    if ! matches.is_present("nosort") {
        // sort these paths via rayon
        things.par_sort_unstable_by(|a, b| {
            let aname = a.file_name().unwrap().to_str().unwrap();
            let bname = b.file_name().unwrap().to_str().unwrap();
            aname.cmp(bname)
        });
    }

    // formatting
    let lscolors = LsColors::from_env().unwrap_or_default();

    // iter over paths and display 
    for thing in things {
        // skip this thing if it's hidden and --all is not set
        if ! matches.is_present("all") {
            if is_hidden(&thing) {
                continue;
            }
        }
        
        let is_dir: bool = thing.is_dir();
        let mut dirchar = " ";
       
        let path = thing.display().to_string().clone();
        let thing = thing.file_name().unwrap().to_str().unwrap();
        index = index - 1;

        // customize this iteration's str
        let current_branch_str;
        if index == 0 {
            current_branch_str = TreeChars::LastEntry.get();
        } else {
            current_branch_str = TreeChars::Entry.get();
        }
        
        // increment tree statistics and update dirchar
        if is_dir {
            dirchar = "/";
            treestat.directories += 1;
        } else {
            treestat.files += 1
        }


        // path formatting
        if ! matches.is_present("boring") {
            let style = lscolors.style_for_path(path)
                    .map(Style::to_ansi_term_style).unwrap_or_default();
            println!("{}{}{}{}{}[0m", prefix, current_branch_str,
                     style.paint(&*thing), dirchar, E);
        } else {
            println!("{}{}{}{}{}[0m", prefix, current_branch_str, thing, dirchar, E);
        }

        // maximum level
        let mut max_level: u64 = std::u64::MAX;
        if let Some(specified_max_level) = matches.value_of("level") {
            max_level = specified_max_level.parse::<u64>().unwrap();
        }

        // check if path is directory, and if so, 
        // recursively get contents
        if is_dir && (max_level != 0 && depth < max_level) {
            let newprefix;
            if index == 0 {
                newprefix = format!("{}{}", prefix, TreeChars::Blank.get());
            } else {
                newprefix = format!("{}{}", prefix, TreeChars::Line.get());
            }

            // use rayon to (possibly) execute this task in parallel
            rayon::scope(|s| {
                s.spawn(|_| {
                    tree(
                        matches,
                        &format!("{}/{}", directory, thing), 
                        &newprefix, 
                        &mut treestat,
                        depth + 1).unwrap();
                });
            });
        }
        debug(format!("found entry {}", thing));
    }
    debug("done".to_owned());
    Ok(())
}

pub fn branch(matches: &ArgMatches) {
    let mut directory: String = env::current_dir()
        .unwrap()
        .display()
        .to_string();

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

    // print directory
    println!("{}[1;34m{}{}[0m/", E, directory, E);

    // add / to directory
    if directory.chars().last().unwrap() != '/' {
        directory = directory + "/";
    }

    // init tree statistics
    let mut treestat = TreeStatistics { directories: 0, files: 0 };

    // print everything
    let result = tree(matches, &directory.clone(), "", &mut treestat, 0);

    // match errors, just in case
    match result {
        Ok(()) => println!("\n{} directories, {} files", treestat.directories, treestat.files),
        Err(err) => error(format!(" {:?}", err)),
    }
}
