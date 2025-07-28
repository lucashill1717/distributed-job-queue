use serde::Deserialize;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

#[derive(Deserialize)]
struct UserInput {
    source: String,
    actions: Vec<String>,
    hosts: Vec<String>
}

fn main() {
    let args: Vec<String> = args().collect();
    let file_name: &String = match args.get(1) {
        None => {
            eprintln!("Job file required as first argument. Exiting.");
            exit(1);
        },
        Some(file_name) => file_name
    };

    let path: &Path = Path::new(file_name);
    let mut file: File = match File::open(&path) {
        Err(why) => {
            eprintln!("Couldn't open {}: {}", path.display(), why);
            exit(2);
        },
        Ok(file) => file
    };

    let mut file_contents: String = String::new();
    if let Err(why) = file.read_to_string(&mut file_contents) {
        eprintln!("Couldn't read from {}: {}", path.display(), why);
        exit(3);
    }

    let user_input: UserInput = serde_json::from_str(&file_contents);
}
