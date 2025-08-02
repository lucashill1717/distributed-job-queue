use bincode;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages;

#[tokio::main]
pub async fn client() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:20057").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

            while let Some(Ok(bytes)) = framed.next().await {
                let message: messages::Message = bincode::deserialize(&bytes).unwrap();
                match message {
                    messages::Message::Start(start) => {
                        println!("Start message received: {:?}", start)
                    }
                    _ => {}
                }

                let done = messages::Message::Done(messages::Done::new(vec!["done :)".to_string()]));
                let encoded = bincode::serialize(&done).unwrap();
                let _ = framed.send(encoded.into()).await;
            }
        });
    }
}
