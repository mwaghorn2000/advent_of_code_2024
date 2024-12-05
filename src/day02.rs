use std::fs;

pub fn run(part: i32) {
    let data = fs::read_to_string("data/input-2.txt")
        .expect("Failed to find file");

    let reports = process_data(data);

    let mut safe_reports_part_1 = 0;
    let mut safe_reports_part_2 = 0;

    generate_safe_reports(&mut safe_reports_part_1, &mut safe_reports_part_2, reports);

    if part == 1 {
        println!("\n--- Day 2 ---\nPart 1: {}\n", safe_reports_part_1);
    } else if part == 2 {
        println!("\n--- Day 2 ---\nPart 2: {}\n", safe_reports_part_2);
    }
}

fn generate_safe_reports(
    safe_reports_part_1: &mut i32,
    safe_reports_part_2: &mut i32,
    reports: Vec<Vec<i32>>
) {
    for report in reports {
        let cols = 2;
        let rows = report.len();
        
        // dp initilisation
        let mut dp: Vec<Vec<bool>> = vec![vec![false; cols]; rows + 1];
        // Track if at this point in the dp, are we increasing
        let mut inc: Vec<Vec<bool>> = vec![vec![false; cols]; rows + 1];
        // Track if at this point in the dp, are we decreasing
        let mut dec: Vec<Vec<bool>> = vec![vec![false; cols]; rows + 1];
        
        // base cases - we set inc and dec to true since at the first element we can
        // either increase of decrease of this element.
        for i in 0..=1 {
            for j in 0..=1 {
                inc[i][j] = true;
                dec[i][j] = true;
                dp[i][j] = true;
            }
        }

        for i in 2..=rows {
            if dp[i - 1][0] && is_valid(report[i - 2], report[i - 1], inc[i - 1][0], dec[i - 1][0]) {
                inc[i][0] = is_increasing(report[i - 2], report[i - 1]);
                dec[i][0] = !is_increasing(report[i - 2], report[i - 1]);
                dp[i][0] = true;
            }

            if dp[i][0] {
                dp[i][1] = true;
                inc[i][1] = inc[i][0];
                dec[i][1] = dec[i][0];
            }

            if dp[i - 1][1] && is_valid(report[i - 2], report[i - 1], inc[i - 1][1], dec[i - 1][1]) {
                dp[i][1] = true;
                inc[i][1] = is_increasing(report[i - 2], report[i - 1]) || inc[i][1];
                dec[i][1] = !is_increasing(report[i - 2], report[i - 1]) || dec[i][1];
            }

            // need to check if we are hitting our dummy value. In this case
            // we are simulating removing the first element
            if i == 2 {
                dp[i][1] = true;
                inc[i][1] = true;
                dec[i][1] = true;
            } else if dp[i - 2][0] && is_valid(report[i - 3], report[i - 1], inc[i - 2][0], dec[i - 2][0]) {
                dp[i][1] = true;
                inc[i][1] = is_increasing(report[i - 3], report[i - 1]) || inc[i][1];
                dec[i][1] = !is_increasing(report[i - 3], report[i - 1]) || dec[i][1]; 
            }
        }

        if dp[report.len()][0] {
            *safe_reports_part_1 += 1;
        } 
        if dp[report.len()][1] || dp[report.len() - 1][0] {
            *safe_reports_part_2 += 1;
        }
    }
}

fn is_valid(prev: i32, curr: i32, inc: bool, dec: bool) -> bool {
    if inc && prev < curr && (curr - prev).abs() <= 3 {
        return true
    }

    if dec && prev > curr && (prev - curr).abs() <= 3 {
        return true
    }

    return false
}

fn is_increasing(prev: i32, curr: i32) -> bool {
    prev < curr
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