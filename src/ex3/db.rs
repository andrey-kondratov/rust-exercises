use std::collections::HashMap;

pub struct DB {
    departments: HashMap<String, Vec<String>>,
}

impl DB {
    pub fn create() -> DB {
        return DB {
            departments: HashMap::<String, Vec<String>>::new(),
        };
    }

    pub fn get_department(&self, department_name: &str) -> Option<(String, &Vec<String>)> {
        let department_name = department_name.to_ascii_uppercase();
        match self.departments.get(&department_name) {
            Some(employees) => Some((department_name, employees)),
            None => None,
        }
    }

    pub fn get_departments(&self) -> Vec<&String> {
        return self.departments.keys().collect();
    }

    pub fn get_employees(&self, department_name: &str) -> Option<&Vec<String>> {
        self.departments.get(&department_name.to_ascii_uppercase())
    }

    pub fn add(&mut self, employee_name: &String, department_name: &String) -> bool {
        let department_name = department_name.to_ascii_uppercase();
        let department = self
            .departments
            .entry(department_name)
            .or_insert(Vec::new());

        department.push(employee_name.to_string());
        department.sort_unstable(); // todo use sorted list instead to sort upon insertion

        println!("ok");
        true
    }
}
