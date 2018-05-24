use super::Response;
use actix_web::{Json, Query};
use param;
use rn_dictionary::{self, Case};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Params {
    shas: param::CSV,
    format: Option<param::Format>,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bulk_names_can_be_formed_from_list() {
        let bulk_names =
            BulkNames::from_list(Case::Snake, &[String::from("abc"), String::from("xyz")]);
        assert_eq!(
            bulk_names,
            BulkNames {
                names: [
                    (
                        String::from("abc"),
                        Some(String::from("ambitiously_timeless_gemot"))
                    ),
                    (String::from("xyz"), None),
                ].iter()
                    .cloned()
                    .collect()
            }
        );
    }
}
