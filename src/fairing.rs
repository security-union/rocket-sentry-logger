use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};

pub struct LoggerFairing;

// TODO: ON REQUEST && ON RESPONSE MIDDLEWARE
impl Fairing for LoggerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Sentry logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        todo!()
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        todo!()
    }
}
