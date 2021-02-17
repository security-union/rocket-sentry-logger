use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use sentry::{Breadcrumb, Level, capture_message};
use serde_json::{json, Value};
use std::{collections::BTreeMap, str::FromStr};

pub struct LoggerFairing;

impl Fairing for LoggerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Sentry logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        let data: BTreeMap<String, Value> = vec![
            ("url".into(), json!(request.uri().path())),
            ("method".into(), json!(request.method().as_str())),
        ]
        .into_iter()
        .collect();
        let breadcrumb = Breadcrumb {
            ty: "http".into(),
            category: Some("request".into()),
            data,
            ..Default::default()
        };
        sentry::add_breadcrumb(breadcrumb);
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        // let values = Value::from_str(&response.body_string().unwrap_or(String::new()));
        // if response.status().code >= 400 {
        //     sentry::capture_message(&response.body_string().unwrap_or(String::new()), Level::Error);
        // }
    }
}
