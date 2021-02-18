//! A fairing for monitoring requests & responses
//! with the sentry SDK for a Rocket Application 

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use sentry::{Breadcrumb, Level};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub struct LoggerFairing;

/// Implement Fairing to allow our logger
/// report bad responses
impl Fairing for LoggerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Sentry logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    /// On each request, add a breadcrumb to the current scope
    /// to record info about the incoming request
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

    /// On each response, check the status code or the 
    /// success property which we use to report bad responses
    /// and report the body to sentry
    fn on_response(&self, _request: &Request, response: &mut Response) {
        let body_str = response.body_string().unwrap_or_default();
        let body: Value = serde_json::from_str(&body_str).unwrap_or_default();

        if response.status().code >= 400 {
            sentry::capture_message(&body_str, Level::Error);
        }

        if let Value::Bool(false) = body["success"] {
            sentry::capture_message(&body_str, Level::Error);
        }
    }
}
