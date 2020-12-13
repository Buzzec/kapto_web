use std::env::var;
use std::fs::read_dir;
use std::process::Command;

const WEBSITE_DIR: &str = "./website";

fn main() {
    add_dir_to_watch(WEBSITE_DIR);

    let command = if var("PROFILE").unwrap() == "release" {
        "npm run build:prod"
    } else {
        "npm run build:dev"
    };
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
    eprintln!("{:?}", output);
}

fn add_dir_to_watch(dir: &str) {
    println!("cargo:rerun-if-changed={}", dir);
    let directory = read_dir(dir).unwrap_or_else(|e| panic!("Cannot find directory {}, {}", dir, e));
    for entry in directory {
        let entry = entry.expect("Unexpected error");
        if entry.file_type().expect("Could not get file type").is_dir() {
            add_dir_to_watch(entry.path().to_str().expect("Invalid unicode"));
        } else {
            println!("cargo:rerun-if-changed={}", entry.path().to_str().expect("Invalid unicode"));
        }
    }
}
