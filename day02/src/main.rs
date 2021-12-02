fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();

    let mut hori: i32 = 0;
    let mut vert: i32 = 0;
    for s in &input {
        let (dir, v)= s.split_once(' ').unwrap();
        let v = v.parse::<i32>().unwrap();

        match dir {
            "forward" => {
                hori += v;
            }
            "up" => {
                vert -= v;
            }
            "down" => {
                vert += v;
            }
            _ => panic!(),
        }
    }
    dbg!(vert*hori);
    
    let mut hori: i32 = 0;
    let mut vert: i32 = 0;
    let mut aim: i32 = 0;
    for s in &input {
        let (dir, v)= s.split_once(' ').unwrap();
        let v = v.parse::<i32>().unwrap();

        match dir {
            "forward" => {
                hori += v;
                vert += aim*v;
            }
            "up" => {
                aim -= v;
            }
            "down" => {
                aim += v;
            }
            _ => panic!(),
        }
    }
    dbg!(vert*hori);
}
