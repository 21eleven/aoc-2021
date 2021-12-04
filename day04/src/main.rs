use std::collections::{HashMap, HashSet};

type Board = Vec<Vec<usize>>;

#[derive(Debug)]
struct BingoBoard {
    board: Board,
    original: Board,
}

impl BingoBoard {
    fn new(
        mut board: Board,
        idx: usize,
        values: &mut HashMap<usize, Vec<(usize, usize, usize)>>,
    ) -> Self {
        let original = board.clone();
        for i in 0..5 {
            for j in 0..5 {
                let key = board[i][j];
                values.entry(key).or_insert(vec![]).push((idx, i, j));
                board[i][j] = 0;
            }
        }
        BingoBoard { board, original }
    }
    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == 0 {
                    sum += self.original[i][j]
                }
            }
        }
        sum
    }
    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            if (0..5).map(|j| self.board[i][j]).sum::<usize>() == 5 {
                return true;
            }
        }
        for j in 0..5 {
            if (0..5).map(|i| self.board[i][j]).sum::<usize>() == 5 {
                return true;
            }
        }
        false
    }
}

fn p1(input: Vec<&str>) {
    let mut lines = input.iter();
    let draws: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = vec![];
    let mut idx = 0;
    let mut values = HashMap::new();
    loop {
        if lines.next().is_none() {
            break;
        }
        let mut board: Board = vec![];
        for _ in 0..5 {
            let ln = lines.next().unwrap();
            board.push(
                ln.trim()
                    .split(" ")
                    .filter(|x| x != &"")
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        }
        boards.push(BingoBoard::new(board, idx, &mut values));
        idx += 1;
    }
    for n in draws {
        for (idx, i, j) in values.get(&n).unwrap() {
            boards[*idx].board[*i][*j] |= 1;
        }
        for board in &boards {
            if board.has_bingo() {
                dbg!(board.sum_unmarked() * n);
                return ();
            }
        }
    }
}
fn p2(input: Vec<&str>) {
    let mut lines = input.iter();
    let draws: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = vec![];
    let mut idx = 0;
    let mut values = HashMap::new();
    loop {
        if lines.next().is_none() {
            break;
        }
        let mut board: Vec<Vec<usize>> = vec![];
        for _ in 0..5 {
            let ln = lines.next().unwrap();
            board.push(
                ln.trim()
                    .split(" ")
                    .filter(|x| x != &"")
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        }
        boards.push(BingoBoard::new(board, idx, &mut values));
        idx += 1;
    }
    let mut winners: HashSet<usize> = HashSet::new();
    for n in draws {
        for (idx, i, j) in values.get(&n).unwrap() {
            boards[*idx].board[*i][*j] |= 1;
        }
        for (idx, board) in boards.iter().enumerate() {
            if board.has_bingo() {
                if winners.len() == boards.len() - 1 && !winners.contains(&idx) {
                    dbg!(board.sum_unmarked() * n);
                    return ();
                } else {
                    winners.insert(idx);
                }
            }
        }
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    p1(input.clone());
    p2(input.clone());
}
