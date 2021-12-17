use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::min;

type Map =  Vec<Vec<u32>>;
type Heap = BinaryHeap<(Reverse<u32>, usize, usize, usize)>;
const DIRS: &[(i32,i32); 4] = &[(1,0), (0,1),(-1,0), (0,-1)];
fn p1(input: Map) {
    let X = input.len();
    let Y = input[0].len();
    let mut dp = vec![vec![vec![u32::MAX; Y]; X]; X+Y-1];
    // dp[0][0][0] = input[0][0];
    dp[0][0][0] = 0;
    for g in 1..X+Y-1 {
        for i in 0..X {
            for j in 0..Y {
                let mut costs = vec![];
                if i == 0 && j == 0{
                    dp[g][i][j] = dp[g-1][i][j];
                }
//                 for (dx,dy) in DIRS {
//                     let xx = i as i32+dx;
//                     let yy = j as i32+dy;
//                     if xx >= 0 && xx < X as i32 && yy >= 0 && yy < Y as i32 {
//                         costs.push(dp[g-1][xx as usize][yy as usize])
// 
//                     }
// 
//                 }
                let upper = if i != 0 {
                    dp[g-1][i-1][j]
                } else {u32::MAX};
                let left = if j != 0 {
                    dp[g-1][i][j-1]
                } else {u32::MAX};
                // let cost = min(upper, left);
                // let cost = *costs.iter().min().unwrap();
                if cost != u32::MAX {
                    dp[g][i][j] = input[i][j] +cost ;
                }
            }
        }
        if dp[g][X-1][Y-1] != u32::MAX {
            dbg!(g);
            dbg!(dp[g][X-1][Y-1]);
        }
    }
    dbg!(dp[X+Y-2][X-1][Y-1]);

}
fn p2(mut input: Map) {
    let X = input.len()*5;
    let Y = input[0].len()*5;
    let x = input.len();
    let y = input[0].len();
    for i in 0..X {
        if i >= x {
            input.push(vec![]);
        }
        for j in 0..Y {
            if i < x && j < y {
                continue
            } else if j < y {
                let mut next = input[i-x][j]+1;
                if next > 9 {
                    next = 1;
                }
                input[i].push(next);
            } else {
                let mut next = input[i][j-y]+1;
                if next > 9 {
                    next = 1;
                }
                input[i].push(next);
            }
        }
    }
    p1(input);


}

fn main() {
    let input: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|ln| {
            ln.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
        }).collect();
    p1(input.clone());
    p2(input.clone());
}
