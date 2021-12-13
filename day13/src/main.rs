use std::collections::HashSet;

type Point = (usize, usize);

fn p1(coords: Vec<Point>, folds: Vec<(&str, usize)>) {
    let xflip = folds[0].1;
    let flipped: HashSet<Point> = coords
        .into_iter()
        .map(|(x, y)| {
            if x > xflip {
                ((xflip - (x - xflip)), y)
            } else {
                (x, y)
            }
        })
        .collect();
    dbg!(flipped.len());
}
fn flip(vals: Vec<Point>, around: usize, xflip: bool) -> Vec<Point> {
    vals.into_iter()
        .map(|(x, y)| {
            if xflip {
                if x > around {
                    ((around - (x - around)), y)
                } else {
                    (x, y)
                }
            } else {
                if y > around {
                    (x, (around - (y - around)))
                } else {
                    (x, y)
                }
            }
        })
        .collect()
}
fn p2(mut coords: Vec<Point>, folds: Vec<(&str, usize)>) {
    let mut xx = *coords.iter().map(|(x, _)| x).max().unwrap();
    let mut yy = *coords.iter().map(|(_, y)| y).max().unwrap();
    for fold in folds {
        match fold {
            ("x", val) => {
                xx = val;
                coords = flip(coords, val, true);
            }
            ("y", val) => {
                yy = val;
                coords = flip(coords, val, false);
            }
            _ => panic!(),
        }
    }
    let filled: HashSet<Point> = coords.into_iter().collect();
    for y in 0..yy {
        for x in 0..xx {
            if filled.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn main() {
    let (coords, folds) = include_str!("input.txt").split_once("\n\n").unwrap();
    let coords = coords
        .lines()
        .map(|ln| {
            let (l, r) = ln.split_once(",").unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        })
        .collect::<Vec<Point>>();
    let folds = folds
        .lines()
        .map(|ln| {
            let parts = ln.split(" ").collect::<Vec<&str>>();
            let (dim, val) = parts[2].split_once("=").unwrap();
            (dim, val.parse().unwrap())
        })
        .collect::<Vec<(&str, usize)>>();
    p1(coords.clone(), folds.clone());
    p2(coords.clone(), folds.clone());
}
