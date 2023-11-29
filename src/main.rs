use rocket::{launch, Rocket, Build, routes};

mod models;
mod database;

mod routes;
mod schema;


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![routes::stocks::index, routes::stocks::get_stock_by_id])
}