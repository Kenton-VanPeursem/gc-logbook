use chrono::NaiveDate;


#[derive(Debug)]
pub enum Specialty {
    General,
    Pediatric,
    Lab,
    Cancer,
    Prenatal,
    Other,
}

impl Specialty {
    pub fn as_str(&self) -> &str {
        match self {
            Specialty::General => "General",
            Specialty::Pediatric => "Pediatric",
            Specialty::Lab => "Lab",
            Specialty::Cancer => "Cancer",
            Specialty::Prenatal => "Prenatal",
            Specialty::Other => "Other",
        }
    }

    pub fn from_str(s: String) -> Specialty {
        match s.as_str() {
            "General" => Specialty::General,
            "Pediatric" => Specialty::Pediatric,
            "Lab" => Specialty::Lab,
            "Cancer" => Specialty::Cancer,
            "Prenatal" => Specialty::Prenatal,
            "Other" => Specialty::Other,
            _ => panic!("Unable to match {:?}", s),
        }
    }
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

impl Gender {
    pub fn as_str(&self) -> &str {
        match self {
            Gender::Male => "M",
            Gender::Female => "F",
            Gender::Unknown => "Unknown",
        }
    }

    pub fn from_str(s: String) -> Gender {
        match s.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            "Unknown" => Gender::Unknown,
            _ => panic!("Unable to match {:?}", s),
        }
    }
}

#[derive(Debug)]
pub struct Patient {
    pub date: NaiveDate,
    pub specialty: Specialty,
    pub gender: Gender,
    pub age: u8,
    pub indication: String,
    pub summary: String,
}

impl Patient {
    pub fn to_csv(&self) -> String {
        return format!("{},{},{},{},{},{}", self.date, self.specialty.as_str(), self.gender.as_str(), self.age, self.indication, self.summary);
    }
}

