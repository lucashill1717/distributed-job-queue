use bincode;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages;

#[tokio::main]
pub async fn client() -> std::io::Result<()> {
    let stream = TcpStream::connect("localhost:20057").await?;
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    let start = messages::Message::Start(messages::Start { tasks: vec![], source: "".to_string() });
    let encoded: Vec<u8> = bincode::serialize(&start).unwrap();
    framed.send(encoded.into()).await.unwrap();

    Ok(())
}
