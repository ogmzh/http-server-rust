use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

use http_server_starter_rust::http::path::Path;
use http_server_starter_rust::request::Request;
use http_server_starter_rust::response::Response;

async fn handle_connection(socket: &mut TcpStream) -> Result<()> {
    let mut buf = [0u8; 1024]; // arbitrary buffer size
    socket
        .read(&mut buf)
        .await
        .context("CTX: handle connection read buffer")?;

    let request = Request::from_byte_array(&buf);
    let response: Response = match request {
        Ok(req) => match req.path {
            Path::Empty => Response::ok("".to_owned()),
            Path::UserAgent => Response::ok(req.agent),
            Path::Echo => {
                let content = String::from(match req.full_path.starts_with('/') {
                    true => req.full_path[1..].split_once('/').unwrap_or_default().1,
                    false => req.full_path.split_once('/').unwrap_or_default().1,
                });
                Response::ok(content)
            }
        },
        Err(e) => {
            eprintln!("error {}", e);
            Response::not_found()
        }
    };
    eprintln!("Response: {response}");

    socket
        .write_all(response.to_string().as_bytes())
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
