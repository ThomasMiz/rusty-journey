// EXERCISE: Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
// the user retrieve a list of all people in a department or all people in the company by department,
// sorted alphabetically.

use std::collections::HashMap;
use std::io;
use std::str::SplitWhitespace;

const ADD_USAGE: &str = "Add <employee> to <department>";
const REMOVE_USAGE: &str = "Remove <employee> from <department>";
const PRINT_USAGE: &str = "Print <department>";
const PRINTALL_USAGE: &str = "Printall";
const EXIT_USAGE: &str = "Exit";

enum ErrorType {
    UnknownCommand(String),
    BadUsage(&'static str),
    DepartmentNotFound(String),
    EmployeeNotFound(String, String),
}

fn handle_add(
    map: &mut HashMap<String, Vec<String>>,
    split: &mut SplitWhitespace<'_>,
) -> Result<(), ErrorType> {
    let employee = split.next().ok_or_else(|| ErrorType::BadUsage(ADD_USAGE))?;

    split
        .next()
        .filter(|x| (*x).eq("to"))
        .ok_or_else(|| ErrorType::BadUsage(ADD_USAGE))?;

    let department = split
        .next()
        .ok_or_else(|| ErrorType::BadUsage(ADD_USAGE))?
        .to_string();

    if !split.next().is_none() {
        return Err(ErrorType::BadUsage(ADD_USAGE));
    }

    let deparment_employees = map.entry(department).or_insert_with(|| Vec::new());
    deparment_employees.push(String::from(employee));

    return Ok(());
}

fn handle_remove(
    map: &mut HashMap<String, Vec<String>>,
    split: &mut SplitWhitespace<'_>,
) -> Result<(), ErrorType> {
    let employee = split
        .next()
        .ok_or_else(|| ErrorType::BadUsage(REMOVE_USAGE))?;

    split
        .next()
        .filter(|x| (*x).eq("from"))
        .ok_or_else(|| ErrorType::BadUsage(REMOVE_USAGE))?;

    let department = split
        .next()
        .ok_or_else(|| ErrorType::BadUsage(REMOVE_USAGE))?;

    if !split.next().is_none() {
        return Err(ErrorType::BadUsage(ADD_USAGE));
    }

    let deparment_employees = map
        .get_mut(department)
        .ok_or_else(|| ErrorType::DepartmentNotFound(department.to_string()))?;

    let (index, _) = deparment_employees
        .iter()
        .filter(|x| (*x).eq(employee))
        .enumerate()
        .next()
        .ok_or_else(|| ErrorType::EmployeeNotFound(department.to_string(), employee.to_string()))?;

    if deparment_employees.len() == 1 {
        map.remove(department);
    } else {
        deparment_employees.swap_remove(index);
    }

    return Ok(());
}

fn handle_print(
    map: &mut HashMap<String, Vec<String>>,
    split: &mut SplitWhitespace<'_>,
) -> Result<(), ErrorType> {
    let department = split
        .next()
        .ok_or_else(|| ErrorType::BadUsage(PRINT_USAGE))?;

    if !split.next().is_none() {
        return Err(ErrorType::BadUsage(ADD_USAGE));
    }

    let employees = map
        .get_mut(department)
        .ok_or_else(|| ErrorType::DepartmentNotFound(department.to_string()))?;

    employees.sort_unstable();
    println!("{department} employees: {employees:?}");

    return Ok(());
}

fn handle_printall(
    map: &mut HashMap<String, Vec<String>>,
    split: &mut SplitWhitespace<'_>,
) -> Result<(), ErrorType> {
    if !split.next().is_none() {
        return Err(ErrorType::BadUsage(PRINTALL_USAGE));
    }

    let mut all = Vec::new();
    for (department, employees) in map {
        for name in employees {
            all.push((department, name));
        }
    }

    all.sort_by(|x, y| x.1.cmp(&y.1));
    println!("All employees:");
    for (department, name) in all {
        println!("Theres {name} from {department}");
    }
    println!("That's all of 'em!");

    return Ok(());
}

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
                let result = match split.next() {
                    None => continue, //"Error: no command specified!",
                    Some(command) => match command {
                        "Add" => handle_add(&mut map, &mut split),
                        "Remove" => handle_remove(&mut map, &mut split),
                        "Print" => handle_print(&mut map, &mut split),
                        "Printall" => handle_printall(&mut map, &mut split),
                        "Printdebug" => {
                            println!("{map:?}");
                            Ok(())
                        }
                        "Exit" => break,
                        other => Err(ErrorType::UnknownCommand(String::from(other))), // println!("Unknown command: {other}"),
                    },
                };

                if let Err(e) = result {
                    match e {
                        ErrorType::UnknownCommand(command) => {
                            println!("Unknown command: {command}")
                        }
                        ErrorType::BadUsage(msg) => {
                            println!("Usage: {msg}")
                        }
                        ErrorType::DepartmentNotFound(department) => {
                            println!("Department not found: {}", department)
                        }
                        ErrorType::EmployeeNotFound(department, employee) => println!(
                            "Employee {} not found in department {}",
                            employee, department
                        ),
                    }
                }
            }
        }
    }
    println!("Goodbye!");
}
