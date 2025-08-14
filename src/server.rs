use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering}
};

use bincode;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, mpsc}
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages::*;

static JOB_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Deserialize)]
pub struct ServerInfo {
    pub source: String,
    pub actions: Vec<Action>
}

struct Job {
    job_id: u32,
    data: String
}

impl Job {
    fn new(job_id: u32, data: String) -> Job {
        Job { job_id, data }
    }
}

async fn queue_builder(tx: mpsc::Sender<Job>, source: String) {
    loop {
        let job_id: u32 = JOB_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        tx.send(Job::new(job_id, "".to_string())).await;
    }
}

async fn thread_runner(rx: Arc<Mutex<mpsc::Receiver<Job>>>, stream: TcpStream, cloned_actions: Arc<Vec<Action>>) {
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
    while let Some(Ok(bytes)) = framed.next().await {
        let message = bincode::deserialize::<Message>(&bytes).unwrap();
        match message {
            Message::Ready(ready) => {
                println!("Ready message received: {ready:?}");
                let mut job_count: u8 = 0;
                while let Some(job) = rx.lock().await.recv().await {
                    let task = Message::Task(Task::new(job.job_id, job.data, cloned_actions.to_vec()));
                    let encoded: Vec<u8> = bincode::serialize(&task).unwrap();
                    framed.send( encoded.into()).await;

                    job_count += 1;
                    if job_count == ready.task_count { break }
                }
            }
            Message::Done(done) => {
                println!("Done message received: {done:?}");
                break;
            }
            _ => {}
        }
    }
}

#[tokio::main]
pub async fn server(info: ServerInfo) -> std::io::Result<()> {
    let actions = Arc::new(info.actions);
    let (tx, rx) = mpsc::channel::<Job>(500);
    let rx = Arc::new(Mutex::new(rx));

    tokio::spawn(async move {
        queue_builder(tx, info.source).await;
    });

    let listener = TcpListener::bind("0.0.0.0:20057").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let cloned_actions = Arc::clone(&actions);
        let rx = Arc::clone(&rx);

        tokio::spawn(async move {
            thread_runner(rx, stream, cloned_actions).await;
        });
    }
}
