use diagonal::diagonal_pos_pos;
use diagonal::diagonal_pos_neg;

use regex::Regex;


fn main() {
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/files/input4 2.txt";
    let content = std::fs::read_to_string(file).unwrap();
    let total = part_two(&content);
    part_one(&content);
    println!("{total}")
}

fn part_one(text: &str) {
    let horizontal = find_horizontal(text);
    let diagonal = find_diaganol(text);
    let vertical = find_vertical(text);
    //find_vertical(&text);
    println!("Horizontal: {horizontal}. Diaganol: {diagonal}, Vertical: {vertical}\n");
    print!("Total: {}", horizontal + diagonal + vertical);

}

fn part_two(text: &str) -> i32{
    let grid: Vec<Vec<char>> = text
    .lines() // Ignore lines with errors
    .map(|line| line.chars().collect())
    .collect();

    let mut total = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 'A' {
                let x_coord = x as i32;
                let y_coord = y as i32;

                if x_coord - 1 < 0 || y_coord - 1 < 0 || x + 1 as usize >= grid.len() || y+ 1 >= grid[0].len() {
                    // out of bounds
                    continue;
                }

                // let mut left_to_right: String;
                let left_to_right = format!("{}{}{}", grid[x-1][y-1], grid[x][y], grid[x+1][y+1]);
                let right_to_left = format!("{}{}{}", grid[x-1][y+1], grid[x][y], grid[x+1][y-1]);

                if (right_to_left == "MAS" || right_to_left == "SAM") && (left_to_right == "MAS" || left_to_right == "SAM"){
                    total += 1
                }
            }

        }
    }
    return total


}


fn find_horizontal(text: &str) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    let re_rev = Regex::new(r"SAMX").unwrap();
    let horizontal_total = text
    .lines()
    .map(|line| {
        re.find_iter(line).count() + re_rev.find_iter(line).count()
    }).sum();
    horizontal_total
}

fn find_vertical(text: &str) -> usize{
    let re = Regex::new(r"XMAS").unwrap();
    let re_rev = Regex::new(r"SAMX").unwrap();
    let mut total = 0;
    
    let grid: Vec<Vec<char>> = text
    .lines() // Ignore lines with errors
    .map(|line| line.chars().collect())
    .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let transposed: Vec<Vec<_>> = (0..cols).map(|col| {
    (0..rows)
        .map(|row| grid[row][col])
        .collect()
    }).collect();

    for i in 0..transposed.len() {
        let string: String = transposed[i].iter().collect();
        total += re.find_iter(&string).count() + re_rev.find_iter(&string).count()

    }
    total
}

fn find_diaganol(text: &str) -> usize{
    let re = Regex::new(r"XMAS").unwrap();
    let re_rev = Regex::new(r"SAMX").unwrap();
    let mut total = 0;
    
    let grid: Vec<Vec<char>> = text
    .lines() // Ignore lines with errors
    .map(|line| line.chars().collect())
    .collect();

    let pos_diag = diagonal_pos_pos(&grid);
    let neg_diag = diagonal_pos_neg(&grid);


    for i in 0..pos_diag.len() {
        let string: String = pos_diag[i].clone().into_iter().collect();
        total += re.find_iter(&string).count() + re_rev.find_iter(&string).count()
    }

    for i in 0..neg_diag.len() {
        let string: String = neg_diag[i].clone().into_iter().collect();
        total += re.find_iter(&string).count() + re_rev.find_iter(&string).count()
    }
    total
    
}

#[test]
fn test_part_one() {
    // can count multiple occurances of Xmas 

    let example = 
    "..X..\n.SAMX.\n.A..A\nXMAS.S\n.X....";

    let test_case =
    "...XXMAS..\n.SAMXMS...\n...S..A...\n..A.A.MS.X\nXMASAMX.MM\nX.....XA.A\nS.S.S.S.SS\n.A.A.A.A.A\n..M.M.M.MM\n.X.X.XMASX";

    // part_one(example);
    part_one(test_case);
}

#[test]
fn test_part_two() {
    // can count multiple occurances of Xmas 
    let test_case =".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........";
    

    // part_one(example);
    let total: i32 = part_two(&test_case);
    assert_eq!(total, 9);
}