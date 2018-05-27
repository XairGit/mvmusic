#[macro_use]
extern crate structopt;
extern crate regex;
#[macro_use]
extern crate log;
extern crate simplelog;

use regex::Regex;
use simplelog::{Config, LevelFilter, TermLogger};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mvmusic", about = "A small utility to copy/move files containing appended youtube URLs")]
struct Opt {
    /// Path containing files to be renamed
    #[structopt(short = "s", long = "source", parse(from_os_str))]
    source: PathBuf,
    /// Optional output directory, if specified renamed files will be placed here
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
    /// Show less output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Delete original files after renaming them
    #[structopt(short = "d", long = "delete")]
    delete: bool,
}

fn main() {
    let args = Opt::from_args();
    if args.quiet {
        TermLogger::init(LevelFilter::Warn, Config::default())
            .expect("Failed to initialize logger");
    } else {
        TermLogger::init(LevelFilter::Info, Config::default())
            .expect("Failed to initialize logger");
    }
    // Rust currently doesn't support partial moves across closures
    // so for now this will stay strictly evaluated
    // in future it would be better to use unwrap_or_else() here
    let output_dir = args.output.unwrap_or(args.source.clone());
    let re = Regex::new(r"(?i)-([a-z0-9-_]+)\.mp3").expect("Failed to compile regex");
    let file_entries = fs::read_dir(&args.source).expect("Failed to read directory");

    for file_entry in file_entries {
        let filename = file_entry
            .expect("Failed to read file")
            .file_name()
            .into_string()
            .expect("Failed to convert filename to string");
        // regex replace method will return original filename if no match is found
        // which results in accidental copying of files
        if !re.is_match(&filename) {
            continue;
        }
        let newfile = Path::new(&output_dir).join(re.replace(&filename, ".mp3").as_ref());
        let originalfile = Path::new(&args.source).join(&filename);
        match fs::copy(&originalfile, &newfile) {
            Ok(_) => info!("Renamed {}", filename),
            Err(error) => panic!(
                "Failed to copy {:?} to {:?} with error {:?}",
                originalfile, newfile, error
            ),
        }
    }
}
