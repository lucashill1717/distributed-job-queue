use bincode;
use num_cpus;
use unicode_segmentation::UnicodeSegmentation;
use serde::Deserialize;
use serde_json::Value;

use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::TcpStream, thread
};

use crate::messages::*;

#[derive(Deserialize)]
pub struct ClientInfo {
    pub server_name: String
}

/// Sends messages::Message with length-delimited encoding.
fn send_message(stream: &mut TcpStream, message: Message) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&message).map_err(|why|
        std::io::Error::new(ErrorKind::InvalidData, why))?;
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

fn get_link_frequencies(data: &String) -> Value {
    let mut map = HashMap::<String, u8>::new();
    let mut buf = String::new();

    let mut first_bracket = false;
    let mut second_bracket = false;
    for c in data.graphemes(true) {
        if c == "[" {
            if !first_bracket { first_bracket = true }
            else if !second_bracket { second_bracket = true }
        }
        if first_bracket && second_bracket {
            if c == "#" || c == "|" || c == "]" {
                *map.entry(buf.clone()).or_insert(1) += 1;
                buf.clear();

                first_bracket = false;
                second_bracket = false;
            }
            else { buf.push_str(c) }
        }
    }

    return serde_json::to_value(map).unwrap();
}

fn process_actions(task: Task) -> (u32, ActionResult) {
    let mut out = ActionResult::new();
    for action in task.actions {
        out.insert(action,  match action {
            Action::LinkFrequencies => get_link_frequencies(&task.data),
            _ => Value::Null
        });
    }
    return (task.id, out);
}

fn process_tasks(tasks: Vec::<Task>, cpu_count: usize) -> HashMap::<u32, ActionResult> {
    let chunk_size = (tasks.len() + cpu_count - 1) / cpu_count;
    let mut handles: Vec<thread::JoinHandle<Vec<(u32, ActionResult)>>> = Vec::new();

    for chunk in tasks.chunks(chunk_size) {
        let chunk_vec = chunk.to_vec();
        let handle = thread::spawn(move || {
            chunk_vec.into_iter().map(|task| process_actions(task)).collect::<Vec<(u32, ActionResult)>>()
        });
        handles.push(handle);
    }

    let mut result = HashMap::<u32, ActionResult>::new();
    for handle in handles {
        for tuple in handle.join().unwrap() {
            result.insert(tuple.0, tuple.1);
        }
    }

    return result;
}

/// Job consumer. Pulls jobs from server, spins up worker threads for processing, then reports back.
pub fn client(info: ClientInfo) -> std::io::Result<()> {
    let cpu_count = num_cpus::get();
    let addr = format!("{}:20057", info.server_name);

    let mut stream = TcpStream::connect(addr)?;

    let ready = Message::Ready(Ready::new(cpu_count as u8));
    send_message(&mut stream, ready)?;

    let mut tasks = Vec::<Task>::with_capacity(cpu_count as usize);
    for _ in 0..cpu_count {
        let message = read_message(&mut stream)?;
        match message {
            Message::Task(task) => tasks.push(task),
            _ => {}
        }
    }

    let results = process_tasks(tasks, cpu_count);
    let done= Message::Done(Done::new(results));
    send_message(&mut stream, done)?;

    Ok(())
}
