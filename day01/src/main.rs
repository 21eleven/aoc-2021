fn main() {
    let mut input: Vec<i32> = include_str!("input.txt")
        .split('\n').filter(|x| x != &"")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut c = 0;
    for i in 1..input.len() {
        if input[i] > input[i-1] {
            c += 1;
        }
    }
    dbg!(c);
    let mut windows = vec![];
    for i in 2..input.len() {
        windows.push(
            input[i]+input[i-1]+input[i-2]
        );
    }
    let mut c = 0;
    for i in 1..windows.len() {
        if windows[i] > windows[i-1] {
            c += 1;
        }
    }
    dbg!(c);
}
