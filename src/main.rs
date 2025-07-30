use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;

mod client;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.get(1) {
        None => {
            println!("Running instance as client.");
            client::client();
            return;
        }
        Some(file_name) => {
            println!("Running instance as server.");
            file_name
        }
    };

    let path: &Path = Path::new(file_name);
    let mut file: File = match File::open(&path) {
        Err(why) => {
            eprintln!("Couldn't open {}: {}", path.display(), why);
            process::exit(1);
        }
        Ok(file) => file
    };

    let mut file_contents: String = String::new();
    if let Err(why) = file.read_to_string(&mut file_contents) {
        eprintln!("Couldn't read from {}: {}", path.display(), why);
        process::exit(2);
    }

    let user_input: server::UserInput = match serde_json::from_str(&file_contents) {
        Err(why) => {
            eprintln!("Parsing input {} failed: {}", path.display(), why);
            process::exit(3);
        }
        Ok(user_input) => user_input
    };

    server::server(user_input);
}
