use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: u32,
        value: [[String; 2]; n],
    }

    let mut path = HashMap::new();
    let mut visited = HashSet::new();

    for e in &value {
        path.insert(e[0].clone(), e[1].clone());
    }

    for e in &value {
        let mut n = e[0].clone();
        while !visited.contains(&n) {
            visited.insert(n.clone());
            if !path.contains_key(&n) {
                break;
            }
            n = path.get(&n).unwrap().clone();
            // println!("n={} e[0]={}", n, e[0]);
            if n == e[0] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
