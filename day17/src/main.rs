use std::collections::HashSet;

fn test(vx: i32, vy: i32,x1: i32, x2: i32, y1:i32, y2: i32) -> i32 {
    let (mut vx, mut vy) = (vx, vy);
    let (mut px, mut py) = (0,0);
    let mut best = i32::MIN;
    while py >= y1 && px <= x2 {
        best = std::cmp::max(best,py);
        if px >= x1 && px <= x2 && py >= y1 && py <= y2 {
            return best
        } 
        else {
            px += vx;
            py += vy;
            vy -= 1;
            if vx >= 1 {
                vx -= 1;
            } 
            else if vx <= -1 {
                vx += 1
            }
        } 


    }
    i32::MIN
}

fn p1(x1: i32, x2: i32, y1:i32, y2: i32) {
    let mut hits: HashSet<(i32, i32)> = HashSet::new();
    let mut best = i32::MIN;
    for yvel in y1..200 {
        for xvel in 0..x2+1 {
            let res = test(xvel, yvel, x1,x2,y1,y2);
            if res != i32::MIN {
                hits.insert((xvel, yvel));
            }
            best = std::cmp::max(best,res);
        }
    }
    dbg!(best);
    dbg!(hits.len());
}

fn p2(input: Vec<&str>) {}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .trim()
        .split(" ")
        .collect();
    let (x1,x2) = input[2][2..input[2].len()-1].split_once("..").unwrap();
    let (y1,y2) = input[3][2..].split_once("..").unwrap();
    let x1 = x1.parse::<i32>().unwrap();
    let x2 = x2.parse::<i32>().unwrap();
    let y1 = y1.parse::<i32>().unwrap();
    let y2 = y2.parse::<i32>().unwrap();
    p1(x1,x2,y1,y2);
}
