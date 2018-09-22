use std::collections::HashMap;
use std::io::stdin;

pub fn departments() {
    let mut departments = HashMap::new();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("could not read stdin");

        let command = input.split_whitespace().collect::<Vec<_>>();

        match command[..] {
            ["Add", employee, "to", department] => {
                departments
                    .entry(String::from(department))
                    .or_insert(Vec::new())
                    .push(String::from(employee));
            }
            ["List", department] => {
                if let Some(employees) = departments.get(department) {
                    employees
                        .iter()
                        .for_each(|employee| println!("{}", employee));
                }
            }
            ["List"] => {
                departments.iter().for_each(|(department, employees)| {
                    println!("{}:", department);
                    employees
                        .iter()
                        .for_each(|employee| println!("  {}", employee));
                });
            }
            ["Quit"] => {
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
