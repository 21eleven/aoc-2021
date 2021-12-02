use std::collections::{HashMap, HashSet};

fn main() {
    let mut input: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|s| String::from(s))
        .collect();

    let mut h : i32= 0;
    let mut vert: i32 = 0;
    for s in &input {
        let (dir, v)= s.split_once(' ').unwrap();

        match dir {
            "forward" => {
                h += v.parse::<i32>().unwrap();
            }
            "up" => {
                vert -= v.parse::<i32>().unwrap();

            }
            "down" => {
                vert += v.parse::<i32>().unwrap();

            }
            _ => panic!(),
        }


    }


    dbg!(vert*h);
    let mut h : i32= 0;
    let mut vert: i32 = 0;
    let mut aim: i32 = 0;
    for s in &input {
        let (dir, v)= s.split_once(' ').unwrap();

        match dir {
            "forward" => {
                h += v.parse::<i32>().unwrap();
                vert += (aim*v.parse::<i32>().unwrap())
            }
            "up" => {
                aim -= v.parse::<i32>().unwrap();

            }
            "down" => {
                aim += v.parse::<i32>().unwrap();

            }
            _ => panic!(),
        }
    }
    dbg!(vert*h);
}
