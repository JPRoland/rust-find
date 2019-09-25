use regex::Regex;
use std::fs::{read_dir, ReadDir};
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let (re, path) = match parse_args(std::env::args().collect()) {
        Some((re, path)) => (re, path),
        None => std::process::exit(1),
    };

    let dirs = get_dirs(read_dir(path)?)?;
    let matches = get_matches(dirs, re);

    for mat in matches {
        println!("{}", mat.to_string_lossy());
    }

    Ok(())
}

fn parse_args(args: Vec<String>) -> Option<(Regex, PathBuf)> {
    if args.len() <= 2 || args.len() > 3 {
        eprintln!("Error: You must supply two arguments.");
        return None;
    }

    let re = match Regex::new(&args[1]) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };

    let path = PathBuf::from(&args[2]);
    if !path.is_dir() {
        eprintln!("Path is not a directory");
        return None;
    }

    Some((re, path))
}

fn get_dirs(dir: ReadDir) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            result.extend(get_dirs(read_dir(path)?)?.iter().cloned());
        } else {
            result.push(path);
        }
    }

    Ok(result)
}

fn get_matches(paths: Vec<PathBuf>, pattern: Regex) -> Vec<PathBuf> {
    let mut result = vec![];

    for path in paths {
        if pattern.is_match(&path.to_string_lossy()) {
            result.push(path);
        }
    }

    result
}
