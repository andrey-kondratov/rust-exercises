use crate::ex3::db::Database;

pub const HELP_TEXT: &str = "Write `add <Name> to <Dept>` to add a person to a department.";

pub fn handle(db: &mut Database, args: Vec<&str>) {
    let mut employee_name = Vec::<&str>::new();
    let mut department_name = Vec::<&str>::new();

    let mut past_separator = false;
    for arg in args {
        if arg.to_lowercase() == "to" {
            past_separator = true;
            continue;
        }

        match past_separator {
            true => department_name.push(arg),
            false => employee_name.push(arg)
        }
    }

    if employee_name.is_empty() || department_name.is_empty() {
        println!("{}", HELP_TEXT);
        return;
    }

    let employee_name = employee_name.join(" ");
    let department_name = department_name.join(" ");
    db.add(&employee_name, &department_name);
}
