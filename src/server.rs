use std::io::ErrorKind;
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering}
};

use bincode;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex}
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

/// Builds up job queue from XML data source. Sends one-page `Job`s into transmitter queue.
async fn queue_builder(tx: mpsc::Sender<Job>, source: String) -> std::io::Result<()> {
    let file = File::open(source).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut in_page = false;
    let mut buffer = String::new();
    while let Some(line) = lines.next_line().await? {
        if !in_page {
            if line.contains("<page>") {
                in_page = true;
                buffer.push_str(&line);
            }
        } else {
            buffer.push_str(&line);

            if line.contains("</page>") {
                in_page = false;

                let job_id: u32 = JOB_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                tx.send(Job::new(job_id, buffer.clone())).await.map_err(|why|
                    std::io::Error::new(ErrorKind::BrokenPipe, why))?;

                buffer.clear();
            }
        }
    }

    Ok(())
}

/// Pulls `Job`s from queue, sending them off to a client. Then handles further communication with client.
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
                    framed.send(encoded.into()).await;

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

/// Job producer. Creates jobs, sends out to clients, and collects returned information.
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
