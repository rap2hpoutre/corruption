use hyper::header::*;
use hyper::mime::*;
use mildew;
use handlebars::Handlebars;
use rustc_serialize::json::{Json, ToJson};

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
        Response::html_str(&Response::file(s))
    }

    pub fn tpl<T>(s: &str, data: T) -> Response where T: ToJson {
        let handlebars = Handlebars::new();
        let contents = handlebars.template_render(&Response::file(s), &data).ok().unwrap();
        Response {
            body: contents,
            content_type: ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)])),
        }
    }

    fn file(s: &str) -> String {
        mildew::file_get_contents(&format!("resources/{}", s))
            .unwrap_or(format!("resources/{} not found", s))
    }
}