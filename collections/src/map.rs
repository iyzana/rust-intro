use std::collections::HashMap;
use std::io::stdin;

pub fn departments() {
    let mut departments = HashMap::new();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input)
            .expect("could not read stdin");

        let command = input.split_whitespace().collect::<Vec<_>>();

        match command[0] {
            "Add" => {
                let entry = departments.entry(String::from(command[3])).or_insert(Vec::new());
                entry.push(String::from(command[1]));
            }
            "List" => {
                let filter = if command.len() >= 2 {
                    Some(command[1])
                } else {
                    None
                };

                for (department, employees) in departments.iter() {
                    if filter.map_or(false, |d| d != department) { continue; }

                    println!("{}:", department);
                    for employee in employees {
                        println!("  {}", employee);
                    }
                }
            }
            "Quit" => { break; }
            _ => {}
        }
    }
}
