use std::io::BufRead;
use std::time::Instant;
use std::cmp;
use std::collections::BinaryHeap;

fn min_3(a: u8, b: u8, c: u8) -> u8 {
    let y = cmp::min(b,c);
    cmp::min(a,y)
}
fn min_2(a: u8, b: u8) -> u8 {
    cmp::min(a,b)
}

fn min(a: u8,b: u8,c: u8,d: u8) -> u8 {
    let x = cmp::min(a,b);
    let y = cmp::min(c,d);
    min_2(x,y)
}
fn not_inside(x: i32, y: i32, grid: Vec<Vec<u8>>, w: u32, h: u32) -> bool {
    x as u32 > w-1 || y as u32 > h-1 || x < 0 || y < 0 || grid[x as usize][y as usize] == 9
}


//Essentially bucket fill algorithm. Could be improved using a scanline method.
fn fill_basin(x: i32, y: i32, grid: &mut Vec<Vec<u8>>, w: u32, h: u32) -> u64 {
    let mut q: BinaryHeap<(i32,i32)> = BinaryHeap::new();
    let mut count = 0;
    q.push((x,y));
    while !q.is_empty() {
        let (x,y) = q.pop().unwrap();
        if !not_inside(x,y, grid.to_vec(), w, h) {
            grid[x as usize][y as usize] = 9;
            count += 1;
            q.push((x, y+1)); 
            q.push((x, y-1));
            q.push((x-1, y));
            q.push((x+1, y));
        }
    }
    if count > 95 {
        println!("--------------[Count: {}]----------------", count);
        print(grid,w,h);
        println!("-----------------------------------------");
    }
    count
}

fn print(grid: &Vec<Vec<u8>>, w: u32, h: u32) {
    for x in 0..w {
        for y in 0..h {
            let v = grid[x as usize][y as usize];
            print!("{}", v);
        }
        println!();
    }
}

fn calc_risk(grid: &mut Vec<Vec<u8>>, w: u32, h: u32) -> u64 {
    let mut up;
    let mut left;
    let mut down;
    let mut right;
    let mut risk: u64 = 0;
    let mut on_edge;
    let mut pts: Vec<(i32, i32)> = Vec::new();
    let mut basin_sizes: BinaryHeap<u64> = BinaryHeap::new();
    for x in 0..w as i32 {
        for y in 0..h as i32 {
            up = 0;
            left = 0;
            down = 0;
            right = 0;
            let v = grid[x as usize][y as usize];
            if x == 0 
            || y == 0 
            || x == (w-1) as i32
            || y == (h-1) as i32 {
                on_edge = true;
            } else {
                on_edge = false;
            }
            if y > 0 {
                up = grid[x as usize][(y-1) as usize];
            }
            if y < (h-1) as i32 {
                down = grid[x as usize][(y+1) as usize];
            }
            if x > 0 {
                left = grid[(x-1) as usize][y as usize];
            }
            if x < (w-1) as i32 {
                right = grid[(x+1) as usize][y as usize];
            }
            if v == 9 {
                continue;
            }
            if x == 0 && y == 0 {
                if v < cmp::min(down, left) {
                    risk += (v + 1) as u64;
                    pts.push((x,y));
                    continue;
                }
                on_edge = false;
            }
            if x == 0 && y == (h-1) as i32 {
                if v < cmp::min(up, right) {
                    risk += (v + 1) as u64;
                    pts.push((x,y));
                    continue;
                }
                on_edge = false;
            }
            if y == 0 && x == (w-1) as i32 {
                if v < cmp::min(down, left) {
                    risk += (v + 1) as u64;
                    pts.push((x,y));
                    continue;
                }
                on_edge = false;
            }
            if x == (w-1) as i32 && y == (h-1) as i32 {
                if v < cmp::min(up, left) {
                    risk += (v + 1) as u64;
                    pts.push((x,y));
                    continue;
                }
                on_edge = false;
            }
            if on_edge {
                if x == 0 {
                    if v < min_3(up, down, right)  {
                        risk += (v + 1) as u64;
                        pts.push((x,y));
                        continue;
                    }
                }
                if y == 0 {
                    if v < min_3(left, down, right)  {
                        risk += (v + 1) as u64;
                        pts.push((x,y));
                        continue;
                    }
                }
                if x == (w-1) as i32 {
                    if v < min_3(left, down, up)  {
                        risk += (v + 1) as u64;
                        pts.push((x,y));
                        continue;
                    }
                }
                if y == (h-1) as i32 {
                    if v < min_3(left, right, up)  {
                        risk += (v + 1) as u64;
                        pts.push((x,y));
                        continue;
                    }
                }
            }
            if v < min(up, down, left, right) {
                risk += (v + 1) as u64;
                pts.push((x,y));
                continue;
            }
        }
    }

    for p in pts {
        let c = fill_basin(p.0,p.1, &mut grid.clone(), w, h);
        basin_sizes.push(c);
    }
    let b1 = basin_sizes.pop();
    let b2 = basin_sizes.pop();
    let b3 = basin_sizes.pop();
    println!("Largest Basins: {} {} {}", b1.unwrap(), b2.unwrap(), b3.unwrap());
    let final_total = b1.unwrap() * b2.unwrap() * b3.unwrap();
    println!("Total: {}", final_total);
    risk
}
pub fn run() {
    println!("---------------------------------------[AOC9- Begin]---------------------------------------- ");
    let fd = std::fs::File::open("./data/aoc9.txt").unwrap();
    let fd2 = std::fs::File::open("./data/aoc9.txt").unwrap();
    let x = std::io::BufReader::new(fd);
    let xb = std::io::BufReader::new(fd2);
    let mut w: u32 = 0;
    let mut h: u32 = 0;
    xb.lines().filter_map(|line: Result<String, _>| line.ok()).for_each(|s: String| {
        w = s.chars().count() as u32;
        h += 1;
    });
    let final_count;
    const RADIX: u32 = 10;
    let mut array = vec![vec![0; h as usize]; w as usize];
    let mut j = 0;

    x.lines().filter_map(|line: Result<String, _>| line.ok()).for_each(|s: String| {
        for (i, c) in s.chars().enumerate() {
            array[i][j] = c.to_digit(RADIX).unwrap() as u8;
        }
        j = j + 1;
    });
    let now = Instant::now();
    final_count = calc_risk(&mut array, w, h);
    println!("Final hazard level: {}", final_count);
    println!("Final time (μs): {}", now.elapsed().as_micros());
    println!("---------------------------------------[AOC9- End]---------------------------------------- ");
}