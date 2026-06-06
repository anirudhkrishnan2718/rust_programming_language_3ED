use std::{
    collections::HashMap,
    io::{self, Write},
    vec,
};

fn main() {
    // Question 1
    let mut v = vec![
        8, 4, 9, 4, 3, 7, 3, 10, 3, 10, 3, 7, 1, 4, 6, 7, 9, 8, 1, 1, 1, 6, 7, 3, 9, 3, 7, 2, 4, 5,
        8, 7, 5, 2, 1, 2, 5, 10, 7, 1, 4, 9, 8, 3, 3, 2, 5, 1, 4, 5,
    ];

    v.sort();
    let h = v.len() / 2;
    if v.len() % 2 == 0 {
        let med: f32 = 0.5 * (v[h] + v[h - 1]) as f32;
        println!("The median value is {med}");
    } else {
        println!("The median value is {}", v[h]);
    }

    let mut d = HashMap::new();
    for item in v {
        let count = d.entry(item).or_insert(0);
        *count += 1;
    }

    println!("Integer frequency\n{d:?}");

    let mut max_val = 0;
    let mut mode = -1;
    for (key, val) in d {
        if val > max_val {
            max_val = val;
            mode = key;
        }
    }

    println!("The mode is {mode} with {max_val} occurrences\n");

    // Question 2
    let inputs = vec![String::from("apple"), String::from("first")];
    for item in inputs {
        println!("{item} becomes {:?}", piglatin(&item));
    }

    //Question 3
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut val = String::new();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        let vec: Vec<&str> = command.trim().split(' ').collect();

        // Add employees to dict
        if vec[0] == "Add" {
            let mut current = employees.entry(vec[3].to_string()).or_insert(vec![]);
            current.push(vec[1].to_string());
        } else if (vec[0] == "Show" && vec[1] == "All") {
            let mut all_empl: Vec<String> = Vec::new();
            for (_key, val) in &employees {
                // println!("{key:?} has {val:?}");
                all_empl.extend_from_slice(val);
            }
            all_empl.sort();
            println!("The list of all employees is {:?}", all_empl);
        } else if (vec[0] == "Show" && employees.contains_key(vec[1])) {
            let mut one_dep = Vec::new();
            one_dep = employees[vec[1]].clone();
            one_dep.sort();
            println!("The list of {} employees is {:?}", vec[1], one_dep);
        } else if vec[0] == "Exit" {
            break;
        } else {
            println!("Invalid Command");
        }
    }
}

fn piglatin(s: &str) -> String {
    let mut copied: Vec<char> = s.chars().collect();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&copied[0]) {
        copied.extend(vec!['-', 'h', 'a', 'y']);
    } else {
        let first = copied[0];
        copied.remove(0);
        copied.extend(vec!['-', first, 'a', 'y']);
    }
    let result: String = copied.into_iter().collect();
    result
}
