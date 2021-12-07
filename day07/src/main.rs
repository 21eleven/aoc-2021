fn compute_diff(n: i32, nums: &Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, x| {acc+(n-x).abs()})
}


fn compute_move(n: i32, nums: &Vec<i32>) -> i32 {
    nums.iter().fold(0i32, |acc, x| {
        acc + (0i32..=(n-x).abs()).into_iter().sum::<i32>()
    })
}

fn p1(input: Vec<i32>) -> i32 {
    let mut cost = i32::MAX;
    let mut prev = cost;
    let mut c = 0;
    loop {
        cost = compute_diff(c, &input);
        if prev < cost {
            return prev
        }
        prev = cost;
        c += 1;
    }
}

fn p2(input: Vec<i32>) -> i32 {
    let mut cost = i32::MAX;
    let mut prev = cost;
    let mut c = 0;
    loop {
        cost = compute_move(c, &input);
        if prev < cost {
            return prev
        }
        prev = cost;
        c += 1;
    }
}

fn main() {
    let mut input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    input.sort();
    dbg!(p1(input.clone()));
    dbg!(p2(input.clone()));
}
