use hyper::header::*;
use hyper::mime::*;
use mildew;
use std::env;
use handlebars::Handlebars;

/*
struct Person {
    name: String,
    age: i16,
}

impl ToJson for Person {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("name".to_string(), self.name.to_json());
        m.insert("age".to_string(), self.age.to_json());
        m.to_json()
    }
}
*/




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
    /*
        pub fn tpl(s: &str) -> Response {
            let mut handlebars = Handlebars::new();

            let contents = mildew::file_get_contents(&format!("resources/{}", s))
                .unwrap_or(format!("resources/{}", s));


        let data = Person {
            name: "Ning Sun".to_string(),
            age: 27
        };
        let contents = handlebars.template_render("Hello, {{name}}", &data).ok().unwrap();

        Response {
            body: contents,
            content_type: ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)])),
        }
    }
    */
}