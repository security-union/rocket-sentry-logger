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
use sentry::{Breadcrumb, ClientOptions};
/// Sentry Log level & User config
pub use sentry::{ClientInitGuard as Guard, Level as LogLevel, User};
pub use steps::{Step, StepType};

/// Initialize a sentry client instance with the recommended sentry configuration.
/// Reads the *SENTRY_DNS* variable from the environment to start the client
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
///     logger::init();
/// }
///```
pub fn init() -> Guard {
    let dsn = std::env::var("SENTRY_DSN").expect("SENTRY_DSN must be set");
    let options = ClientOptions {
        send_default_pii: true,
        attach_stacktrace: true,
        release: sentry::release_name!(),
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
///   data: None,
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

/// Returns an instance of [`LoggerFairing`] to be attached on a rocket instance
///
/// ```rust
/// fn main() {
///    rocket::ignite()
///    .attach(logger::fairing());
/// }
/// ```
pub fn fairing() -> impl Fairing {
    LoggerFairing
}
