use std::io::BufRead;
use std::time::Instant;
use rayon::prelude::*;

fn _toggle(n: u8, k: usize) -> u8 {
    n ^ n << k 
}
fn set(n: u8, k: usize) -> u8 {
    n | 1_u8 << k
}
fn clear(n: u8, k: u8) -> u8 {
    n & !(1 << k)
}
fn encode(s: &str) -> u8 {
    let mut d = 0;
    for c in s.chars() {
        match c {
            'a' => { d = set(d, 0) }
            'b' => { d = set(d, 1) }
            'c' => { d = set(d, 2) }
            'd' => { d = set(d, 3) }
            'e' => { d = set(d, 4) }
            'f' => { d = set(d, 5) }
            'g' => { d = set(d, 6) }
            _ => {}
        }
    }
    d
}
fn set_known(d: u8) -> u8 {
    set(d, 7) 
}
fn set_unknown(d: u8) -> u8 {
    clear(d, 7)
}
fn symdiff(d1: u8, d2: u8) -> u8 {
    set_unknown(d1 ^ d2)
}
fn diff(d1: u8, d2: u8) -> u8 {
    set_unknown(d1 & !d2)
}
pub fn calc_str(strings: String) -> u64 {
    
    //Knowns
    let mut _two:   u8 = 0;
    let mut _three: u8 = 0;
    let mut _four:  u8 = 0;
    let mut _eight: u8 = 0;
    //Unknowns
    let mut _zero:  u8 = 0;
    let mut _one:   u8 = 0;
    let mut _five:  u8 = 0;
    let mut _six:   u8 = 0;
    let mut _seven: u8 = 0;
    let mut _nine:  u8 = 0;
    let r: Vec<&str> = strings.split(" | ").collect();
    let observed: Vec<&str> = r[0].split_whitespace().collect();
    let four_displays: Vec<&str> = r[1].split_whitespace().collect();
    let outputs = four_displays.clone();
    for s in &observed {
        match s.chars().count() {
            2 => { _one   = encode(s);   _one = set_known(_one);  }
            3 => { _seven = encode(s); _seven = set_known(_seven);}
            4 => { _four  = encode(s);  _four = set_known(_four); }
            7 => { _eight = encode(s); _eight = set_known(_eight);}
            _ => { }
        }
    }
    let _m = |n,m| symdiff(n, m);
    let _d = |n,m| diff(n, m);
    for s in &observed {
        let x = encode(s);
        let first  = _d(x, _one).count_ones();
        let second = _d(x, _four).count_ones();
        let third  = _d(x, _seven).count_ones();
        let forth  = _m(x, _eight).count_ones();
        if first == 3 && second == 2 && third == 2 && forth == 2 {
            _three = x;
            _three = set_known(_three);
        }
        if first == 4 && second == 3 && third == 3 && forth == 1 {
            _zero = x;
            _zero = set_known(_zero);
        }
        if first == 4 && second == 3 && third == 3 && forth == 2 {
            _two = x;
            _two = set_known(_two);
        }
        if first == 4 && second == 2 && third == 3 && forth == 2 {
            _five = x;
            _five = set_known(_five);
        }
        if first == 5 && second == 3 && third == 4 && forth == 1 {
            _six = x;
            _six = set_known(_six);
        }
        if first == 4 && second == 2 && third == 3 && forth == 1 {
            _nine = x;
            _nine = set_known(_nine);
        }
    }
    let mut out: String = String::from("");
    for d in &outputs {
        let mut x = encode(*d);
        x = set_known(x);
        match x {
            n if _zero  == n => out.push('0'),
            n if _one   == n => out.push('1'),
            n if _two   == n => out.push('2'),
            n if _three == n => out.push('3'),
            n if _four  == n => out.push('4'),
            n if _five  == n => out.push('5'),
            n if _six   == n => out.push('6'),
            n if _seven == n => out.push('7'),
            n if _eight == n => out.push('8'),
            n if _nine  == n => out.push('9'),
            _ => {}
        }
    }
    let count = out.parse().unwrap();
    count
}

pub fn run() {
    println!("---------------------------------------[AOC8 - Begin]-----------------------------------------");
    let fd = std::fs::File::open("./data/aoc8.txt").unwrap();
    let x = std::io::BufReader::new(fd);
    let now = Instant::now();
    let final_count: u64 = x.lines()
    .filter_map(|line: Result<String, _>| line.ok())
    .par_bridge()
    .map(|s: String| {
        calc_str(s)
    }).reduce(|| 0, |x, y| x + y);
    println!("Final count: {}", final_count);
    println!("Final time (μs): {}", now.elapsed().as_micros());
    println!("---------------------------------------[AOC8 - End]-------------------------------------------");
}
