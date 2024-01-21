use std::fmt::{Display, Formatter};

use crate::http::{content_type::ContentType, status::Status, version::Version};

#[derive(Debug, Clone)]
pub enum Content {
    Text(String),
    Binary(Vec<u8>),
}
impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Binary(_) => write!(f, "binary content"),
            Content::Text(text) => write!(f, "{text}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub version: Version,
    pub status: Status,
    pub content_length: usize,
    pub content_type: ContentType,
    pub content: Option<Content>,
}

impl Response {
    pub fn ok_str(content: String) -> Self {
        Self {
            status: Status::Ok,
            version: Version::V1_1,
            content_type: ContentType::TextPlain,
            content_length: content.len(),
            content: Some(Content::Text(content)),
        }
    }

    pub fn ok_bin(content: Vec<u8>) -> Self {
        Self {
            status: Status::Ok,
            version: Version::V1_1,
            content_type: ContentType::OctetStream,
            content_length: content.len(),
            content: Some(Content::Binary(content)),
        }
    }

    pub fn not_found_str() -> Self {
        Self {
            status: Status::NotFound,
            version: Version::V1_1,
            content_type: ContentType::TextPlain,
            content_length: 0,
            content: None,
        }
    }

    pub fn not_found_bin() -> Self {
        Self {
            status: Status::NotFound,
            version: Version::V1_1,
            content_type: ContentType::OctetStream,
            content_length: 0,
            content: None,
        }
    }

    pub fn bad_req_bin() -> Self {
        Self {
            status: Status::BadRequest,
            version: Version::V1_1,
            content_type: ContentType::OctetStream,
            content_length: 0,
            content: None
        }
    }

    pub fn bad_req_str() -> Self {
        Self {
            status: Status::BadRequest,
            version: Version::V1_1,
            content_type: ContentType::TextPlain,
            content_length: 0,
            content: None
        }
    }

    pub fn created(content_length: usize) -> Self {
        Self {
            status: Status::BadRequest,
            version: Version::V1_1,
            content_type: ContentType::TextPlain,
            content_length,
            content: None
        }
    }
}

impl From<Response> for Vec<u8> {
    fn from(val: Response) -> Self {
        let version: &str = val.version.into();
        let status: &str = val.status.into();
        let content_type: &str = val.content_type.into();
        let new_line: &str = "\r\n";

        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(version.as_bytes());
        bytes.extend(b" ");
        bytes.extend(status.as_bytes());
        bytes.extend(new_line.as_bytes());
        bytes.extend(b"Content-Type: ");
        bytes.extend(content_type.as_bytes());
        bytes.extend(new_line.as_bytes());
        let content_length = format!("Content-Length: {}", val.content_length);
        bytes.extend(content_length.as_bytes());
        bytes.extend(new_line.as_bytes());
        bytes.extend(new_line.as_bytes());
        if let Some(Content::Binary(content)) = val.content {
            bytes.extend(&content);
        } else if let Some(Content::Text(content)) = val.content {
            bytes.extend(content.as_bytes());
        }

        bytes
    }
}

// this should get deleted imo
impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let version: &str = self.version.into();
        let status: &str = self.status.into();
        let content_type: &str = self.content_type.into();
        let content_length = self.content_length.to_string();

        write!(
            f,
            "{} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            version, status, content_type, content_length, self.content.clone().unwrap_or(Content::Text("".to_string()))
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn http_response_to_string() {
        let response = Response::ok_str("test".to_owned());
        let response_content = response.content.clone();
        let response_content_length = response.content_length.to_string();
        let response_status_str: &str = response.status.into();
        let response_ver_str: &str = response.version.into();
        let response_str: String = response.to_string();
        let content_type_str: &str = ContentType::TextPlain.into();
        assert_eq!(response_status_str, "200 OK");
        assert_eq!(response_ver_str, "HTTP/1.1");
        assert_eq!(response_content.unwrap().to_string(), "test");
        assert_eq!(response_content_length, "4");
        assert_eq!(content_type_str, "text/plain");
        assert_eq!(
            response_str,
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest"
        );
    }

    #[test]
    fn http_response_to_binary() {
        let response = Response::ok_bin(vec![0u8, 0u8, 0u8, 0u8]);
        let response_content = response.content.clone();
        let response_content_length = response.content_length.to_string();
        let response_status_str: &str = response.status.into();
        let response_ver_str: &str = response.version.into();
        let response_str: String = response.to_string();
        let content_type_str: &str = ContentType::OctetStream.into();
        assert_eq!(response_status_str, "200 OK");
        assert_eq!(response_ver_str, "HTTP/1.1");
        assert_eq!(response_content.unwrap().to_string(), "binary content");
        assert_eq!(response_content_length, "4");
        assert_eq!(content_type_str, "application/octet-stream");
        assert_eq!(response_str, "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: 4\r\n\r\nbinary content");
    }
}
