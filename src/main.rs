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
    let mut show_hidden = false;

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
                "find" => {
                    let result = FindArgs::try_parse_from(parts);
                    let args = match result {
                        Ok(args) => args,
                        Err(_error) => {
                            println!("Please specify valid arguments for find");
                            continue;
                        }
                    };
                    find(args);
                }
                "cd" => cd(&mut current_dir, parts.next(), show_hidden),
                "ls" => ls(&current_dir, show_hidden),
                "show" => show_hidden = true,
                "hide" => show_hidden = false,
                "-help" => println!(""), // Fill this out
                "exit" => {
                    println!("Process terminated");
                    return;
                }
                _ => println!("Please specify a valid command. Use -help for a list of commands"),
            }
        } else {
            continue;
        }
    }
}

fn cd(current_dir: &mut PathBuf, arg: Option<&str>, show_hidden: bool) {
    if let Some(directory) = arg {
        if directory.starts_with("C:\\") {
            *current_dir = PathBuf::from(directory);
            return;
        } else if directory == ".." {
            current_dir.pop();
            return;
        } else {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_name() == directory {
                            if !show_hidden
                                && entry.file_name().to_string_lossy().chars().nth(0) == Some('.')
                            {
                                continue;
                            }
                            if entry.file_type().unwrap().is_file() {
                                println!("Cannot cd into a file");
                            } else if entry.file_type().unwrap().is_dir() {
                                *current_dir = entry.path();
                                return;
                            }
                        }
                    }
                }
            } else {
                println!("Failed to open {:?}", &current_dir);
            }
        }
    }
    println!("Please specifiy a valid directory to move to")
}

fn ls(current_dir: &PathBuf, show_hidden: bool) {
    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if !show_hidden && entry.file_name().to_string_lossy().chars().nth(0) == Some('.') {
                    continue;
                }
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
