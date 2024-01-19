use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

async fn handle_connection(socket: &mut TcpStream) -> Result<()> {
    let mut buf = Vec::new();
    socket
        .read_to_end(&mut buf)
        .await
        .context("CTX: handle connection read buffer")?;
    let req_data = String::from_utf8_lossy(&buf[..]);

    let response = if req_data.starts_with("GET / HTTP/1.1") {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };

    socket
        .write_all(response.as_bytes())
        .await
        .context("CTX: write connection response")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")
        .await
        .expect("Failed to bind to address");

    loop {
        if let Ok((mut socket, _)) = listener.accept().await {
            spawn(async move {
                if let Err(e) = handle_connection(&mut socket).await {
                    eprintln!("Error handling connection: {:?}", e);
                }
            });
        }
    }
}
