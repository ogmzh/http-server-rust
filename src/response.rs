use anyhow::{Error, Result};
use crate::http::{version::Version, status::Status, content_type::ContentType};

#[derive(Debug, Clone)]
pub struct Response {
    pub version: Version,
    pub status: Status,
    pub content_length: u16,
    pub content_type: ContentType,
    pub content: String,
}

impl TryInto<String> for Response {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let version: &str = self.version.into();
        let status: &str = self.status.into();
        let content_type: &str = self.content_type.into();
        let response = format!(
            "{} {}\r\n\r\nContent-Type: {}\r\n\r\nContent-Length: {}\r\n\r\n\r\n\r\n{}",
            version, status, content_type, self.content_length, self.content
        );
        Ok(response)
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
            content_type: ContentType::TextPlain
        };
        let response_content = response.content.clone();
        let response_content_length = response.content_length.to_string();
        let response_status_str: &str = response.status.into();
        let response_ver_str: &str = response.version.into();
        let response_str: String = response.try_into().unwrap();
        let content_type_str: &str = ContentType::TextPlain.into();
        assert_eq!(response_status_str, "200 OK");
        assert_eq!(response_ver_str, "HTTP/1.1");
        assert_eq!(response_content, "test");
        assert_eq!(response_content_length, "4");
        assert_eq!(content_type_str, "text/plain");
        assert_eq!(response_str, "HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\n\r\nContent-Length: 4\r\n\r\n\r\n\r\ntest");
    }
}
