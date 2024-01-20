use anyhow::{Context, Result};
use http_server_starter_rust::http::{
    content_type::ContentType, method::Method, status::Status, version::Version,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

use http_server_starter_rust::request::Request;
use http_server_starter_rust::response::Response;

const EMPTY_RESPONSE: Response = Response {
    status: Status::Ok,
    version: Version::V1_1,
    content: String::new(),
    content_length: 0,
    content_type: ContentType::TextPlain,
};

async fn handle_connection(socket: &mut TcpStream) -> Result<()> {
    let mut buf = [0u8; 1024]; // arbitrary buffer size
    socket
        .read(&mut buf)
        .await
        .context("CTX: handle connection read buffer")?;

    let request =
        Request::from_byte_array(&buf).context("CTX: could not parse Request from byte buffer")?;

    let response: Response = match request.method {
        Method::Get => EMPTY_RESPONSE,
        _ => EMPTY_RESPONSE,
    };

    let response_string: String = response.try_into()?;
    socket
        .write_all(response_string.as_bytes())
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
