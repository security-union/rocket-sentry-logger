#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate sentry;

mod fairing;
mod steps;
use rocket::fairing::Fairing;
use sentry::ClientInitGuard;

use fairing::LoggerFairing;
pub enum LogLevel {
    Warning,
    Error,
    Debug,
    Info,
}

// TODO: IMPLEMENT SENTRY CLIENT OPTIONS WRAPPER
pub struct LogOptions {}

pub struct Logger {
    guard: ClientInitGuard,
}

// TODO: LOGGER IMPLEMENTATION
impl Logger {
    pub fn init(options: LogOptions) {
        todo!()
    }

    pub fn log(message: &str, level: LogLevel) {
        todo!()
    }

    pub fn track_step() {
        todo!()
    }

    pub fn set_user() {
        todo!()
    }

    pub fn fairing() -> impl Fairing {
        LoggerFairing
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
