#[macro_use]
extern crate structopt;
extern crate regex;

use std::fs;
use std::path::PathBuf;
use std::path::Path;
use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rnmusic", about = "A small utility to strip youtube URLs from filenames")]
struct Opt {
    /// Path to search for files
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: PathBuf,
}

fn rename_files(src_path: PathBuf) {
    let re = Regex::new(r"(?i)-([a-z0-9-_]+)\.mp3").expect("Failed to compile regex");
    let file_entries = match fs::read_dir(&src_path) {
        Ok(file_entries) => file_entries,
        Err(error) => panic!("Error: Failed to open directory ({:?})", error.kind()),
    };

    for file_entry in file_entries {
        let file = match file_entry {
            Ok(file) => file,
            Err(error) => panic!(
                "Error: Failed to read directory contents ({:?})",
                error.kind()
            ),
        };
        let filename = match file.file_name().into_string() {
            Ok(filename) => filename,
            Err(os_string) => panic!(
                "Error: Failed to convert filename ({:?}) to string",
                os_string
            ),
        };
        let newfile = Path::new(&src_path)
            .join(
                re.replace(&filename, ".mp3").to_string()
            );
        let originalfile = Path::new(&src_path).join(&filename);
        match fs::copy(&originalfile, &newfile) {
            Ok(_) => println!("Renamed {}", filename),
            Err(error) => panic!("Failed to copy {} to {:?} with error {:?}", filename, newfile, error),
        }
    }
}

fn main() {
    let args = Opt::from_args();
    rename_files(args.path);
}
