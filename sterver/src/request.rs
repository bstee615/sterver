use std::fmt;
use std::fmt::Display;
use std::convert::{ From };

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
    UPDATE,
    DELETE,
    UNKNOWN,
}

impl From<String> for HttpMethod {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "UPDATE" => HttpMethod::UPDATE,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::UNKNOWN,
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    pub path: String,
    version: String,
}

impl HttpRequest {
    pub fn from_str(s: &str) -> Option<Self> {
        let mut split = s.split(" ");
        let method = HttpMethod::from(split.next()?.to_owned());
        let path = split.next()?.to_owned();
        let version = split.next()?.to_owned();
        Some(Self { method: method, path: path, version: version })
    }
    
    pub fn is_valid(&self) -> bool {
        match self.method {
            HttpMethod::UNKNOWN => false,
            _ => true
        }
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\t{}\t{}", self.method, self.path, self.version)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn request_from_str_success() {
        let req = HttpRequest::from_str("GET /usr/benja HTTP/1.1").unwrap();
        assert!(match req.method {
                    HttpMethod::GET => true,
                    _ => false,
                });
        assert_eq!(req.path, "/usr/benja");
        assert_eq!(req.version, "HTTP/1.1");
        assert!(req.is_valid());
    }
    
    #[test]
    fn request_from_str_fails_insufficient_args() {
        assert!(match HttpRequest::from_str("GET /usr/benja") {
            None => true,
            Some(_) => false,
        });
    }
    
    #[test]
    fn request_from_str_fails_invalid_method() {
        let req = HttpRequest::from_str("GTE /usr/benja HTTP/1.1").unwrap();
        assert!(match req.method {
                    HttpMethod::UNKNOWN => true,
                    _ => false,
                });
        assert_eq!(req.path, "/usr/benja");
        assert_eq!(req.version, "HTTP/1.1");
        assert!(!req.is_valid());
    }

    #[test]
    fn method_from_string() {
        assert!(match HttpMethod::from(String::from("GET")) {
            HttpMethod::GET => true,
            _ => false
        });
        assert!(match HttpMethod::from(String::from("POST")) {
            HttpMethod::POST => true,
            _ => false
        });
        assert!(match HttpMethod::from(String::from("UPDATE")) {
            HttpMethod::UPDATE => true,
            _ => false
        });
        assert!(match HttpMethod::from(String::from("DELETE")) {
            HttpMethod::DELETE => true,
            _ => false
        });
        assert!(match HttpMethod::from(String::from("DELTE")) {
            HttpMethod::UNKNOWN => true,
            _ => false
        });
    }
}
