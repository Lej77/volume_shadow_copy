//! This small CLI program locks a file and prevents it from being read or
//! written to by other programs.
//!
//! The program will fail to lock a file if the file is already locked.

use std::{env, fs::OpenOptions, io::stdin, os::windows::fs::OpenOptionsExt, path::PathBuf};

fn main() {
    let file_path = PathBuf::from(
        env::args_os()
            .nth(1)
            .expect("first argument should be a file path"),
    );
    let should_lock = if let Some(second_arg) = env::args_os().nth(2) {
        if second_arg != "--shared" {
            panic!("the second argument can only be --shared");
        }
        false
    } else {
        true
    };

    let mut options = OpenOptions::new();
    options.read(true);
    if should_lock {
        // No sharing:
        options.share_mode(0);
    }
    let file = options.open(&file_path).expect("failed to lock file");

    if should_lock {
        println!("Locked file at {}", file_path.display());
    } else {
        println!("Opened file in shared mode at {}", file_path.display());
    }
    println!();
    println!("Press enter to release the locked file and exit");
    let _ = stdin().read_line(&mut String::new());

    drop(file);
}
