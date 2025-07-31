use bincode;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages;

#[derive(Deserialize)]
pub struct UserInput {
    source: String,
    actions: Vec<String>,
    // hosts: Vec<String>
}

#[tokio::main]
pub async fn server(user_input: UserInput) -> std::io::Result<()> {
    let stream: TcpStream = TcpStream::connect("localhost:20057").await?;
    let mut framed: Framed<TcpStream, LengthDelimitedCodec> = Framed::new(stream, LengthDelimitedCodec::new());

    let start: messages::Start = messages::Start::new(user_input.actions, user_input.source);
    let encoded: Vec<u8> = bincode::serialize(&start).unwrap();
    framed.send(encoded.into()).await.unwrap();

    if let Some(Ok(bytes)) = framed.next().await {
        let done: messages::Done = bincode::deserialize(&bytes).unwrap();
        println!("Done message received: {:?}", done);
    }

    Ok(())
}
