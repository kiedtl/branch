use std::fs;
use std::io;
use std::env;
use std::result;
use std::vec::Vec;
use std::path::Path;

use clap::ArgMatches;
use rayon::prelude::*;

use crate::file::*;
use crate::outp::*;

// get listing of contents of this
// directory
fn tree(
            matches: &ArgMatches,
            directory: &str,
            mut treestat: &mut TreeStatistics,
            depth: u64,
       ) -> result::Result<(), io::Error>
{
    // walk file tree
    let mut things: Vec<_> = fs::read_dir(&*directory)?.map(|e| {
        e.unwrap().path()
    }).collect();

    // don't sort if told not to
    if matches.is_present("sort") {
        // sort these paths via rayon
        things.par_sort_unstable_by(|a, b| {
            let aname = a.file_name().unwrap().to_str().unwrap();
            let bname = b.file_name().unwrap().to_str().unwrap();
            aname.cmp(bname)
        });
    }

    // iter over paths and display
    for thing in things {
        // skip this thing if it's hidden and --all is not set
        if ! matches.is_present("all") {
            if is_hidden(&thing) {
                continue;
            }
        }

        let is_dir: bool = thing.is_dir();
        let path = thing.display().to_string().clone();
        let thing = thing.file_name().unwrap().to_str().unwrap();

        // increment tree statistics
        if is_dir {
            treestat.directories += 1;
        } else {
            if ! matches.is_present("dirs") {
                treestat.files += 1
            }
        }

        // don't print stuff if --count is specified
        if ! matches.is_present("count") {
            if matches.is_present("dirs") && !is_dir { } else {
                println!("{}", path);
            }
        }

        // maximum level
        let mut max_level: u64 = std::u64::MAX;
        if let Some(specified_max_level) = matches.value_of("level") {
            max_level = specified_max_level.parse::<u64>().unwrap();
        }

        // check if path is directory, and if so,
        // recursively get contents
        if is_dir && (max_level != 0 && depth < max_level) {
            // use rayon to (possibly) execute this task in parallel
            rayon::scope(|s| {
                s.spawn(|_| {
                    tree(
                        matches,
                        &format!("{}/{}", directory, thing),
                        &mut treestat,
                        depth + 1).unwrap();
                });
            });
        }
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
    if !matches.is_present("count") {
        println!("{}/", directory);
    }

    // init tree statistics
    let mut treestat = TreeStatistics { directories: 0, files: 0 };

    // print everything
    let result = tree(matches, &directory.clone(), &mut treestat, 0);

    // match errors, just in case
    match result {
        Ok(()) => {
            if matches.is_present("count") {
                if matches.is_present("dirs") {
                    treestat.print_dirs();
                } else {
                    treestat.print_all();
                }
            }
        }, 
        Err(err) => error(format!(" {:?}", err)),
    }
}

