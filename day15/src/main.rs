use std::collections::BinaryHeap;
use std::cmp::Reverse;

type Map =  Vec<Vec<u32>>;
type Heap = BinaryHeap<(Reverse<u32>, usize, usize, usize)>;
const DIRS: &[(usize,usize); 2] = &[(1,0), (0,1)];
fn p1(input: Map) {
    let mut heap: Heap = BinaryHeap::new();
    let X = input.len();
    let Y = input[0].len();
    heap.push((Reverse(0), X-1+Y-2, 0usize, 0usize));
    let target = (X-1, Y-1);
    let mut done = false;
    while !done {
        let (Reverse(cost), _, x, y) = heap.pop().unwrap();
        for (dx, dy) in DIRS {
            let xx = x+*dx as usize;
            let yy = y+*dy as usize;
            if xx >= 0 && xx < X && yy >= 0 && yy < Y {
                let n = input[xx][yy];
                // dbg!(xx,yy, cost);
                if (xx, yy) == target {
                    dbg!(cost+n);
                    done = true;
                    break
                }
                heap.push((Reverse(cost+n), (X-1-xx)+(Y-1-yy), xx, yy));
            }
        }
    }

}
fn p2(input: Vec<&str>) {}

fn main() {
    let input: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|ln| {
            ln.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
        }).collect();
    p1(input.clone());
}
