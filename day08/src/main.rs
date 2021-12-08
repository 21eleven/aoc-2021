use std::collections::{HashMap, HashSet};

fn p1(input: Vec<&str>) {
    let digits = input.iter()
        .map(|ln| {
            ln.split_once(" | ").unwrap().1.trim()
                .split(" ")
                .map(|s| {
                    match s.len() {
                        2|4|3|7 => {
                            1
                        }
                        _ => 0
                    }
                }).sum()
        }).collect::<Vec<usize>>();
    dbg!(digits.iter().sum::<usize>());

}
fn p2(input: Vec<&str>) {
    let digits = input.iter()
        .map(|ln| {
            let mut lc: HashMap<String, usize> = HashMap::new();
            let (signals, output) = ln
                .split_once(" | ").unwrap();
            let mut signals: Vec<String>= signals.split(" ")
                .map(|s| {
                    let mut c: Vec<char> = s.chars().collect();
                    c.sort();
                    c.iter().collect()
                }).collect();
            signals.sort_by_key(|x| x.len());
            lc.insert(signals[0].clone(), 1);
            lc.insert(signals[1].clone(), 7);
            lc.insert(signals[2].clone(), 4);
            lc.insert(signals[9].clone(), 8);
            let mut six_idx = 0;
            for idx in 6..9 {
                let one = signals[0].chars().collect::<HashSet<char>>();
                let four = signals[2].chars().collect::<HashSet<char>>();
                let other = signals[idx].chars().collect::<HashSet<char>>();
                let w1 = one.intersection(&other);
                let w4 = four.intersection(&other);
                match (w1.count(), w4.count()) {
                    (2, 4) => lc.insert(signals[idx].clone(), 9),
                    (1, 3) => {six_idx = idx; lc.insert(signals[idx].clone(), 6)}
                    (2, 3) => lc.insert(signals[idx].clone(), 0),
                    _ => panic!(),
                };
            }
            for idx in 3..6 {
                let one = signals[0].chars().collect::<HashSet<char>>();
                let six = signals[six_idx].chars().collect::<HashSet<char>>();
                let other = signals[idx].chars().collect::<HashSet<char>>();
                let w1 = one.intersection(&other);
                let w6 = six.intersection(&other);
                match (w1.count(), w6.count()) {
                    (2, _) => lc.insert(signals[idx].clone(), 3),
                    (1, 5) => lc.insert(signals[idx].clone(), 5),
                    (1, 4) => lc.insert(signals[idx].clone(), 2),
                    _ => panic!(),
                };
            }
            let nums: Vec<usize> = output.trim().split(" ")
                .map(|n| {
                    let mut c: Vec<char> = n.chars().collect();
                    c.sort();
                    *lc.get::<String>(&c.iter().collect()).unwrap()
                }).collect();
            nums[0]*1000+nums[1]*100+nums[2]*10+nums[3]
        }).collect::<Vec<usize>>();
    dbg!(digits.iter().sum::<usize>());
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();
    p1(input.clone());
    p2(input.clone());
}
