extern crate actix_web;
extern crate rn_dictionary;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix_web::{server, App, HttpResponse, Json, Path, Query, http::{self, StatusCode}};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::ops::Deref;

fn release_name(info: Path<String>) -> HttpResponse {
    match rn_dictionary::lookup(&info.into_inner()) {
        Ok(name) => HttpResponse::build(StatusCode::OK).body(name.to_string()),
        Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).finish(),
    }
}

#[derive(Deserialize)]
struct Params {
    shas: CSVParam,
}

struct CSVParam(Vec<String>);

impl Deref for CSVParam {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CSVParam {
    fn deserialize<D>(d: D) -> Result<CSVParam, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        Ok(CSVParam(s.split(',').map(|s| s.to_string()).collect()))
    }
}

#[derive(Serialize)]
struct Response<T>
where
    T: Serialize
{
    data: T
}

impl<T> Response<T>
where
    T: Serialize
{
    fn new(data: T) -> Self {
        Self { data }
    }
}

#[derive(Serialize)]
struct BulkNames {
    names: HashMap<String, Option<String>>,
}

impl BulkNames {
    fn new(names: HashMap<String, Option<String>>) -> Self {
        Self { names }
    }

    fn from_list(shas: &[String]) -> Self {
        let mut map = HashMap::new();
        for sha in shas {
            match rn_dictionary::lookup(&sha) {
                Ok(name) => map.insert(sha.to_string(), Some(name.to_string())),
                Err(_) => map.insert(sha.to_string(), None),
            };
        }
        Self::new(map)
    }
}

fn release_names(q: Query<Params>) -> Json<Response<BulkNames>> {
    Json(Response::new(BulkNames::from_list(&q.shas)))
}

fn main() {
    server::new(|| {
        App::new()
            .route("/api/release-name", http::Method::GET, release_names)
            .route("/api/release-name/{sha}", http::Method::GET, release_name)
    }).bind("0.0.0.0:6767")
        .unwrap()
        .run();
}
