use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut bits = vec![0i32; 12];

    for ln in &input {
        for (i, c) in ln.chars().enumerate() {
            if c == '1' {
                bits[i] = bits[i] + 1;
            } else {
                bits[i] = bits[i] - 1;
            }
        }
    }

    let bits2: Vec<_> = bits.into_iter().rev().collect();
    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, c) in bits2.into_iter().enumerate() {
        if c > 0 {
            // gamma gets one
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    dbg!(gamma * epsilon);

    let mut o2 = input.clone();
    let mut c: usize = 0;

    loop {
        let bit = if o2.iter().fold(0i32, |bc, s| {
            if s.chars().skip(c).next().unwrap() == '1' {
                bc + 1
            } else {
                bc - 1
            }
        }) >= 0
        {
            '1'
        } else {
            '0'
        };

        o2 = o2
            .into_iter()
            .filter(|x| {
                if x.chars().skip(c).next().unwrap() == bit {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&str>>();

        if o2.len() == 1 {
            break;
        }
        if o2.len() == 0 {
            panic!()
        }
        c += 1;
    }

    let mut co2 = input.clone();
    let mut c = 0;

    loop {
        let bit = if co2.iter().fold(0i32, |bc, s| {
            if s.chars().skip(c).next().unwrap() == '1' {
                bc + 1
            } else {
                bc - 1
            }
        }) >= 0
        {
            '0'
        } else {
            '1'
        };

        co2 = co2
            .into_iter()
            .filter(|x| {
                if x.chars().skip(c).next().unwrap() == bit {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&str>>();

        if co2.len() == 1 {
            break;
        }
        if co2.len() == 0 {
            panic!()
        }
        c += 1;
    }

    dbg!(i32::from_str_radix(o2[0], 2).unwrap() * i32::from_str_radix(co2[0], 2).unwrap());
    dbg!(
        i32::from_str_radix(o2[0], 2).unwrap(),
        i32::from_str_radix(co2[0], 2).unwrap()
    );
}
