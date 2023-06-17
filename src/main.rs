use clap::Parser;
use regex::Regex;
use std::env::current_dir;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct FindArgs {
    /// Directories to search through
    #[arg(short = 'd', long = "dir", required = true)]
    directories: Vec<String>,

    /// A regex pattern to search for in the given directories
    #[arg(short = 'm', long = "match", required = true)]
    patterns: Vec<String>,

    /// File to write the output to. Default is stdout
    #[arg(short = 'o', long = "output")]
    output_file: Option<String>,

    /// Minimum file size in bytes to search for
    #[arg(short = 's', long = "size")]
    file_size: Option<u64>,

    /// Search for directories matching pattern as well as files
    #[arg(short = 'a', long = "all")]
    all: Option<bool>,

    /// Only search for patterns including this special character
    #[arg(short = 'c', long = "char")]
    special_character: Option<char>,

    /// How many levels of folders to search
    #[arg(short = 'l', long = "level")]
    nesting_depth: Option<i32>,

    /// Specific file type to search for
    #[arg(short = 't', long = "type")]
    file_type: Option<String>,

    /// Only search directories and files that meet a specific permissions level
    #[arg(short = 'p', long = "perms")]
    permissions: Option<String>,
}

fn main() {
    let mut current_dir = current_dir().unwrap();
    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        print!("{}> ", current_dir.to_string_lossy());
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string(); // Trim trailing newline characters

        let mut parts = input.split(" ");
        let command = parts.next();

        if let Some(first) = command {
            match first {
                "find" => find(FindArgs::parse_from(parts)),
                "cd" => current_dir = cd(current_dir, parts.next()).clone(),
                "ls" => ls(&current_dir),
                _ => continue,
            }
        } else {
            continue;
        }
    }
}

fn cd(current_dir: PathBuf, arg: Option<&str>) -> PathBuf {
    if let Some(directory) = arg {
        if directory.starts_with("C:\\") {
            return PathBuf::from(directory);
        } else if directory == ".." {
            let mut new_dir = current_dir.clone();
            new_dir.pop();
            return new_dir;
        } else {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_type().unwrap().is_file() {
                            println!("Cannot cd into a file");
                        } else if entry.file_type().unwrap().is_dir() {
                            if entry.file_name() == directory {
                                return PathBuf::from(entry.path());
                            }
                        }
                    }
                }
            } else {
                panic!("Failed to open {:?}", &current_dir);
            }
        }
    }

    assert!(false); // Big problems if reaches here.
    return PathBuf::new();
}

fn ls(current_dir: &PathBuf) {
    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_str().unwrap());
            }
        }
    }
}

fn find(args: FindArgs) {
    for directory in &args.directories {
        for pattern in &args.patterns {
            search_directory(
                PathBuf::from(directory.clone()),
                &Regex::new(&pattern).unwrap(),
            );
        }
    }
}

fn search_directory(path: PathBuf, pattern: &Regex) {
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_file() {
                    if pattern.is_match(entry.path().to_str().unwrap()) {
                        println!("{}", &entry.path().display());
                    }
                } else if entry.file_type().unwrap().is_dir() {
                    if pattern.is_match(entry.path().to_str().unwrap()) {
                        println!("{}", &entry.path().display());
                    }
                    search_directory(PathBuf::from(entry.path()), &pattern)
                }
            }
        }
    } else {
        panic!("Failed to open {:?}", &path);
    }
}
