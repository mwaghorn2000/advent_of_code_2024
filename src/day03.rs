use std::fs;
use regex::Regex;

// enum Event {
//     Do(usize),
//     Dont(usize),
//     Mul(usize, i32, i32),
// }

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
    let mut do_idxs = get_regex_idx(&data, &do_re);
    let mut dont_idxs = get_regex_idx(&data, &dont_re);

    let muls: Vec<(i32, i32)> = get_matches(&data, &mul_re);

    // insert a do at the very start
    do_idxs.insert(0, 0);
    // insert a dont at the very end
    dont_idxs.push(data.len() + 1);
    
    // used to track which do and dont we are up to in our scan
    let mut do_idx = 0;
    let mut dont_idx = 0;

    let mut total = 0;

    /*
        a greedy approach.

        Essentially we ensure we are ahead of a do with no dont's inbetween the do
        and the multiplier. This ensures we are only taking multipliers which are 
        under the effect of a do. 
     */
    for i in 0..muls.len() { 
        let do_plz = do_idxs[do_idx];
        let dont = dont_idxs[dont_idx];

        
        if muls_idxs[i] < dont {
            total += muls[i].0 * muls[i].1;
        } else if !(muls_idxs[i] < do_plz) {
            if do_plz > dont {
                total += muls[i].0 * muls[i].1;
                if dont_idx < dont_idxs.len() - 1 {
                    dont_idx += 1
                }
            } else {
                if do_idx < do_idxs.len() - 1 {
                    do_idx += 1;
                }
            }
        }
    };

    return total
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