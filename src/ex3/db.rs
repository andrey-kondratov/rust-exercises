use std::collections::HashMap;

pub struct Database {
    departments: HashMap<String, Vec<String>>,
    total_count: i32
}

impl Database {
    pub fn new() -> Database {
        Database {
            departments: HashMap::<String, Vec<String>>::new(),
            total_count: 0
        }
    }

    pub fn get_department(&self, department_name: &str) -> Option<(String, &Vec<String>)> {
        let department_name = department_name.to_ascii_uppercase();

        self.departments
            .get(&department_name)
            .and_then(|employees| Some((department_name, employees)))
    }

    pub fn get_departments(&self) -> Vec<&String> {
        self.departments.keys().collect()
    }

    pub fn get_employees_total_count(&self) -> &i32 {
        &self.total_count
    }

    pub fn get_employees(&self, department_name: &str) -> Option<&Vec<String>> {
        self.departments.get(&department_name.to_ascii_uppercase())
    }

    pub fn add(&mut self, employee_name: &String, department_name: &String) {
        let department_name = department_name.to_ascii_uppercase();
        let department = self
            .departments
            .entry(department_name)
            .or_insert(Vec::new());

        department.push(employee_name.to_string());
        department.sort_unstable(); // todo use sorted list instead to sort upon insertion

        self.total_count += 1;

        println!("ok");
    }
}
