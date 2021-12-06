struct LantFish {
    age: usize,
}

impl LantFish {
    fn cycle(&mut self) -> Option<LantFish> {
        if self.age == 0 {
            self.age = 6;
            Some(LantFish { age: 8})
        } else {
            self.age -= 1;
            None
        }
    }
}

fn p1(input:Vec<usize>) {
    let mut school: Vec<LantFish> = vec![];
    for n in input {
        school.push(LantFish{age:n});
    }
    for _ in 0..256 {
        let mut nextgen = vec![];
        for i in 0..school.len() {
            match school[i].cycle() {
                Some(baby) => nextgen.push(baby),
                None => {}
            }
        }
        school.append(&mut nextgen);
    }
    dbg!(&school.len());
}

fn p2(input:Vec<usize>) {
    let mut dp: Vec<u128> = vec![0; 357];
    for n in &input {
        dp[*n] += 1;
    }
    let mut school = input.len() as u128;
    for i in 0..256 {
        school += dp[i];
        dp[i+7] += dp[i];
        dp[i+9] += dp[i];
        dbg!(&school);
    }
}
fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    p2(input.clone());

}
