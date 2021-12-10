

fn p1(input: Vec<&str>) -> Vec<&str> {
    let mut score = 0;
    let mut incomp: Vec<&str> = vec![];
    for ln in input {
        let mut stack: Vec<char> = vec![];
        let mut corrupt = false;
        for c in ln.chars() {
            match c {
                ')' => {
                    let opp = stack.pop().unwrap();
                    if opp != '(' {
                        corrupt = true;
                        score += 3;
                        break
                    }
                }
                ']' => {
                    let opp = stack.pop().unwrap();
                    if opp != '[' {
                        corrupt = true;
                        score += 57;
                        break
                    }
                }
                '}' => {
                    let opp = stack.pop().unwrap();
                    if opp != '{' {
                        corrupt = true;
                        score += 1197;
                        break
                    }
                }
                '>' => {
                    let opp = stack.pop().unwrap();
                    if opp != '<' {
                        corrupt = true;
                        score += 25137;
                        break
                    }
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        if !corrupt {
            incomp.push(ln);
        }
    }
    dbg!(&score);
    incomp

}
fn p2(input: Vec<&str>) {
    dbg!(&input);
    let mut scores: Vec<u64> = vec![];
    for ln in input {
        let mut stack = vec![];
        for c in ln.chars() {
            match c {
                ')'|']'|'}'|'>' => {
                    stack.pop().unwrap();
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        let mut lnscore = 0;
        while !stack.is_empty() {
            lnscore = (lnscore*5) + match stack.pop().unwrap() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            }
        }
        scores.push(lnscore);
    }
    scores.sort();
    dbg!(scores[scores.len()/2]);
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
    // let input: Vec<&str> = include_str!("input2.txt")
        .lines()
        .collect();
    let incomp = p1(input.clone());
    p2(incomp);
}
