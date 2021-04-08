//! A Logger wrapper to integrate the Sentry SDK on a Rocket Server easily
//!
//! Offers a set of functions which helps to log simple messages to sentry,
//! configure users, and set up a fairing on a Rocket server

#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate sentry;

pub mod fairing;
mod steps;

use fairing::LoggerFairing;
use rocket::fairing::Fairing;
use sentry::{protocol::Event, Breadcrumb, ClientOptions};
pub use sentry::{ClientInitGuard as Guard, Level as LogLevel, User};
use serde_json::Value;
use std::{borrow::Cow, sync::Arc};
pub use steps::{Step, StepType};

/// Initial Logger configuration. Lets you configure the *service* name such as *Recipes API*,
/// release name & running environment.
///
///```rust
/// fn main() {
///     let config = InitConfig {
///         service: Some("Recipes API"),
///         environment: "Production",
///         ..Default::default()
///     }
///     logger::init(Some(config));
/// }
///```
pub struct InitConfig {
    pub service: Option<&'static str>,
    pub release: Option<Cow<'static, str>>,
    pub environment: &'static str,
}

impl Default for InitConfig {
    fn default() -> Self {
        InitConfig {
            service: None,
            release: sentry::release_name!(),
            environment: "development",
        }
    }
}

/// Initialize a sentry client instance with the recommended sentry configuration &
/// additional config which can be set with the [`InitConfig`] struct.
///
/// Returns a Sentry ClientInitGuard which will stop the logging service when dropped
///
/// # Panics!
///
/// Panics if the sentry instance is not enabled after the init is done.
/// That can happens due to an invalid dns.
///
///```rust
/// fn main() {
///     logger::init(None);
/// }
///```
pub fn init(dsn: String, config: Option<InitConfig>) -> Guard {
    let config = config.unwrap_or_default();
    let service: String = config.service.unwrap_or("unknown").into();
    let options = ClientOptions {
        send_default_pii: true,
        attach_stacktrace: true,
        release: config.release,
        environment: Some(config.environment.into()),
        before_send: Some(Arc::new(move |mut event: Event| {
            if event.level != LogLevel::Fatal {
                event.stacktrace = None;
            }
            event.tags.insert("Service".into(), service.clone());
            Some(event)
        })),
        ..Default::default()
    };
    let guard = sentry::init((dsn, options));
    if !guard.is_enabled() {
        panic!("Could not initialize sentry");
    }
    guard
}

/// Logs a message to sentry.
/// Use the *LogLevel* enum to set up the desired logging level.
/// Every step tracked previous to the log on the same execution thread
/// will be sent along with the message.
///
///```rust
/// fn main() {
///     logger::log("This is a mock message, Hello World!", LogLevel::Info);
/// }
///```
pub fn log(message: &str, level: LogLevel) {
    let _uuid = sentry::capture_message(message, level);
}

/// Tracks an step to be sent along with the next logged message or event.
///
/// ```rust
/// let step = Step {
///   ty: StepType::Error,
///   title: "Bad request".into(),
///   message: "Mike made a bad request".into(),
///   level: LogLevel::Info,
///   body: None,
/// };
///
/// logger::track_step(step);
/// ```
pub fn track_step(step: Step) {
    let breadcrumb: Breadcrumb = step.into();
    sentry::add_breadcrumb(breadcrumb);
}

/// Allows you to set info about the user related with the current scope.
///
/// ```rust
/// let user = User {
///   id: Some("aslfnsvn-dsvjnfv-ffjfvkfjd"),
///   email: Some("jaster@mail.com"),
///   username: Some("Jaster"),
///   ..Default::default()
/// };
///
/// logger::set_user(user);
/// ```
pub fn set_user(user: User) {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(user));
    })
}

/// Allows you to set extra data about the message.
///
/// ```rust
/// logger::add_data("Response body", json!(body));
/// logger::log("Bad Request: name field required", LogLevel::Error);
/// ```
pub fn add_data(key: &str, data: Value) {
    sentry::configure_scope(|scope| {
        scope.set_extra(key, data);
    })
}

/// Allows you to set additional tags to the sentry issue.
///
/// ```rust
/// logger::set_tag("API", "emergency");
/// logger::log("Bad Request: name field required", LogLevel::Error);
/// ```
pub fn set_tag(name: &str, value: &str) {
    sentry::configure_scope(|scope| {
        scope.set_tag(name, value);
    })
}

/// Returns an instance of [`LoggerFairing`] to be attached on a rocket instance
///
/// ```rust
/// fn main() {
///    rocket::ignite()
///    .attach(logger::fairing());
/// }
/// ```
pub fn fairing(ignore_list: Option<Vec<u16>>) -> impl Fairing {
    LoggerFairing { ignore_list }
}
