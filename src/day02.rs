use std::fs;

pub fn run(part: i32) {
    let data = fs::read_to_string("data/input-2.txt")
        .expect("Failed to find file");

    let reports = process_data(data);

    let safe_reports_part_1 = 0;
    let safe_reports_part_2 = 0;

    generate_safe_reports(safe_reports_part_1: &i32, safe_reports_part_2: &i32, reports);

    if part == 1 {
        println!("\n--- Day 2 ---\nPart 1: {}\n", safe_reports_part_1);
    } else if part == 2 {
        println!("\n--- Day 2 ---\nPart 2: {}\n", safe_reports_part_2);
    }
}

fn generate_safe_reports(
    safe_reports_part_1: &i32,
    safe_reports_part_2: &i32,
    reports: Vec<Vec<i32>>
) {
    for report in reports {
        let cols = 2;
        let rows = report.len();
        
        // dp initilisation
        let mut dp: Vec<Vec<bool>> = vec![vec![false; cols]; rows + 1];

        // base cases
        dp[0][0] = true;
        dp[1][0] = true;
        dp[0][1] = true;
        dp[1][1] = true;

        for i in 1..rows {
            
        }
    }
}

fn check_increasing(value_1: i32, value_2: i32) -> bool {
    value_1 < value_2
}

fn check_safe(value_1: i32, value_2: i32) -> bool {
    if 
}

fn process_data(data: String) -> Vec<Vec<i32>> {

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        let level: Vec<&str> = line.split_whitespace().collect();
        let level: Result<Vec<i32>, String> = level.iter()
            .map(|s| s.trim().parse::<i32>()
            .map_err(|e| format!("Failed to parse '{}': {}", s, e)))
            .collect();

        let level = level.unwrap();

        reports.push(level);
    }

    reports
}