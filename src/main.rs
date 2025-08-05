use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::Read,
    path::Path,
    process::ExitCode
};

mod client;
mod messages;
mod server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    run_type: Option<RunTypes>
}

#[derive(Subcommand)]
enum RunTypes {
    /// Run the program as a client, pulling jobs from server. Requires a specified server.
    Client {
        /// Server name or IP address
        #[arg(short, long)]
        server: String
    },
    /** Run the program as the server, responding to clients with jobs and accumulating results.
    Requires a path to a job file. */
    Server {
        /// Job file path
        #[arg(short, long)]
        job_file: String
    }
}

fn main() -> ExitCode {
    let mut file_path: String = "".to_string();
    let new_args = Args::parse();
    match new_args.run_type {
        None => {}
        Some(RunTypes::Client { server }) => {
            println!("Running instance as client.");
            match client::client(client::ClientInfo { server_name: server }) {
                Err(why) => {
                    eprintln!("Client instance terminated unexpectedly: {}", why);
                    return ExitCode::FAILURE;
                }
                Ok(_) => return ExitCode::SUCCESS
            };
        }
        Some(RunTypes::Server { job_file }) => {
            println!("Running instance as server.");
            file_path = job_file;
        }
    }

    let path: &Path = Path::new(&file_path);
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

    let server_info: server::ServerInfo = match serde_json::from_str(&file_contents) {
        Err(why) => {
            eprintln!("Parsing input {} failed: {}", path.display(), why);
            return ExitCode::FAILURE;
        }
        Ok(server_info) => server_info
    };

    match server::server(server_info) {
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
