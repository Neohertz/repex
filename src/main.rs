use std::{fs::{self, read_dir}, io::{stdin, stdout, Write}, path::{Path, PathBuf}, process::exit};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to search
    #[arg(short, long)]
    dir: std::path::PathBuf,

    /// Extension to replace
    #[arg(short, long)]
    old: String,

    /// New Extension
    #[arg(short, long)]
    new: String,

    /// Replace without warning.
    #[arg(short, long, default_value_t = false)]
    force: bool
}

fn get_files_with_ext(vec: &mut Vec<PathBuf>, ext: &String, path: &Path) {
    if path.is_dir() {
        let paths = read_dir(&path).expect("Failed to read file?");
        
        for res in paths {
            let sub_path = res.unwrap().path();

            if sub_path.is_dir() {
                get_files_with_ext(vec, ext, &sub_path)
            } else {
                let fs_ext = sub_path.extension();

                if fs_ext != None {
                    if fs_ext.unwrap().to_str().unwrap() == ext {
                        vec.push(sub_path)
                    }
                }
            
            }
        }
    }
}

/**
 * Ask the user if they want to continue. Returns a boolean.
 */
fn confirm(message: String) -> bool {
    let mut confirmation = String::new();
    print!("{}: ", message);
    let _ = stdout().flush();

    stdin().read_line(&mut confirmation).expect("Failed to read input.");
    confirmation = confirmation.trim().to_lowercase();

    match confirmation.as_ref() {
        "y" => true,
        _ => false
    }
}

fn main() {
    let args = Args::parse();

    let mut valid_paths = Vec::new();

    // Format old extensions.
    let old_ext = args.old.replace(".", "").trim().to_string();
    let new_ext = args.new.replace(".", "").trim().to_string();


    if !args.dir.exists() {
        println!("[Repex] Path '{}' does not exist.", args.dir.to_str().unwrap());
        exit(1);
    }

    get_files_with_ext(&mut valid_paths, &old_ext, args.dir.as_path());

    if valid_paths.len() < 1 {
        println!("[Repex] No files found with extension {}!", old_ext);
        exit(1);
    }

    let mut failures = 0;

    if confirm(format!("Are you sure? You will be replacing {} files! (y/N) ", &valid_paths.len())) {
        for val in &valid_paths {
            let new = val.with_extension(&new_ext);
            let result = fs::rename(val.as_path(), new.as_path());

            if result.is_err() {
                failures += 1;
            }
        }
        println!("[Repex] Successfully renamed {}/{} files.", (valid_paths.len() - failures), valid_paths.len());
    }
}