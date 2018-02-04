#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rn_dictionary;
extern crate rocket;

use rocket::response::status::NotFound;

#[get("/api/release-name/<sha>")]
fn api_release_name(sha: String) -> Result<String, NotFound<String>> {
    match rn_dictionary::lookup(&sha) {
        Ok(name)    => Ok(format!("{}", name)),
        Err(_)      => Err(NotFound("No name found".to_string())),
    }
}
fn main() {
    rocket::ignite().mount("/", routes![api_release_name]).launch();
}
