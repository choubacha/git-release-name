extern crate actix_web;
extern crate git_release_name;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;

use actix_web::{http, server, App};
use serde::Serialize;

mod index;
mod param;
mod random;
mod show;

#[derive(Serialize)]
pub struct Response<T>
where
    T: Serialize,
{
    data: T,
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

fn main() {
    server::new(|| {
        App::new()
            .route("/api/release-name", http::Method::GET, index::handler)
            .route(
                "/api/release-name/random",
                http::Method::GET,
                random::handler,
            )
            .resource("/api/release-name/{sha}", |r| {
                r.method(http::Method::GET).with2(show::handler)
            })
    }).bind("0.0.0.0:6767")
        .unwrap()
        .run();
}
