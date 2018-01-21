#[macro_use] extern crate nickel;
extern crate rn_dictionary;

use nickel::{Nickel, HttpRouter};
use nickel::status::StatusCode;

fn main() {
    let mut server = Nickel::new();
    server.get("/api/release-name/:sha", middleware! { |req, mut res|
        if let Some(sha) = req.param("sha") {
            match rn_dictionary::lookup(&sha) {
                Ok(name) => format!("{}", name),
                Err(_) => {
                    res.set(StatusCode::UnprocessableEntity);
                    "".to_string()
                }
            }
        } else {
            "SHA not detected".to_string()
        }
    });
    server.listen("0.0.0.0:6767").expect("Server failed to launch");
}
