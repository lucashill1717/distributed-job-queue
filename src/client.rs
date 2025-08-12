use bincode;
use num_cpus;
use serde::Deserialize;

use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::TcpStream
};

use crate::messages::*;

#[derive(Deserialize)]
pub struct ClientInfo {
    pub server_name: String
}

/// Sends messages::Message with length-delimited encoding.
fn send_message(stream: &mut TcpStream, message: Message) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&message)
        .map_err(|why| std::io::Error::new(ErrorKind::InvalidData, why))?;
    let length: [u8; 4] = (encoded.len() as u32).to_be_bytes();

    stream.write_all(&length)?;
    stream.write_all(&encoded)?;

    Ok(())
}

/// Reads stream into messages::Message with length-delimited encoding.
fn read_message(stream: &mut TcpStream) -> std::io::Result<Message> {
    let mut length_buf = [0u8; 4];
    stream.read_exact(&mut length_buf)?;

    let length = u32::from_be_bytes(length_buf);
    let mut message_buf: Vec<u8> = vec![0u8; length as usize];
    stream.read_exact(&mut message_buf)?;

    match bincode::deserialize::<Message>(&message_buf) {
        Ok(message) => Ok(message),
        Err(why) => Err(std::io::Error::new(ErrorKind::InvalidData, why))
    }
}

pub fn client(info: ClientInfo) -> std::io::Result<()> {
    let cpu_count = num_cpus::get() as u8;
    let addr = format!("{}:20057", info.server_name);

    let mut stream = TcpStream::connect(addr)?;
    let ready = Message::Ready(Ready::new(cpu_count));
    send_message(&mut stream, ready)?;

    let message = read_message(&mut stream)?;
    match message {
        Message::Job(job) => {
            println!("Job received: {:?}", job);
        }
        _ => {}
    }

    let map = HashMap::<u32, HashMap::<Action, String>>::new();
    let done= Message::Done(Done::new(map));
    send_message(&mut stream, done)?;

    Ok(())
}
