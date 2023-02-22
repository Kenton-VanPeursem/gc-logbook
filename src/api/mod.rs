use crate::database::patient::Patient;
use rocket::fairing::AdHoc;
use rocket::{delete, get, post, put, Build, Rocket};

use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;

use rocket_sync_db_pools::{database, rusqlite};

use self::rusqlite::params;

#[database("rusqlite_patients")]
struct Db(rusqlite::Connection);

#[get("/patient/<id>")]
async fn get_patient_by_id(db: Db, id: u64) -> String {
    format!("Found Patient {id}")
}

#[post("/patient", data = "<patient_form>")]
async fn create_patient(db: Db, patient: Json<Patient>) -> String {
    format!("New Patient {:?}", patient)
}

#[put("/patient", data = "<patient_form>")]
async fn update_patient(db: Db, patient: Json<Patient>) -> String {
    format!("Update Patient {:?}", patient)
}

#[delete("/patient/<id>")]
async fn delete_patient(db: Db, id: u64) -> String {
    format!("Delete Patient {id}")
}

async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    Db::get_one(&rocket)
        .await
        .expect("database mounted")
        .run(|conn| {
            conn.execute(
                r#"
                CREATE TABLE patients (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    date DATE,
                    specialty VARCHAR(9),
                    gender VARCHAR(7),
                    age INTEGER,
                    indication VARCHAR(250),
                    summary CLOB
                );"#,
                params![],
            )
        })
        .await
        .expect("can init rusqlite DB");

    return rocket;
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Rusqlite Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Rusqlite Init", init_db))
            .mount("/rusqlite", routes![list, create, read, delete, destroy])
    })
}
