use crate::ex3::db::DB;

pub const HELP_TEXT: &str =
    "Write `list <Dept>` or `list` to get employees of a department or all employees.";

pub fn handle(db: &DB, args: Vec<&str>) {
    if args.len() == 0 {
        let mut total: usize = 0;
        for department_name in db.get_departments() {
            total += list_department(department_name, db);
        }

        println!("Total employees in the company: {}.", total);
        return;
    }

    match db.get_department(&args.join(" ")) {
        Some((department_name, employees)) => print_department(&department_name, employees),
        None => println!("Department not found."),
    }
}

fn list_department(department_name: &str, db: &DB) -> usize {
    let mut count: usize = 0;

    match db.get_employees(department_name) {
        Some(employees) => {
            println!(
                "There are {number} employees in {department}: {names}.",
                number = employees.len(),
                department = department_name,
                names = employees.join(", ")
            );
            count += employees.len();
        }
        None => println!(
            "There are no employees in {}.",
            department_name.to_ascii_uppercase()
        ),
    }

    count
}

fn print_department(department_name: &str, employees: &Vec<String>) {
    println!(
        "There are {number} employees in {department}: {names}.",
        number = employees.len(),
        department = department_name,
        names = employees.join(", ")
    );
}
