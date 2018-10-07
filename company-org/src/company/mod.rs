// use extern crate
extern crate rustc_serialize;

use rustc_serialize::json;

/// Company, storage departments
/// 
/// Use attribute implement RustcDecodable, RustcEncodable, Debug
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Company {
    pub departments: Vec<Department>,
}

/// Department, storage its name and its employee.
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Department {
    pub name: String,
    pub employee: Vec<String>,
}

impl Company {

    /// Add department to a company.
    pub fn add_department(&mut self, department: Department) {
        self.departments.push(department);
    }

    /// Remove a department by a name.
    pub fn remove_department(&mut self, dep_name: String) {
        let index = self
            .departments
            .iter()
            .position(|d| *d.name == dep_name)
            .unwrap();

        self.departments.remove(index);
    }

    /// Serialize Company to json string.
    pub fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }

    /// Deserialize json string to Company.
    pub fn serialize_to_company(encoded: &String) -> Company {
        json::decode(encoded).unwrap()
    }
}
