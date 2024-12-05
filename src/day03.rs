use std::fs;
use regex::Regex;

enum Event {
    Do(usize),
    Dont(usize),
    Mul(usize, i32, i32),
}

pub fn run(part: i32) {
    let data = fs::read_to_string("data/input-3.txt")
        .expect("Failed to read file");

    if part == 1 {
        println!("\n--- Day 3 ---\nPart 1: {}\n",part_1(data));
    } else if part == 2 {
        println!("\n--- Day 3 ---\nPart 2: {}\n", part_2(data));
    } else {
        println!("\n--- Day 3 ---\nNo such part\n");
    }
    
}

fn get_product(values: Vec<(i32, i32)>) -> i32 {
    let mut total = 0;

    for item in values {
        total += item.0 * item.1;
    }

    total
}

fn part_1(data: String) -> i32 {
    let re = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)").unwrap();
    
    let matches: Vec<(i32, i32)> = get_matches(&data, &re);

    get_product(matches)
}

fn part_2(data: String) -> i32 {
    // defining necessary regex
    let mul_re = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    // the indexes of all the matchs in the data string
    let muls_idxs = get_regex_idx(&data, &mul_re);
    let do_idxs = get_regex_idx(&data, &do_re);
    let dont_idxs = get_regex_idx(&data, &dont_re);

    let muls: Vec<(i32, i32)> = get_matches(&data, &mul_re);

    let mut events: Vec<Event> = Vec::new();

    for (idx, item) in muls_idxs.iter().enumerate() {
        events.push(Event::Mul(item.clone(), muls[idx].0, muls[idx].1));
    };

    for item in do_idxs {
        events.push(Event::Do(item.clone()));
    };

    for item in dont_idxs {
        events.push(Event::Dont(item.clone()));
    };

    events.sort_by_key(|event| match event {
        Event::Do(pos) | Event::Dont(pos) | Event::Mul(pos, _, _) => *pos,
    });

    let mut dont = false;
    let mut total = 0;

    for event in events {
        match event  {
            Event::Do(_) => dont = false,
            Event::Dont(_) => dont = true,
            Event::Mul(_, x, y) => {
                if !dont {
                    total += x * y; 
                }
            }
        };
    };

    total
}

// returns matches for 'mul(x,y)' in the data as a tuple of values
fn get_matches(data: &String, regex: &Regex) -> Vec<(i32, i32)> {
    let matches: Vec<(i32, i32)> = regex.captures_iter(&data).map(|vals| {
        let val1 =  vals.name("x").unwrap().as_str();
        let val2 = vals.name("y").unwrap().as_str();

        let val1: i32 = match val1.trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("Error with value 1: {}", e),
        };

        let val2: i32 = match val2.trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("Error with value 2: {}", e),
        };

        (val1, val2)
    }).collect();

    matches
}

// gets all the endpoint indexes of a regex and a string.
fn get_regex_idx(data: &String, regex: &Regex) -> Vec<usize> {
    regex.find_iter(&data).map(|mat| mat.end()).collect()
}