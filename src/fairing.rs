//! A fairing for monitoring requests & responses
//! with the sentry SDK for a Rocket Application

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use sentry::{Breadcrumb, Level};
use serde_json::{json, Value};
use std::{collections::BTreeMap, io::Cursor};

/// Rocket fairing to record requests info as sentry breadcrumbs &
/// reports events for bad responses
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
        sentry::configure_scope(|scope| {
            scope.clear_breadcrumbs();
        });
        sentry::add_breadcrumb(breadcrumb);
    }

    /// On each response, check the status code or the
    /// success property which we use to report bad responses
    /// and report the body to sentry
    fn on_response(&self, _request: &Request, response: &mut Response) {
        let status = response.status().clone();
        let body_str = response.body_string();
        if status.code >= 400 {
            sentry::with_scope(
                |scope| {
                    scope.set_extra("Response", json!(body_str.clone()));
                },
                || {
                    sentry::capture_message(&format!("Response: {}", status.reason), Level::Error);
                },
            );
        }
        response.set_sized_body(Cursor::new(body_str.unwrap_or_default()));
    }
}
