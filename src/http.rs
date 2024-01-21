// GET /index.html HTTP/1.1
// Host: localhost:4221
// User-Agent: curl/7.64.1

pub mod version {
    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("Failed to parse version")]
    pub struct VersionParseError;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Version {
        V1_1,
    }

    impl TryFrom<&str> for Version {
        type Error = VersionParseError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "HTTP/1.1" => Ok(Self::V1_1),
                _ => Err(VersionParseError),
            }
        }
    }

    impl TryFrom<Option<&str>> for Version {
        type Error = VersionParseError;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(value) => Self::try_from(value),
                None => Err(VersionParseError),
            }
        }
    }

    impl From<Version> for &str {
        fn from(val: Version) -> Self {
            match val {
                Version::V1_1 => "HTTP/1.1",
            }
        }
    }
}

pub mod method {

    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("Failed to parse method")]
    pub struct MethodParseError;

    #[derive(Debug, PartialEq)]
    pub enum Method {
        Get,
    }

    impl TryFrom<&str> for Method {
        type Error = MethodParseError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "GET" => Ok(Self::Get),
                _ => Err(MethodParseError),
            }
        }
    }

    impl TryFrom<Option<&str>> for Method {
        type Error = MethodParseError;
        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(value) => Self::try_from(value),
                None => Err(MethodParseError),
            }
        }
    }

    impl From<Method> for &str {
        fn from(val: Method) -> Self {
            match val {
                Method::Get => "GET",
            }
        }
    }
}

pub mod status {

    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("Failed to parse status")]
    pub struct StatusParseError;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Status {
        Ok,
        NotFound,
    }

    impl From<Status> for &str {
        fn from(val: Status) -> Self {
            match val {
                Status::Ok => "200 OK",
                Status::NotFound => "404 Not Found",
            }
        }
    }
}

pub mod content_type {

    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("Failed to parse content type")]
    pub struct ContentTypeParseError;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum ContentType {
        TextPlain,
        OctetStream,
    }

    impl From<ContentType> for &str {
        fn from(val: ContentType) -> Self {
            match val {
                ContentType::TextPlain => "text/plain",
                ContentType::OctetStream => "application/octet-stream"
            }
        }
    }
}

pub mod path {

    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum TryFromPathError {
        #[error("Failed to parse path")]
        Parse,

        #[error("Missing path")]
        Missing,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Path {
        Empty,
        Echo,
        UserAgent,
        Files,
    }

    impl From<Path> for &str {
        fn from(val: Path) -> Self {
            match val {
                Path::Empty => "/",
                Path::Echo => "/echo",
                Path::Files => "/files",
                Path::UserAgent => "/user-agent"
            }
        }
    }

    impl TryFrom<&str> for Path {
        type Error = TryFromPathError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(match value {
                "/" => Self::Empty,
                "/user-agent" => Self::UserAgent,
                _ if value.starts_with("/echo") => Path::Echo,
                _ if value.starts_with("/files") => Path::Files,
                _ => return Err(TryFromPathError::Parse),
            })
        }
    }

    impl TryFrom<String> for Path {
        type Error = TryFromPathError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            Self::try_from(value.as_str())
        }
    }

    impl TryFrom<Option<&str>> for Path {
        type Error = TryFromPathError;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(value) => Self::try_from(value),
                None => Err(TryFromPathError::Missing),
            }
        }
    }
}
