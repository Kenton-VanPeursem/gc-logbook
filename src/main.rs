mod database;

use crate::database::Database;
use crate::database::patient::{Gender, Specialty, Patient};

use chrono::NaiveDate;

fn main() {
    let db = Database::new("data/patients.db");

    // let patient = Patient {
    //     date: NaiveDate::from_ymd_opt(1997, 1, 24).unwrap(),
    //     age: 28u8,
    //     gender: Gender::Female,
    //     indication: "Family history of craniosynostosis".to_string(),
    //     specialty: Specialty::Prenatal,
    //     summary: "Father with craniosynostosis, they are worries about their children. 50/50 chance.".to_string()
    // };
    //
    // if let Err(error) = db.insert_patient(patient) {
    //     panic!("Encountered err: {}", error);
    // };

    let patients = db.lookup_patient_by_date(NaiveDate::from_ymd_opt(1997, 1, 24).unwrap()).unwrap();

    for p in patients {
        println!("{:?}", p);
    }

    let patients = db.lookup_patient_by_specialty(Specialty::Prenatal).unwrap();
    for p in patients {
        println!("{:?}", p);
    }
}
