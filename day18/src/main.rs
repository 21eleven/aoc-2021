#[derive(Clone)]
struct SnailFishN  {
    vals: Vec<u32>,
    lvls: Vec<u32>,
}

impl SnailFishN {
    fn parse(input: &str) -> Self {
        let mut vals = vec![];
        let mut lvls = vec![];
        let mut curr = 0;
        for c in input.chars() {
            match c {
                '['=> curr += 1,
                ']'=> curr -= 1,
                ','=>{}
                _ => {
                    let n = c.to_digit(10).unwrap();
                    vals.push(n);
                    lvls.push(curr);
                }
            }
        }
        SnailFishN { vals, lvls }
    }
    fn add(&mut self, mut other: SnailFishN) {
        self.vals.append(&mut other.vals);
        self.lvls.append(&mut other.lvls);
        self.lvls = self.lvls.clone().into_iter().map(|lvl| lvl+1).collect();
        while self.needs_reduction() {
            self.explodes();
            self.splits();
        }
    }
    fn needs_reduction(&self) -> bool {
        for lvl in &self.lvls {
            if *lvl == 5 {
                return true
            }
        }
        for val in &self.vals {
            if val >= &10 {
                return true
            }
        }
        false
    }
    fn explodes(&mut self) {
        let mut N = self.vals.len();
        let mut i = 0;
        while i < N {
            if self.lvls[i] == 5 {
                if i + 2 < N {
                    self.vals[i+2] += self.vals[i+1];
                }
                self.vals.remove(i+1);
                self.lvls.remove(i+1);
                if i != 0 {
                    self.vals[i-1] += self.vals[i];
                }
                self.vals.remove(i);
                self.lvls.remove(i);
                self.vals.insert(i,0);
                self.lvls.insert(i,4);
                N = self.lvls.len();
                i = 0;
            }
            i += 1;
        }
    }
    fn splits(&mut self) {
        let N = self.vals.len();
        for i in 0..N {
            if self.vals[i] >=10 {
                let tmp = self.vals[i];
                let l = tmp/2;
                let r = tmp - l;
                self.lvls[i] += 1;
                self.vals[i] = l;
                self.lvls.insert(i+1, self.lvls[i]);
                self.vals.insert(i+1, r);
                return
            }
        }
    }
    fn magnitude(&self) -> u32 {
        let mut vals = self.vals.clone();
        let mut lvls = self.lvls.clone();
        let mut lvl = 5;
        while lvl > 0 {
            let mut clean = true;
            for i in 0..lvls.len() {
                if lvls[i] == lvl {
                    clean = false;
                    let mut n = vals.remove(i)*3;
                    n += vals.remove(i)*2;
                    lvls.remove(i);
                    lvls.remove(i);
                    vals.insert(i,n);
                    lvls.insert(i, lvl-1);
                    break
                }
            }
            if clean {
                lvl -= 1;
            }
        }
        vals.pop().unwrap()
    }
    fn print(&self) {
        for n in &self.vals {
            print!("{},", n);
        }
        print!("\n");
        for n in &self.lvls{
            print!("{},", n);
        }
        print!("\n");
    }

}

fn p1(input: Vec<&str>) {
    let mut nums: Vec<SnailFishN> = input.into_iter().map(|ln| SnailFishN::parse(ln)).collect();
    let mut sn = nums.pop().unwrap();
    while !nums.is_empty() {
        sn.add(nums.pop().unwrap());
    }
    sn.print();
    dbg!(sn.magnitude());

}
fn p2(input: Vec<&str>) {
    let mut nums: Vec<SnailFishN> = input.into_iter().map(|ln| SnailFishN::parse(ln)).collect();
    let mut m = 0;
    for i in 0..nums.len()-1 {
        for j in i+1..nums.len() {
            let mut a = nums[i].clone();
            let mut y = nums[i].clone();
            let mut b = nums[j].clone();
            let mut x = nums[j].clone();
            a.add(b);
            m = std::cmp::max(m, a.magnitude());
            x.add(y);
            m = std::cmp::max(m, x.magnitude());
        }

    }
    dbg!(m);
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .rev()
        .collect();
    p1(input.clone());
    p2(input.clone());
}
