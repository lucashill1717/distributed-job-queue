use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::ExitCode;

mod client;
mod server;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.get(1) {
        None => {
            println!("Running instance as client.");
            match client::client() {
                Err(why) => {
                    eprintln!("Client instance terminated unexpectedly: {}", why);
                    return ExitCode::FAILURE;
                }
                Ok(_) => return ExitCode::SUCCESS
            };
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
            return ExitCode::FAILURE;
        }
        Ok(file) => file
    };

    let mut file_contents: String = String::new();
    if let Err(why) = file.read_to_string(&mut file_contents) {
        eprintln!("Couldn't read from {}: {}", path.display(), why);
        return ExitCode::FAILURE;
    }

    let user_input: server::UserInput = match serde_json::from_str(&file_contents) {
        Err(why) => {
            eprintln!("Parsing input {} failed: {}", path.display(), why);
            return ExitCode::FAILURE;
        }
        Ok(user_input) => user_input
    };

    match server::server(user_input) {
        Err(why) => {
            eprintln!("Server instance terminated unexpectedly: {}", why);
            return ExitCode::FAILURE;
        }
        Ok(_) => {
            println!("Server shutdown successfully.");
        }
    };

    return ExitCode::SUCCESS;
}
