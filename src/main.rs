mod api;
mod database;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(api::stage())
}
