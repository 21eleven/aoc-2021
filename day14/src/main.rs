use std::collections::HashMap;

fn p1(start: Vec<char>, lc: HashMap<(char, char), char>) {
    let mut string = start;
    for _ in 0..40 {
        let mut next = vec![];
        string.windows(2).for_each(|w| {
            next.push(w[0]);
            next.push(*lc.get(&(w[0], w[1])).unwrap());
        });
        next.push(string.pop().unwrap());
        string = next;
        // dbg!(&string);
    }
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in string {
        *counter.entry(c).or_insert(0) += 1
    }
    let mut counts: Vec<&usize> = counter.values().collect();
    counts.sort();
    dbg!(counts[counts.len() - 1] - counts[0]);
}
// fn p2(input: Vec<&str>) {
fn p2(start: Vec<char>, lc: HashMap<(char, char), char>) {
    let mut cache: HashMap<(char, char, usize), HashMap<char, u128>> = HashMap::new();
    let mut counter: HashMap<char, u128> = HashMap::new();
    // for c in &start {
    //     *counter.entry(*c).or_insert(0) += 1
    // }
    fn dp(
        c1: char,
        c2: char,
        gen: usize,
        lc: &HashMap<(char, char), char>,
        memo: &mut HashMap<(char, char, usize), HashMap<char, u128>>,
    ) -> HashMap<char, u128> {
        match memo.get(&(c1,c2,gen)) {
            Some(res) => res.clone(),
            None => {
                if gen == 0 {
                    let mut counts: HashMap<char,u128> = HashMap::new();
                    *counts.entry(c1).or_insert(0) += 1;
                    *counts.entry(c2).or_insert(0) += 1;
                    memo.insert((c1,c2,gen), counts.clone());
                    counts
                } else {
                    let mut counts: HashMap<char,u128> = HashMap::new();
                    let c3 = lc.get(&(c1,c2)).unwrap();
                    let cA = dp(c1, *c3, gen-1, lc, memo);
                    let cB = dp(*c3, c2, gen-1, lc, memo);
                    for (k, v) in cA.into_iter() {
                        *counts.entry(k).or_insert(0) += v;
                    }
                    for (k, v) in cB.into_iter() {
                        *counts.entry(k).or_insert(0) += v;
                    }
                    *counts.entry(*c3).or_insert(0) -= 1;
                    memo.insert((c1,c2,gen), counts.clone());
                    counts
                }
            }
        }

    }
    for w in start.windows(2) {
        let counts = dp(w[0], w[1], 40, &lc, &mut cache);
        for (k, v) in counts.into_iter() {
            *counter.entry(k).or_insert(0) += v;
        }
    }
    for i in 1..start.len()-1 {
        *counter.entry(start[i]).or_insert(0) -= 1;
    }
    let mut counts: Vec<&u128> = counter.values().collect();
    counts.sort();
    dbg!(counts[counts.len() - 1] - counts[0]);
}

fn main() {
    let input = include_str!("input2.txt");
    // let input: Vec<&str> = include_str!("input2.txt")
    let (start, rules) = input.split_once("\n\n").unwrap();
    let mut lc = HashMap::new();
    for ln in rules.lines() {
        let (l, r) = ln.split_once(" -> ").unwrap();
        let c1 = l.chars().nth(0).unwrap();
        let c2 = l.chars().nth(1).unwrap();
        let c3 = r.chars().nth(0).unwrap();
        lc.insert((c1, c2), c3);
    }
    let mut start: Vec<char> = start.chars().collect();
    p2(start.clone(), lc.clone());
    // p1(input.clone());
}
