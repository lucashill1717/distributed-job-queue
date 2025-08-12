use std::sync::atomic::{AtomicU32, Ordering};

use bincode;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages;

static JOB_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Deserialize, Debug)]
pub struct ServerInfo {
    pub source: String,
    pub actions: Vec<messages::Action>
}

#[tokio::main]
pub async fn server(info: ServerInfo) -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:20057").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(bytes)) = framed.next().await {
                let message = bincode::deserialize::<messages::Message>(&bytes).unwrap();
                match message {
                    messages::Message::Ready(ready) => {
                        println!("Ready message received: {ready:?}");
                        let job_id: u32 = JOB_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                        let job = messages::Message::Job(messages::Job::new(job_id, "".to_string(), vec![]));
                        let encoded: Vec<u8> = bincode::serialize(&job).unwrap();
                        framed.send( encoded.into()).await;
                    }
                    messages::Message::Done(done) => {
                        println!("Done message received: {done:?}");
                    }
                    _ => {}
                }
            }
        });
    }
}
