use std::collections::HashSet;

const DIRS: &[(i32, i32)] = &[(1,0), (1,1), (1,-1), (0,1), (0,-1), (-1,1), (-1,0), (-1,-1)];

fn p1(input: Vec<&str>) {
    let mut energy = input.into_iter().map(|ln| {
        ln.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();
    fn chk_flash(x:i32, y:i32, octo: &mut Vec<Vec<u32>> ,flashed: &mut HashSet<(usize, usize)>, flashes: &mut i32, add: bool) {
        if x >= 0 && y >= 0 && x < octo.len() as i32 && y < octo[0].len() as i32 {
            let xu = x as usize;
            let yu = y as usize;
            if add && !flashed.contains(&(xu,yu)){
                octo[xu][yu] += 1;
            }
            if !flashed.contains(&(xu,yu)) && octo[xu][yu] > 9 {
                flashed.insert((xu,yu));
                octo[xu][yu] = 0;
                *flashes += 1;
                for (dx, dy) in DIRS {
                    chk_flash(x+dx, y+dy, octo, flashed, flashes, true)
                }
            }
        } 
    }
    let mut flashes = 0;
    for _ in 0..100 {
        for i in 0..energy.len() {
            for j in 0..energy[0].len() {
                energy[i][j] += 1;
            }
        }
        let mut flashed = HashSet::new();
        for i in 0..energy.len() {
            for j in 0..energy[0].len() {
                chk_flash(i as i32,j as i32, &mut energy, &mut flashed, &mut flashes, false);
            }
        }
    }
    dbg!(flashes);

}
fn p2(input: Vec<&str>) {
    let mut energy = input.into_iter().map(|ln| {
        ln.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();
    fn chk_flash(x:i32, y:i32, octo: &mut Vec<Vec<u32>> ,flashed: &mut HashSet<(usize, usize)>, flashes: &mut i32, add: bool) {
        if x >= 0 && y >= 0 && x < octo.len() as i32 && y < octo[0].len() as i32 {
            let xu = x as usize;
            let yu = y as usize;
            if add && !flashed.contains(&(xu,yu)){
                octo[xu][yu] += 1;
            }
            if !flashed.contains(&(xu,yu)) && octo[xu][yu] > 9 {
                flashed.insert((xu,yu));
                octo[xu][yu] = 0;
                *flashes += 1;
                for (dx, dy) in DIRS {
                    chk_flash(x+dx, y+dy, octo, flashed, flashes, true)
                }
            }
        } 
    }
    let mut flashes = 0;
    let mut c = 0;
    loop {
        c += 1;
        for i in 0..energy.len() {
            for j in 0..energy[0].len() {
                energy[i][j] += 1;
            }
        }
        let mut flashed = HashSet::new();
        for i in 0..energy.len() {
            for j in 0..energy[0].len() {
                chk_flash(i as i32,j as i32, &mut energy, &mut flashed, &mut flashes, false);
            }
        }
        if energy.iter().map(|row| {
            row.iter().sum::<u32>()
        }).sum::<u32>() == 0{
            dbg!(c);
            break
        }
    }

}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();
    p1(input.clone());
    p2(input.clone());
}
