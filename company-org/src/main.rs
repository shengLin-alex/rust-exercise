#![feature(rustc_private)]

mod company;

fn main() {
    let dep_one = company::Department {
        name: String::from("sales"),
        employee: vec!["jackson".to_string(), "andy".to_string()],
    };

    let dep_two = company::Department {
        name: String::from("rd"),
        employee: vec!["sam".to_string(), "lin".to_string()],
    };

    let mut company = company::Company {
        departments: Vec::new(),
    };

    company.add_department(dep_one);
    company.add_department(dep_two);

    company.remove_department(String::from("sales"));

    company.add_department(company::Department{
        name: String::from("ec"),
        employee: vec!["jason".to_string()]
    });

    let company_string = company.to_json();
    let company_decode = company::Company::serialize_to_company(&company_string);

    println!("{:?}", company_decode);
}
