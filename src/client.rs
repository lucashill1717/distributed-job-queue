use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener
};

use crate::messages::Message;

#[tokio::main]
pub async fn client() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("localhost:20057").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            match socket.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    println!("Received {:?}", &buf[..n]);
                    let _ = socket.write_all(b"ACK").await;
                }
                _ => {}
            }
        });
    }
}
