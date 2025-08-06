use bincode;
use num_cpus;
use serde::Deserialize;

use std::{
    io::Write,
    net::TcpStream
};

use crate::messages;

#[derive(Deserialize)]
pub struct ClientInfo {
    pub server_name: String
}

// Convert to simple thread based.
// Get CPU count
// loop {
//   Request #CPU tasks
//   Process tasks with standard library threads
//   Send back accumulated results
// }

/// Sends messages::Message with length-delimited encoding.
fn send_message(stream: &mut TcpStream, message: messages::Message) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&message).unwrap();
    let length: [u8; 4] = (encoded.len() as u32).to_be_bytes();

    stream.write(&length)?;
    stream.write_all(&encoded)?;

    Ok(())
}

pub fn client(info: ClientInfo) -> std::io::Result<()> {
    let cpu_count = num_cpus::get() as u8;
    let addr = format!("{}:20057", info.server_name);

    let mut stream = TcpStream::connect(addr)?;
    let ready = messages::Message::Ready(messages::Ready::new(cpu_count));
    send_message(&mut stream, ready)?;

    Ok(())
}
