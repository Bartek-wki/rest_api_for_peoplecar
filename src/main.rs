#[macro_use]
extern crate rocket;

use rocket::Request;

mod routes;
mod services;

use routes::people_car::calculate_dissel_usage_for_distance;
use routes::people_car::probability_of_unit_injector_fail;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[catch(500)]
fn wrong_input() -> &'static str {
    "Check the correctness of your data"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![
                calculate_dissel_usage_for_distance,
                probability_of_unit_injector_fail
            ],
        )
        .register("/api", catchers![not_found, wrong_input])
}
