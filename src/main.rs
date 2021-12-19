
fn print(p: u8) {
    println!("[{:08b}]", p)
}

fn toggle(n: u8, k: usize) -> u8 {
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

fn set_if(d: u8, mask: u8, b: bool) -> u8 {
    d ^ (d & !mask) | ((!b as u8) & mask)
}

fn new_from_str(s: &str) -> u8 {
    let mut value = 0;
    match s.chars().count() {
        2 => { value = encode(&s) }
        3 => { value = encode(&s) }
        4 => { value = encode(&s) }
        8 => { value = encode(&s) }
        _ => { /* Do nothing */   }
    }
    value
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
fn compliment(d: u8) -> u8 {
    set_unknown(!d)
}
fn plus(d1: u8, d2: u8) -> u8 {
    set_unknown(d1 | d2)
}
fn intersect(d1: u8, d2: u8) -> u8 {
    set_unknown(d1 & d2)
}
fn diff(d1: u8, d2: u8) -> u8 {
    set_unknown(d1 & !d2)
}

fn symdiff(d1: u8, d2: u8) -> u8 {
    set_unknown(d1 ^ d2)
}

fn main() {
    let str_eg: String = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");

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

    let r: Vec<&str> = str_eg.split(" | ").collect();
    let observed: Vec<&str> = r[0].split_whitespace().collect();
    let _four_displays: Vec<&str> = r[1].split_whitespace().collect();
    
    for s in &observed {
        let d: u8 = encode(s);
        print(d);
        match s.chars().count() {
            2 => { _one   = encode(s);   _one = set_known(_one);  }
            4 => { _four  = encode(s);  _four = set_known(_four); }
            3 => { _seven = encode(s); _seven = set_known(_seven);}
            7 => { _eight = encode(s); _eight = set_known(_eight);}
            6 => {   
                    if !is_known(_zero) {
                        _zero = encode(s);
                        if _zero == plus(diff(diff(_eight.clone(), _one.clone()),_four.clone()), diff(_eight.clone(),_two.clone())) {
                            _zero = set_known(_zero);
                            break;
                        }
                    }
                    if !is_known(_six) {
                        _six = encode(s);
                        if _six == 99 {}//TODO work out set logic
                    }
                    if !is_known(_nine) {
                        _nine = encode(s);
                        if _nine == 99 {}// TODO work out set logic
                    }
            }
            5 => {   
                    if !is_known(_two) {
                        _two = encode(s);
                        if _two == 99 {}//TODO work out set logic
                    }
                    if !is_known(_five) {
                        _five = encode(s);
                        if _five == 99 {}//TODO work out set logic
                    }
                    if !is_known(_three) {
                        _three = encode(s);
                        if _three == plus(symdiff(_eight, plus(symdiff(_eight,_five),symdiff(_eight,_two))), _one) {
                            _three = set_known(_three);
                        }
                    }
                }
            _ => {}
        }
    }
    
    
    // _zero = set_known(_zero.clone());
    // let y = union(diff(diff(_eight.clone(), _one.clone()),_four.clone()), diff(_eight.clone(),_two.clone()));
    // if y != _zero.clone() {
    //     _zero = set_unknown(_zero.clone());
    // }
    // let x = union(diff(_eight.clone(), union(diff(_eight.clone(),_five.clone()),diff(_eight.clone(),_two.clone()))), _one.clone());
    // if x != _three {
    //     _three = set_unknown(_three.clone());
    // }

    if is_known(_zero) {
        println!("0: ");
        print(_zero);
    }
    if is_known(_three) {
        println!("3: ");
        print(_three);
    }
    if is_known(_one) {
        println!("1: ");
        print(_one);
    }
    if is_known(_four) {
        println!("4: ");
        print(_four);
    }
    if is_known(_seven) {
        println!("7: ");
        print(_seven);
    }
    if is_known(_eight) {
        println!("8: ");
        print(_eight);
    }


    //let mut x = 0b0000_0001;
    //x = set_known(x);
    //reminder that I am an idiot
}
