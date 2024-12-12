use std::{collections::HashSet, thread::current};

fn main() {
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/files/input7.txt";
    let content = std::fs::read_to_string(file).unwrap();
    let result = part_one(&content, false);
    let new_result = part_one(&content, true);
    println!("{new_result}");
}

fn part_one(text: &str, is_part_two: bool) -> i64 {
    let mut solutions:HashSet<i64> = HashSet::new();

    for line in text.lines() {
        let values = line.split_once(':').unwrap();
        let test_value = values.0.parse::<i64>().unwrap();
        let nums: Vec<i64> = values.1
        .trim_start()
        .split_ascii_whitespace()
        .map(|value| value.parse::<i64>().unwrap()).collect();
        
        find_solution(&nums, 0, test_value, 0, &mut solutions, is_part_two);


        
    }
    return solutions.iter().sum();

}

fn find_solution(nums: &Vec<i64>, current_index: usize, target: i64, current_sum: i64, set: &mut HashSet<i64>, is_part_two: bool) {
    // check if index is still valid
    if current_index >= nums.len() {
        if current_sum == target {
            set.insert(current_sum);
        }
        // stop algorithm!
        return
    }
    let current_num = nums[current_index];
        // Basic operations: addition and multiplication without concatenation
        find_solution(nums, current_index + 1, target, current_sum + current_num, set, is_part_two); // +
        find_solution(nums, current_index + 1, target, current_sum * current_num, set, is_part_two); // *
            

    if is_part_two && current_index > 0 {
        // Concatenation logic
        let concatenated = format!("{}{}", current_sum, current_num)
            .parse::<i64>()
            .unwrap();
        find_solution(nums, current_index + 1, target, concatenated, set, is_part_two); // ||

    }
    
}

#[test]
fn help () {
    let mut solutions:HashSet<i64> = HashSet::new();
    let nums: Vec<i64> = vec![11, 6, 16, 20];
    find_solution(&nums, 0, 292, 0, &mut solutions, true);
}   


#[test]
fn test() {
    let test_instructions: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let result  =part_one(test_instructions, false);
    assert_eq!(result, 3749)
}

#[test]
fn test_part_two() {
    let test_instructions: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let result = part_one(test_instructions, true);
    assert_eq!(result, 11387)
}