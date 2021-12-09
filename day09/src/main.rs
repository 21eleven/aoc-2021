use std::collections::HashSet;

fn p1(input: Vec<Vec<usize>>) -> Vec<(i32,i32)>{
    let m = input.len();
    let n = input[0].len();
    let dirs: Vec<(i32, i32)> = vec![(1,0), (-1,0), (0,1), (0,-1)];
    let mut lows :Vec<usize> = vec![];
    let mut pos: Vec<(i32, i32)> = vec![];
    for i in 0i32..m as i32 {
        for j in 0i32..n as i32 {
            let val = input[i as usize][j as usize];
            let mut chk = true;
            for (di, dj) in &dirs {
                let ii = i+di;
                let jj = j+dj;
                if ii >=0 && ii < m as i32 && jj >= 0 && jj < n as i32 {
                    if input[ii as usize][jj as usize] <= val {
                        chk = false
                    }
                }
            }
            if chk == true {
                lows.push(val+1);
                pos.push((i,j));
            }
        }
    }
    dbg!(lows.clone().into_iter().sum::<usize>());
    pos
}
fn p2(input: Vec<Vec<usize>>, lows: Vec<(i32,i32)>) {
    fn dfs(x: i32, y: i32, seen: &mut HashSet<(i32,i32)>, input: &Vec<Vec<usize>>) {
        let m = input.len();
        let n = input[0].len();
        let dirs: Vec<(i32, i32)> = vec![(1,0), (-1,0), (0,1), (0,-1)];
        for (dx, dy) in &dirs {
            let xx = x+dx;
            let yy = y+dy;
            if xx >=0 && xx < m as i32 && yy >= 0 && yy < n as i32 {
                let val = input[xx as usize][yy as usize]; 
                if !seen.contains(&(xx,yy)) && val != 9 {
                    seen.insert((xx,yy));
                    dfs(xx, yy, seen, input);
                }
            }
        }
    }
    let mut sizes = vec![];
    for (x,y) in lows {
        let mut seen = HashSet::new();
        dfs(x,y, &mut seen, &input);
        sizes.push(seen.len());
    }
    sizes.sort();
    dbg!(sizes[sizes.len()-1]*sizes[sizes.len()-2]*sizes[sizes.len()-3]);
}

fn main() {
    let input: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|ln|{
            ln.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
        })
        .collect();
    let lows = p1(input.clone());
    p2(input.clone(), lows);
}
