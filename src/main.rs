use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn print(p: u8) {
    println!("[{:08b}]", p)
}

fn _toggle(n: u8, k: usize) -> u8 {
    n ^ n << k 
}

fn set(n: u8, k: usize) -> u8 {
    n | 1_u8 << k
}

fn get(n: u8, k: usize) -> u8 {
    (n >> k) & 1_u8
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

fn is_known(d: u8) -> bool {
    get(d, 7) == 1
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

fn calc_str(strings: String) -> u64 {
    
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
    //let all_str = observed.into_iter().chain(four_displays.into_iter()).collect::<Vec<&str>>();   //collect knowns
    for s in &observed {
        match s.chars().count() {
            2 => { _one   = encode(s);   _one = set_known(_one);  }
            3 => { _seven = encode(s); _seven = set_known(_seven);}
            4 => { _four  = encode(s);  _four = set_known(_four); }
            7 => { _eight = encode(s); _eight = set_known(_eight);}
            _ => { }
        }
    }//TODO figure out wtf is going on.
    let _m = |n,m| symdiff(n, m);
    let _d = |n,m| diff(n, m);
    for s in &observed {
        let x = encode(s);
        let first  = _d(x, _one).count_ones();
        let second = _d(x, _four).count_ones();
        let third  = _d(x, _seven).count_ones();
        let forth  = _m(x, _eight).count_ones();
        if first == 3 && second == 2 && third == 2 && forth == 2 {
            _three = x;//done
            _three = set_known(_three);
        }
        if first == 4 && second == 3 && third == 3 && forth == 1 {
            _zero = x;//done
            _zero = set_known(_zero);
        }
        if first == 4 && second == 3 && third == 3 && forth == 2 {
            _two = x;//done
            _two = set_known(_two);
        }
        if first == 4 && second == 2 && third == 3 && forth == 2 {
            _five = x;//done
            _five = set_known(_five);
        }
        if first == 5 && second == 3 && third == 4 && forth == 1 {
            _six = x;//done
            _six = set_known(_six);
        }
        if first == 4 && second == 2 && third == 3 && forth == 1 {
            _nine = x;//done
            _nine = set_known(_nine);
        }
    }
    let mut i = 4;
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
        i -= 1;
    }
    let count = out.parse().unwrap();
    println!("Count: {}", count);
    count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut final_count = 0;
    //let mut i = 0;
    if let Ok(lines) = read_lines("./data/aoc8.txt") {
        for line in lines {
            if let Ok(s) = line {
                //println!("Line: {} ", i);
                //i += 1;
                final_count += calc_str(s);
            }
        }
    }
    println!("Final count: {}", final_count);
}
