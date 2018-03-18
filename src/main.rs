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
    /// Path containing files to be renamed
    #[structopt(short = "s", long = "source", parse(from_os_str))]
    source: PathBuf,
    /// Output directory, renamed files will be placed here
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
    /// Verbose output
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn rename_files(src_dir: &PathBuf, out_dir: &PathBuf, verbose: bool) {
    let re = Regex::new(r"(?i)-([a-z0-9-_]+)\.mp3").expect("Failed to compile regex");
    let file_entries = fs::read_dir(&src_dir).expect("Failed to read directory");

    for file_entry in file_entries {
        let filename = file_entry
            .expect("Failed to read file")
            .file_name()
            .into_string()
            .expect("Failed to convert filename to string");
        let newfile = Path::new(&out_dir).join(re.replace(&filename, ".mp3").as_ref());
        let originalfile = Path::new(&src_dir).join(&filename);
        if verbose {
            match fs::copy(&originalfile, &newfile) {
                Ok(_) => println!("Renamed {}", filename),
                Err(error) => panic!(
                    "Failed to copy {} to {:?} with error {:?}",
                    filename, newfile, error
                ),
            }
        } else {
            fs::copy(&originalfile, &newfile).expect("Failed to copy file");
        }
    }
}

fn main() {
    let args = Opt::from_args();
    let output_dir = args.output.unwrap_or(args.source.clone());
    rename_files(&args.source, &output_dir, args.verbose);
}
