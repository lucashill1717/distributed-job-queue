use serde::Deserialize;

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

pub fn client(info: ClientInfo) -> std::io::Result<()> {
    Ok(())
}

// #[tokio::main]
// pub async fn client(info: ClientInfo) -> std::io::Result<()> {
//     let worker_count = Handle::current().metrics().num_workers();
//     let (job_tx, job_rx) = channel::<messages::Message>(worker_count);
//     let (result_tx, result_rx) = channel::<messages::Message>(worker_count);

//     let stream = TcpStream::connect(format!("{}:20057", info.server_name)).await?;
//     let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

//     loop {
//         let live_task_count = Handle::current().metrics().num_alive_tasks() as u8;
//         let ready = messages::Message::Ready(messages::Ready::new(worker_count as u8 - live_task_count));
//         let encoded: Vec<u8> = bincode::serialize(&ready).unwrap();
//         framed.send(encoded.into()).await?;
//         break;
//     }

//     Ok(())
// }
