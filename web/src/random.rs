use super::Response;
use actix_web::{Json, Query};
use git_release_name::{self, Case};
use param::Format;
use rand;

#[derive(Deserialize)]
pub struct Params {
    format: Option<Format>,
}

#[derive(Serialize)]
pub struct Name {
    name: String,
    sha: String,
}

pub fn handler(q: Query<Params>) -> Json<Response<Name>> {
    let format = q.format.unwrap_or(Case::Lower.into());
    let sha = format!("{:08x}", rand::random::<u32>());
    let name = git_release_name::lookup(&sha)
        .map(|p| p.with_case(*format).to_string())
        .unwrap_or(String::new());

    Json(Response::new(Name { name, sha }))
}
