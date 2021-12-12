use std::collections::{HashSet, HashMap};

fn p1(input: Vec<&str>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut smol: HashSet<String> = HashSet::new();
    let mut seen_smol: HashSet<String> = HashSet::new(); 
    seen_smol.insert("start".to_string());
    for ln in &input {
        let (l,r) = ln.split_once("-").unwrap();
        graph.entry(l.to_string()).or_insert(vec![]).push(r.to_string());
        graph.entry(r.to_string()).or_insert(vec![]).push(l.to_string());
        if l == l.to_lowercase() {
            smol.insert(l.to_string());
        }
        if r == r.to_lowercase() {
            smol.insert(r.to_string());
        }
    }
    fn dfs(pos: String, mut path: Vec<String>, graph: &HashMap<String, Vec<String>>, paths: &mut HashSet<String>, mut seen_smol: HashSet<String>, smol: &HashSet<String>) {
        path.push(pos.clone());
        if pos == "end".to_string() {
            paths.insert(path.join(","));
        } else {
            if smol.contains(&pos) {
                seen_smol.insert(pos.clone());
            }
            for node in graph.get(&pos).unwrap() {
                if !seen_smol.contains(node) {
                    dfs(node.clone(), path.clone(), graph, paths, seen_smol.clone(), smol)

                }
            }
        }
    }
    let mut paths = HashSet::new();
    dfs("start".to_string(), vec![], &graph, &mut paths, seen_smol.clone(), &smol);

    dbg!(paths.len());

}
fn p2(input: Vec<&str>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut smol: HashSet<String> = HashSet::new();
    let mut seen_smol: HashSet<String> = HashSet::new(); 
    seen_smol.insert("start".to_string());
    for ln in &input {
        let (l,r) = ln.split_once("-").unwrap();
        graph.entry(l.to_string()).or_insert(vec![]).push(r.to_string());
        graph.entry(r.to_string()).or_insert(vec![]).push(l.to_string());
        if l == l.to_lowercase() {
            smol.insert(l.to_string());
        }
        if r == r.to_lowercase() {
            smol.insert(r.to_string());
        }
    }
    fn dfs(pos: String, mut path: Vec<String>, graph: &HashMap<String, Vec<String>>, paths: &mut HashSet<String>, mut seen_smol: HashSet<String>, smol: &HashSet<String>, twice: bool) {
        path.push(pos.clone());
        if pos == "end".to_string() {
            paths.insert(path.join(","));
        } else {
            if smol.contains(&pos) {
                seen_smol.insert(pos.clone());
            }
            for node in graph.get(&pos).unwrap() {
                if !seen_smol.contains(node) {
                    dfs(node.clone(), path.clone(), graph, paths, seen_smol.clone(), smol, twice);
                } else if seen_smol.contains(node) && twice == false && node != &"start".to_string() {
                    dfs(node.clone(), path.clone(), graph, paths, seen_smol.clone(), smol, true);
                }
            }
        }
    }
    let mut paths = HashSet::new();
    dfs("start".to_string(), vec![], &graph, &mut paths, seen_smol.clone(), &smol, false);

    dbg!(paths.len());
}

fn main() {
    let input: Vec<&str> = include_str!("input2.txt")
        .lines()
        .collect();
    p1(input.clone());
    p2(input.clone());
}
