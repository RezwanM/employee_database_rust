use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the Company Employee Database!");

    let mut employee_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Select an option:\n(1) Add an employee\n(2) Get employee names within a department\n(3) Get all employee names by department\n(4) Quit");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                let info: (String, String) = add_employee();
                let mut employees: Vec<String> =
                    employee_map.get(&info.0).unwrap_or(&Vec::new()).to_vec();
                employees.push(info.1);
                employee_map.insert(info.0, employees);
                println!("\n");
                continue;
            }
            2 => {
                let department: String = get_department();

                match employee_map.get_key_value(&department) {
                    Some(department) => {
                        println!("Members of the department are:");
                        println!("{:?}\n", employee_map.get(department.0).unwrap());
                        continue;
                    }
                    None => {
                        println!("No such department found!");
                        println!("Try from the following:");
                        for key in employee_map.keys() {
                            println!("{key}");
                        }
                        println!("\n");
                        continue;
                    }
                }
            }
            3 => {
                let sorted_keys: Vec<String> = sort_department(&employee_map);
                println!("Here are the employees by department:");

                for key in sorted_keys {
                    println!("{key}:");
                    println!("{:?}\n", employee_map.get(&key).unwrap());
                }

                continue;
            }
            4 => break,
            _ => {
                println!("Invalid selection! Please try again.\n");
                continue;
            }
        }
    }
}

fn add_employee() -> (String, String) {
    let info: (String, String);

    loop {
        println!(
            "Enter a new employee info in the format 'Add <employee_name> to <department_name>':"
        );
        println!("e.g., Add Sally to Engineering");

        let mut prompt = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        if prompt.trim().split_whitespace().count() != 4 {
            println!("Please enter prompt in the correct format!\n");
            continue;
        } else if prompt.trim().split_whitespace().nth(0) != Some("Add") {
            println!("Please enter prompt in the correct format!\n");
            continue;
        } else if prompt.trim().split_whitespace().nth(2) != Some("to") {
            println!("Please enter prompt in the correct format!\n");
            continue;
        } else {
            let prompt_new: &str = &prompt;
            let department_name = prompt_new
                .trim()
                .split_whitespace()
                .nth(3)
                .unwrap()
                .to_string();
            let employee_name = prompt_new
                .trim()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_string();

            info = (department_name, employee_name);

            break;
        }
    }
    info
}

fn get_department() -> String {
    let department: String;

    loop {
        println!("Enter department name:");

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        if input_str.trim().split_whitespace().count() > 1 {
            println!("Please enter a single department name!\n");
            continue;
        } else {
            department = input_str
                .trim()
                .split_whitespace()
                .nth(0)
                .unwrap()
                .to_string();
            break;
        }
    }
    department
}

fn sort_department(employees: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut keys: Vec<String> = employees.clone().into_keys().collect();
    keys.sort_unstable();
    keys
}
