use std::collections::HashMap;

fn mkint(chars: &[char]) -> u64 {
    let mut n = 0;
    for c in chars {
        n <<= 1;
        n |= match c {
            '1' => 1,
            _ => 0
        }
    }
    n
}
#[derive(Debug)]
struct Packet {
    ver: u64,
    typeid: u64,
    packets: Option<Vec<Packet>>,
    literal: Option<u64>
}
impl Packet {
    fn parse(bits: &[char]) -> (Packet, usize) {
        let ver = mkint(&bits[..3]);
        let typeid = mkint(&bits[3..6]);
        if typeid == 4 {
            let mut i = 6;
            let mut literal = 0;
            let mut j;
            loop {
                j = i + 5;
                literal <<= 4;
                literal |= mkint(&bits[i+1..j]);
                if bits[i] == '0' {
                    break
                }
                i = j;
            }
            (Packet {
                ver, typeid, packets: None, literal: Some(literal)
            }, j)
        } else {
            let mut packets = vec![];
            let mut j;
            if &bits[6] == &'0' {
                let mut i = 7 + 15;
                j = i;
                j += mkint(&bits[7..i]) as usize;
                while i < j {
                    let (pack, x) = Packet::parse(&bits[i..j]);
                    packets.push(pack);
                    i += x;
                }
            } else {
                j = 7 + 11;
                let n = mkint(&bits[7..j]) as usize;
                for _ in 0..n {
                    let (pack, x) = Packet::parse(&bits[j..]);
                    packets.push(pack);
                    j += x;
                }
            }
            (Packet {
                ver, typeid, packets: Some(packets), literal: None
            }, j)
        }
    }
    fn value(&self) -> u64 {
        match self.literal {
            Some(n) => n,
            None => {
                match self.typeid {
                    0 => {
                        self.packets.as_ref().unwrap().iter()
                            .map(|p| p.value()).sum()
                    }
                    1 => {
                        self.packets.as_ref().unwrap().iter()
                            .fold(1, |acc, p| acc*p.value())
                    }
                    2 => {
                        self.packets.as_ref().unwrap().iter()
                            .map(|p| p.value()).min().unwrap()
                    }
                    3 => {
                        self.packets.as_ref().unwrap().iter()
                            .map(|p| p.value()).max().unwrap()
                    }
                    5 => {
                        if self.packets.as_ref().unwrap()[0].value() > self.packets.as_ref().unwrap()[1].value() {
                            1
                        } else {0}
                    }
                    6 => {
                        if self.packets.as_ref().unwrap()[0].value() < self.packets.as_ref().unwrap()[1].value() {
                            1
                        } else {0}
                    }
                    7 => {
                        if self.packets.as_ref().unwrap()[0].value() == self.packets.as_ref().unwrap()[1].value() {
                            1
                        } else {0}
                    }
                    _ => panic!(),
                }
            }
        }
    }
}

fn p1(input: Vec<char>) {
    let (packet, _) = Packet::parse(&input[0..]);
    let mut sumver = 0;
    fn add_ver(p: Packet, s: &mut u64) {
        *s += p.ver;
        match p.packets {
            Some(packets) => {
                for pack in packets {
                    add_ver(pack, s);
                }
            }
            _ => {}
        }
    }
    add_ver(packet, &mut sumver);
    dbg!(&sumver);

}
fn p2(input: Vec<char>) {
    let (packet, _) = Packet::parse(&input[0..]);
    dbg!(&packet);
    dbg!(packet.value());

}

fn main() {
    let mut lu: HashMap<char, Vec<char>> = HashMap::new();
    lu.insert('0', vec!['0','0','0','0']);
    lu.insert('1', vec!['0','0','0','1']);
    lu.insert('2', vec!['0','0','1','0']);
    lu.insert('3', vec!['0','0','1','1']);
    lu.insert('4', vec!['0','1','0','0']);
    lu.insert('5', vec!['0','1','0','1']);
    lu.insert('6', vec!['0','1','1','0']);
    lu.insert('7', vec!['0','1','1','1']);
    lu.insert('8', vec!['1','0','0','0']);
    lu.insert('9', vec!['1','0','0','1']);
    lu.insert('A', vec!['1','0','1','0']);
    lu.insert('B', vec!['1','0','1','1']);
    lu.insert('C', vec!['1','1','0','0']);
    lu.insert('D', vec!['1','1','0','1']);
    lu.insert('E', vec!['1','1','1','0']);
    lu.insert('F', vec!['1','1','1','1']);
    let input: &str = include_str!("input.txt").trim();
    let mut bits = vec![];
    input.chars().for_each(|c| {
        bits.append(&mut lu.get(&c).unwrap().clone());
    });
    p1(bits.clone());
    p2(bits.clone());
}
