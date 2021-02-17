#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate sentry;

mod fairing;
mod steps;

use fairing::LoggerFairing;
use rocket::fairing::Fairing;
use sentry::{Breadcrumb, ClientOptions};
/// Sentry Log level & User config
pub use sentry::{Level as LogLevel, User};
pub use steps::{Step, StepType};

pub struct Logger;

impl Logger {
    pub fn init() {
        let dns = std::env::var("SENTRY_DNS").expect("SENTRY_DNS must be set");
        let options = ClientOptions {
            send_default_pii: true,
            attach_stacktrace: true,
            release: sentry::release_name!(),
            ..Default::default()
        };
        let _guard = sentry::init((dns, options));
    }

    pub fn log(message: &str, level: LogLevel) {
        let _uuid = sentry::capture_message(message, level);
    }

    pub fn track_step(step: Step) {
        let breadcrumb: Breadcrumb = step.into();
        sentry::add_breadcrumb(breadcrumb);
    }

    pub fn set_user(user: User) {
        sentry::configure_scope(|scope| {
            scope.set_user(Some(user));
        })
    }

    pub fn fairing() -> impl Fairing {
        LoggerFairing
    }
}
