use bincode;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::{
    net::TcpListener,
    spawn
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages;

#[derive(Deserialize)]
pub struct ServerInfo {
    pub source: String,
    pub actions: Vec<String>
}

#[tokio::main]
pub async fn server(info: ServerInfo) -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:20057").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        spawn(async move {
            let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(bytes)) = framed.next().await {
                let message: messages::Message = bincode::deserialize(&bytes).unwrap();
                match message {
                    messages::Message::Start(start) => {
                        println!("Start message received: {:?}", start)
                    }
                    _ => {}
                }
            }
        });
    }
}
