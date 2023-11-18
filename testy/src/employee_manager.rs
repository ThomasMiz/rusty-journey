// EXERCISE: Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
// the user retrieve a list of all people in a department or all people in the company by department,
// sorted alphabetically.

use std::collections::HashMap;
use std::io;

const ADD_USAGE: &str = "Add <employee> to <department>";
const REMOVE_USAGE: &str = "Remove <employee> from <department>";
const PRINT_USAGE: &str = "Print <department>";
const PRINTALL_USAGE: &str = "Printall";
const EXIT_USAGE: &str = "Exit";

const NO_DEPARTMENT_ERROR: &str = "No such department!";
const NO_EMPLOYEE_ERROR: &str = "Employee not found!";

fn main() {
    println!("Welcome to Boglant Engineering's amazing EmployeeManagerPlus++!");
    println!("The following commands are available:");
    println!("--> {ADD_USAGE}");
    println!("--> {REMOVE_USAGE}");
    println!("--> {PRINT_USAGE}");
    println!("--> {PRINTALL_USAGE}");
    println!("--> {EXIT_USAGE}");

    // Maps each department name to a list of employee names
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for ele in io::stdin().lines() {
        match ele {
            Err(_) => break,
            Ok(line) => {
                let mut split = line.split_whitespace();
                match split.next() {
                    None => println!("Error: no command specified!"),
                    Some(command) => match command {
                        "Add" => match split.next() {
                            None => println!("Usage: {ADD_USAGE}"),
                            Some(employee) => match split.next() {
                                Some("to") => match split.next() {
                                    None => println!("Usage: {ADD_USAGE}"),
                                    Some(department) => {
                                        let employees = map
                                            .entry(department.to_string())
                                            .or_insert_with(|| Vec::new());
                                        employees.push(String::from(employee));
                                    }
                                },
                                _ => println!("Usage: {ADD_USAGE}"),
                            },
                        },
                        "Remove" => match split.next() {
                            None => println!("Usage: {REMOVE_USAGE}"),
                            Some(employee) => match split.next() {
                                Some("from") => match split.next() {
                                    None => println!("Usage: {REMOVE_USAGE}"),
                                    Some(department) => {
                                        let department = department.to_string();
                                        let maybe_employees = map.get_mut(&department);
                                        match maybe_employees {
                                            None => println!("{NO_DEPARTMENT_ERROR}"),
                                            Some(employees) => {
                                                match employees
                                                    .iter()
                                                    .filter(|x| x.as_str().eq(employee))
                                                    .enumerate()
                                                    .next()
                                                {
                                                    None => println!("{NO_EMPLOYEE_ERROR}"),
                                                    Some((index, _)) => {
                                                        if employees.len() == 1 {
                                                            map.remove(&department);
                                                        } else {
                                                            employees.swap_remove(index);
                                                        }
                                                    }
                                                };
                                            }
                                        }
                                    }
                                },
                                _ => println!("Usage: {REMOVE_USAGE}"),
                            },
                        },
                        "Print" => match split.next() {
                            None => println!("Usage: {PRINT_USAGE}"),
                            Some(department) => {
                                let maybe_employees = map.get_mut(&department.to_string());
                                match maybe_employees {
                                    None => println!("{NO_DEPARTMENT_ERROR}"),
                                    Some(employees) => {
                                        employees.sort_unstable();
                                        println!("{department} employees: {employees:?}");
                                    }
                                }
                            }
                        },
                        "Printall" => {
                            let mut all = Vec::new();
                            for (department, employees) in &map {
                                for name in employees {
                                    all.push((department, name));
                                }
                            }

                            all.sort_by(|x, y| x.1.cmp(y.1));
                            println!("All employees:");
                            for (department, name) in all {
                                println!("Theres {name} from {department}");
                            }
                            println!("That's all of 'em!");
                        }
                        "Printdebug" => {
                            println!("{map:?}");
                        }
                        "Exit" => break,
                        other => println!("Unknown command: {other}"),
                    },
                }
            }
        }
    }
    println!("Goodbye!");
}
