use hyper::header::*;
use hyper::mime::*;
use mildew;
use std::env;

pub struct Response {
    pub content_type: ContentType,
    pub body: String,
}

impl Response {
    pub fn html_str(s: &str) -> Response {
        Response {
            body: s.to_string(),
            content_type: ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)])),
        }
    }

    pub fn html(s: &str) -> Response {
        let contents = mildew::file_get_contents(&format!("resources/{}", s))
            .unwrap_or(format!("resources/{}", s));
        Response {
            body: contents,
            content_type: ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)])),
        }
    }
}