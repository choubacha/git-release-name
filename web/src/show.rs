use actix_web::{HttpResponse, Path, Query, http::StatusCode};
use param::Format;
use git_release_name::{self, Case};

#[derive(Deserialize)]
pub struct Params {
    format: Option<Format>,
}

pub fn handler(info: Path<String>, q: Query<Params>) -> HttpResponse {
    let format = q.format.unwrap_or(Case::Lower.into());
    match git_release_name::lookup(&info.into_inner()) {
        Ok(name) => HttpResponse::build(StatusCode::OK).body(name.with_case(*format).to_string()),
        Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).finish(),
    }
}
