use std::{collections::HashMap, fs};

// Assumes that part is either 1 or 2
pub fn run(part: i32) {
    let data = fs::read_to_string("data/input.txt")
        .expect("File not found");

    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();

    convert_data(&mut x, &mut y, data);

    if part == 1 {
        println!("\n--- Day 1 ---\nPart 1: {}\n", part_1(&x, &y));
        return
    } else if part == 2 {
        println!("\n--- Day 1 ---\nPart 2: {}\n", part_2(&x, &y));
    } else {
        println!("Invalid Part!\n");
    }
}

fn parse_to_i32(string: &str) -> i32 {
    match string.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing {}: {}", string, e);
            panic!("Exiting");
        }
    }
}

fn convert_data(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>, data: String) {
    for line in data.lines() {
        let s:Vec<&str> = line.split_whitespace().collect();
        let d1 = parse_to_i32(s[0]);
        let d2 = parse_to_i32(s[1]);

        vec1.push(d1);
        vec2.push(d2);
    }

    vec1.sort();
    vec2.sort();
}

fn part_1(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let mut tally = 0;

    for (count, val) in vec1.iter().enumerate() {
        tally += (val - vec2[count]).abs();
    };

    return tally
}

fn part_2(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for val in vec2 {
        map.entry(*val).and_modify(|x| *x += 1).or_insert(1);
    };

    let mut tally = 0;

    for val in vec1 {
        tally += val * map.get(&val).unwrap_or(&0);
    }

    tally
}

