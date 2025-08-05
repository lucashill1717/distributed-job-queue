use bincode;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages;

#[derive(Deserialize)]
pub struct ClientInfo {
    pub server_name: String
}

#[tokio::main]
pub async fn client(info: ClientInfo) -> std::io::Result<()> {
    let stream = TcpStream::connect(format!("{}:20057", info.server_name)).await?;
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    let ready = messages::Message::Ready(messages::Ready::new(8));
    let encoded: Vec<u8> = bincode::serialize(&ready).unwrap();
    framed.send(encoded.into()).await.unwrap();

    Ok(())
}
