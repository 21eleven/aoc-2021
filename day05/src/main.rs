use std::collections::HashMap;

fn p1(input: Vec<&str>) {
    let mut counter: HashMap<(usize, usize), usize> = HashMap::new();
    for ln in &input {
        let (p1, p2) = ln.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();
        let mut x1 = x1.parse::<usize>().unwrap();
        let mut x2 = x2.parse::<usize>().unwrap();
        let mut y1 = y1.parse::<usize>().unwrap();
        let mut y2 = y2.parse::<usize>().unwrap();
        if x1 == x2 {
            if x1 > x2 {
                x1 ^= x2;
                x2 ^= x1;
                x1 ^= x2;
            }
            if y1 > y2 {
                y1 ^= y2;
                y2 ^= y1;
                y1 ^= y2;
            }
            println!("{} vert {} {} {} {}", &ln, x1, y1, x2, y2);
            for yy in y1..=y2 {
                // dbg!((x1, yy));
                let c = counter.entry((x1, yy)).or_insert(0);
                *c += 1;
            }

        } else if y1 == y2 {
            if x1 > x2 {
                x1 ^= x2;
                x2 ^= x1;
                x1 ^= x2;
            }
            if y1 > y2 {
                y1 ^= y2;
                y2 ^= y1;
                y1 ^= y2;
            }
            println!("{} hori {} {} {} {}", &ln, x1, y1, x2, y2);
            for xx in x1..=x2 {
                // dbg!((xx, y1));
                let c = counter.entry((xx, y1)).or_insert(0);
                *c += 1;
            }
        } else {
            // panic!("neither horizontal nor vertical");
            //
            // for n in 0..(x2-x1) {
            //     let c = counter.entry((x1+n, y2+n)).or_insert(0);
            //     *c += 1;
            // }
            if x2 < x1 {
                x1 ^= x2;
                x2 ^= x1;
                x1 ^= x2;
                y1 ^= y2;
                y2 ^= y1;
                y1 ^= y2;
            }
            if y2 < y1 {
                for n in 0..=(x2-x1) {
                    let c = counter.entry((x1+n, y1-n)).or_insert(0);
                    *c += 1;
                }

            } else {
                for n in 0..=(x2-x1) {
                    let c = counter.entry((x1+n, y1+n)).or_insert(0);
                    *c += 1;
                }
            }
            println!("{} diag {} {} {} {}", &ln, x1, y1, x2, y2);
        }
    }
    dbg!(
        &counter.values().filter(|x| x>&&1).count()//.collect::<Vec<usize>>().len()
    );
    dbg!(&counter.get(&(2,1)));
    for y in 0..10 {
        for x in 0..10 {
            match counter.get(&(x,y)) {
                Some(n) => print!("{}", n),
                None => print!("{}",'.'),
            }
        }
        print!("{}",'\n');
    }
}
    // for y 
fn p2(input: Vec<&str>) {
    dbg!(&input);
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
    // let input: Vec<&str> = include_str!("input2.txt")
        .lines()
        .collect();
    dbg!(&input);
    p1(input.clone());
}
