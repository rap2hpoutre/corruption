use hyper::header::*;
use hyper::mime::*;
use mildew;
use handlebars::Handlebars;
use rustc_serialize::json;
use rustc_serialize::Encodable;
use rustc_serialize::json::ToJson;

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

    pub fn tpl_str<T>(s: &str, data: T) -> Response where T: ToJson {
        let handlebars = Handlebars::new();
        let contents = handlebars.template_render(s, &data).ok().unwrap();
        Response::html_str(&contents)
    }

    pub fn tpl<T>(s: &str, data: T) -> Response where T: ToJson {
        Response::tpl_str(&Response::file(s), data)
    }

    pub fn json<T>(data: T) -> Response where T: Encodable {
        Response {
            body: json::encode(&data).unwrap(),
            content_type: ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])),
        }
    }

    fn file(s: &str) -> String {
        mildew::file_get_contents(&format!("resources/{}", s))
            .unwrap_or(format!("resources/{} not found", s))
    }
}