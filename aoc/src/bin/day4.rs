use diagonal::diagonal_pos_pos;
use diagonal::diagonal_pos_neg;
use regex::Regex;

static traverse: [char; 3] = ['M', 'A', 'S'];

fn main() {
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/files/input4.txt";
    let content = std::fs::read_to_string(file).unwrap();
    part_one(&content);
    
}

fn part_one(text: &str) {
    let horizontal = find_horizontal(text);
    let diagonal = find_diaganol(text);
    let vertical = find_vertical(text);
    //find_vertical(&text);
    println!("Horizontal: {horizontal}. Diaganol: {diagonal}, Vertical: {vertical}\n");
    print!("Total: {}", horizontal + diagonal + vertical);

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