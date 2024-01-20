use std::fmt::{Display, Formatter};

use crate::http::{content_type::ContentType, status::Status, version::Version};

#[derive(Debug, Clone)]
pub struct Response {
    pub version: Version,
    pub status: Status,
    pub content_length: usize,
    pub content_type: ContentType,
    pub content: String,
}

impl Response {
    pub fn ok(content: String) -> Self {
        Self {
            status: Status::Ok,
            version: Version::V1_1,
            content_type: ContentType::TextPlain,
            content_length: content.len(),
            content,
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: Status::NotFound,
            version: Version::V1_1,
            content_type: ContentType::TextPlain,
            content_length: 0,
            content: "".to_owned(),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let version: &str = self.version.into();
        let status: &str = self.status.into();
        let content_type: &str = self.content_type.into();
        let content_length = self.content_length.to_string();

        write!(
            f,
            "{} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            version, status, content_type, content_length, self.content
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn http_response_to_string() {
        let response = Response {
            version: Version::V1_1,
            status: Status::Ok,
            content: "test".to_string(),
            content_length: 4,
            content_type: ContentType::TextPlain,
        };
        let response_content = response.content.clone();
        let response_content_length = response.content_length.to_string();
        let response_status_str: &str = response.status.into();
        let response_ver_str: &str = response.version.into();
        let response_str: String = response.to_string();
        let content_type_str: &str = ContentType::TextPlain.into();
        assert_eq!(response_status_str, "200 OK");
        assert_eq!(response_ver_str, "HTTP/1.1");
        assert_eq!(response_content, "test");
        assert_eq!(response_content_length, "4");
        assert_eq!(content_type_str, "text/plain");
        assert_eq!(response_str, "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest");
    }
}
