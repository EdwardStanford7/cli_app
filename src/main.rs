use clap::Parser;
use regex::Regex;
use std::env::current_dir;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug, Default)]
#[command(no_binary_name(true))]
struct FindArgs {
    /// Directories to search through, default will search current directory.
    #[clap(short = 'd', long = "dir")]
    directories: Vec<String>,

    /// A regex pattern to search for in the given directories, default will match all files.
    #[clap(short = 'm', long = "match")]
    patterns: Vec<String>,

    /// File to write the output to, default is stdout.
    #[clap(short = 'o', long = "output")]
    output_file: Option<String>,

    /// Minimum file size in bytes to search for, default is 0.
    #[clap(short = 's', long = "size")]
    file_size: Option<u64>,

    /// Search for directories as well as files.
    #[clap(short = 'a', long = "all")]
    all: Option<bool>,

    /// How many levels of folders to search.
    #[clap(short = 'l', long = "level")]
    nesting_depth: Option<i32>,

    /// Only search directories and files that meet a specific permissions level
    #[clap(short = 'p', long = "perms")]
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
        let mut parts = input.split_whitespace();
        let command = parts.next();

        if let Some(first) = command {
            match first {
                "find" => {
                    if parts.next() == Some("-help") {
                        println!("Available flags for find:");
                        println!("\t-d, --dir <directories>: Directories to search through, default will search current directory");
                        println!("\t-m, --match <patterns>: A regex pattern to search for in the given directories, default will match all files");
                        println!("\t-o, --output <output_file>: File to write the output to, default is stdout");
                        println!("\t-s, --size <file_size>: Minimum file size in bytes to search for, default is 0");
                        println!("\t-a, --all: Search for directories as well as files");
                        println!(
                            "\t-l, --level <folder_level>: How many levels of folders to search"
                        );
                        println!("\t-p, --perms <permissions>: Only search directories and files that meet a specific permissions level");
                        continue;
                    }
                    let result = FindArgs::try_parse_from(parts);
                    let args = match result {
                        Ok(args) => args,
                        Err(_error) => {
                            println!("Please specify valid arguments for find");
                            continue;
                        }
                    };
                    find(&current_dir, args);
                }
                "cd" => cd(&mut current_dir, parts.next(), show_hidden),
                "ls" => ls(&current_dir, show_hidden),
                "show" => show_hidden = true,
                "hide" => show_hidden = false,
                "help" => {
                    println!("Available commands:");
                    println!("\tfind <file/directory>: Search for a file or directory \n\t\t use find -help for more details");
                    println!("\tcd <directory>: Change the current directory");
                    println!("\tls: List files and directories in the current directory");
                    println!("\tshow: Show hidden files and directories");
                    println!("\thide: Hide hidden files and directories");
                    println!("\thelp: Show this help menu");
                    println!("\texit: Terminate the process");
                }

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

fn find(current_dir: &PathBuf, args: FindArgs) {
    if args.directories.is_empty() {
        if args.patterns.is_empty() {
            search_directory(current_dir, &Regex::new(r".*").unwrap())
        } else {
            for pattern in &args.patterns {
                search_directory(current_dir, &Regex::new(&pattern).unwrap());
            }
        }
    } else {
        for directory in &args.directories {
            if args.patterns.is_empty() {
                search_directory(&PathBuf::from(directory), &Regex::new(r".*").unwrap())
            } else {
                for pattern in &args.patterns {
                    search_directory(&PathBuf::from(directory), &Regex::new(&pattern).unwrap());
                }
            }
        }
    }
}

fn search_directory(path: &PathBuf, pattern: &Regex) {
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
                    search_directory(&PathBuf::from(entry.path()), &pattern)
                }
            }
        }
    } else {
        panic!("Failed to open {:?}", &path);
    }
}
