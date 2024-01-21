use std::env;
use std::ops::Deref;
use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::{fs, spawn};

use http_server_starter_rust::http::path::Path;
use http_server_starter_rust::request::Request;
use http_server_starter_rust::response::Response;

async fn handle_connection(socket: &mut TcpStream, file_directory: &Option<String>) -> Result<()> {
    let mut buf = [0u8; 1024]; // arbitrary buffer size
    socket
        .read(&mut buf)
        .await
        .context("CTX: handle connection read buffer")?;

    println!(
        "handle connection file dir {file_directory:?} {}",
        file_directory.is_some()
    );

    let request = Request::from_byte_array(&buf);
    let response: Response = match request {
        Ok(req) => match req.path {
            Path::Empty => Response::ok_str("".to_owned()),
            Path::UserAgent => Response::ok_str(req.agent),
            Path::Echo => {
                let content = String::from(match req.full_path.starts_with('/') {
                    true => req.full_path[1..].split_once('/').unwrap_or_default().1,
                    false => req.full_path.split_once('/').unwrap_or_default().1,
                });
                Response::ok_str(content)
            }
            Path::Files => match file_directory {
                Some(directory) => {
                    let file_directory = format!(
                        "{}/{}",
                        directory,
                        req.full_path[1..].split_once('/').unwrap().1
                    );
                    let metadata = fs::metadata(&file_directory).await;
                    if metadata.is_ok() && metadata.unwrap().is_file() {
                        let content = fs::read(&file_directory).await?;
                        Response::ok_bin(content)
                    } else {
                        eprintln!("NOT FOUND 1");
                        Response::not_found_bin()
                    }
                }
                None => {
                    eprintln!("NOT FOUND 2");
                    Response::not_found_str()
                }
            },
        },
        Err(e) => {
            eprintln!("error {}", e);
            eprintln!("NOT FOUND 3");
            Response::not_found_str()
        }
    };
    eprintln!("Response: {response}");

    // i couldve done this nicer but eh
    let response_bytes: Vec<u8> = if file_directory.is_some() {
        response.into()
    } else {
        response.to_string().as_bytes().into()
    };

    socket
        .write_all(&response_bytes)
        .await
        .context("CTX: write connection response")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_directory = args
        .iter()
        .position(|arg| arg == "--directory")
        .and_then(|index| args.get(index + 1).cloned());

    let shared_file_directory = Arc::new(file_directory);

    let listener = TcpListener::bind("127.0.0.1:4221")
        .await
        .expect("Failed to bind to address.");

    loop {
        if let Ok((mut socket, _)) = listener.accept().await {
            let directory = Arc::clone(&shared_file_directory);
            spawn(async move {
                if let Err(e) = handle_connection(&mut socket, directory.deref()).await {
                    eprintln!("Error handling connection: {:?}", e);
                }
            });
        }
    }
}
