use actix_web::{Json, Query};
use std::collections::HashMap;
use rn_dictionary::{self, Case};
use param;
use super::Response;

#[derive(Deserialize)]
pub struct Params {
    shas: param::CSV,
    format: Option<param::Format>,
}

#[derive(Serialize)]
pub struct BulkNames {
    names: HashMap<String, Option<String>>,
}

impl BulkNames {
    fn new(names: HashMap<String, Option<String>>) -> Self {
        Self { names }
    }

    fn from_list(case: Case, shas: &[String]) -> Self {
        let mut map = HashMap::new();
        for sha in shas {
            let name = rn_dictionary::lookup(&sha)
                .map(|name| name.with_case(case).to_string())
                .ok();
            map.insert(sha.to_string(), name);
        }
        Self::new(map)
    }
}

pub fn handler(q: Query<Params>) -> Json<Response<BulkNames>> {
    let format = q.format.unwrap_or(Case::Lower.into());

    Json(Response::new(BulkNames::from_list(*format, &q.shas)))
}
