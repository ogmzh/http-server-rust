use anyhow::{Context, Result};
use std::str::SplitWhitespace;
use thiserror::Error;

// now i see that my file structure leads to this redundancy... gah...
use crate::http::{method::Method, path::Path, version::Version};

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Unknown request content")]
    UnknownRequestContent,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: Path,
    pub full_path: String,
    pub version: Version,
    pub host: String,
    pub agent: String,
    pub body: Option<Vec<u8>>,
}

impl Request {
    fn parse_request_line(values: &mut SplitWhitespace) -> Result<(Method, String, Path, Version)> {
        let method = values
            .next()
            .ok_or(RequestError::UnknownRequestContent)?
            .try_into()?;
        let full_path: String = values
            .next()
            .ok_or(RequestError::UnknownRequestContent)?
            .into();
        let path: Path = full_path.clone().try_into().context("CTX: No such path")?;
        let version = values
            .next()
            .ok_or(RequestError::UnknownRequestContent)?
            .try_into()?;
        Ok((method, full_path, path, version))
    }

    fn parse_line(line: &str, keyword: &str) -> String {
        line.split_once(keyword)
            .map_or("", |(_, value)| value.trim())
            .to_string()
    }

    pub fn from_byte_array(buff: &[u8]) -> Result<Self> {
        let req_data = String::from_utf8_lossy(buff);
        let mut lines = req_data.lines();

        let (method, full_path, path, version) =
            Request::parse_request_line(&mut lines.next().unwrap_or_default().split_whitespace())?;

        let host = Request::parse_line(lines.next().unwrap_or_default(), "Host:");
        let agent = Request::parse_line(lines.next().unwrap_or_default(), "User-Agent:");
        let body = lines.nth(4).map(|s| s.to_owned().into_bytes());
        Ok(Self {
            method,
            full_path,
            path,
            version,
            host,
            agent,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_payload() {
        let payload = r#"GET / HTTP/1.1
                        Host: localhost:4221
                        User-Agent: test-agent/1.0.0
                        "#;
        let req = Request::from_byte_array(payload.as_bytes()).unwrap();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.path, Path::Empty);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.host, "localhost:4221");
        assert_eq!(req.agent, "test-agent/1.0.0");
    }

    #[test]
    fn parse_empty_content() {
        let payload = r#"GET / HTTP/1.1
                        Host: localhost:4221
                        User-Agent: test-agent/1.0.0
                        "#;
        let req = Request::from_byte_array(payload.as_bytes()).unwrap();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.path, Path::Empty);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.host, "localhost:4221");
        assert_eq!(req.agent, "test-agent/1.0.0");
    }

    #[test]
    fn parse_echo_content() {
        let payload = r#"GET /echo/foo/bar HTTP/1.1
                        Host: localhost:4221
                        User-Agent: test-agent/1.0.0
                        "#;
        let req = Request::from_byte_array(payload.as_bytes()).unwrap();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.path, Path::Echo);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.host, "localhost:4221");
        assert_eq!(req.agent, "test-agent/1.0.0");
    }
}
