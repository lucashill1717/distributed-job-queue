use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInput {
    source: String,
    actions: Vec<String>,
    hosts: Vec<String>,
}

#[tokio::main]
pub async fn server(user_input: UserInput) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:20057").await?;
    stream.write_all(b"Hello").await?;

    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Response: {:?}", &buf[..n]);

    Ok(())
}
