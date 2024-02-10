use crate::ex3::db::Database;

pub const HELP_TEXT: &str =
    "Write `list <Dept>` or `list` to get employees of a department or all employees.";

static EMPTY_VEC: Vec<String> = Vec::new();

pub fn handle(db: &Database, args: Vec<&str>) {
    let department_name = &args.join(" ");
    if !department_name.trim().is_empty() {
        match db.get_department(department_name) {
            Some((department_name, employees)) => print_department(&department_name, employees),
            None => println!("Department not found."),
        }

        return
    }

    let mut total: usize = 0;
    for department_name in db.get_departments() {
        match db.get_employees(department_name) {
            Some(employees) => {
                print_department(department_name, employees);
                total += employees.len();
            }
            None => print_department(department_name, &EMPTY_VEC),
        }
    }

    println!("Total employees in the company: {}.", total);
}

fn print_department(department_name: &str, employees: &Vec<String>) {
    let number = employees.len();
    let employees = employees.join(", ");
    match number {
        0 => println!("There are no employees in {}.", department_name),
        1 => println!("There is 1 employee in {}: {}.", department_name, employees),
        _ => println!(
            "There are {} employees in {}: {}.",
            number, department_name, employees
        ),
    };
}
