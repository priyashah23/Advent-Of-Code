use std::result;

use regex::Regex;
use diagonal::{diagonal_pos_neg, diagonal_pos_pos};

fn main() {
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/files/input4.txt";
    let content = std::fs::read_to_string(file).unwrap();
}

fn part_one(text: &str) {
    let grid: Vec<Vec<String>> = text
        .lines() // Ignore lines with errors
        .map(|line| line.split_whitespace().map(String::from).collect())
        .collect();

    find_diaganol(&text);
    let total = find_horizontal(text) + find_vertical(&grid);
    print!("{total}");

}

fn find_horizontal(text: &str) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    let horizontal_total = text
    .lines()
    .map(|line| {
        let reversed: String = line.chars().rev().collect();
        re.find_iter(line).count() + re.find_iter(&reversed).count()
    }).sum();
    horizontal_total
}

fn find_vertical(grid: &Vec<Vec<String>>) -> usize {
    // Iterate through each column
    let mut total = 0;

    for x in 0..grid.len() {
        let mut vertical_line = String::new();
        for y in 0..grid.len() {
            let current_row = &grid[y][0];
            let vertical = current_row.chars().nth(x).unwrap();
            vertical_line.push(vertical);
            
        }
        let current_total = find_horizontal(&vertical_line);
        total += current_total;
    }
    total
}

fn find_diaganol(text: &str)  {
    let grid: Vec<Vec<char>> = text
    .lines() // Ignore lines with errors
    .map(|line| line.chars().collect())
    .collect();

    let result_pos = diagonal_pos_pos(&grid);
    let result_neg = diagonal_pos_neg(&grid);
    
    for vec in result_pos {
        let string: String = vec.into_iter().collect();
        println!("{}", string);
    }

}

#[test]
fn test_part_one() {
    // can count multiple occurances of Xmas 

    let example = 
    "..X..\n.SAMX.\n.A..A\nXMAS.S\n.X....";

    let test_case =
    "...XXMAS..
    .SAMXMS...
    ...S..A...
    ..A.A.MS.X
    XMASAMX.MM
    X.....XA.A
    S.S.S.S.SS
    .A.A.A.A.A
    ..M.M.M.MM
    .X.X.XMASX";

    part_one(example);
    // part_one(test_case);
}