use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut copy = list.clone();
    copy.sort();
    copy[copy.len() / 2]
}

pub fn mode(list: &Vec<i32>) -> Option<i32> {
    let mut counts = HashMap::new();

    for i in list.iter() {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    counts.iter().map(|(_, v)| *v).max()
}
