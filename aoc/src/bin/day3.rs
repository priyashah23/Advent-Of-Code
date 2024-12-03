use regex::Regex;

fn main() {
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/aoc/src/files/input3.txt";
    let content = std::fs::read_to_string(file).unwrap();

    let part_one_total: i32 = part_one(&content);
    let part_two_total: i32 = part_two(&content);
    println!("Part One: {part_one_total}");
    println!("Part Two: {part_two_total}");

}

fn part_one (text: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let elements : Vec<&str> = re.captures_iter(text).map(|m| m.get(0).unwrap().as_str()).collect();
    
    let interg = Regex::new(r"\d+").unwrap();

    let mut total = 0;

    for x in elements {
        let numbers : Vec<&str> = interg.captures_iter(x).map(|m| m.get(0).unwrap().as_str()).collect();
        let first_num = numbers[0].parse::<i32>().unwrap();
        let second_num = numbers[1].parse::<i32>().unwrap();
        total += first_num * second_num;
    }

    total
}

fn part_two(text: &str) -> i32 {
    let stuff = Regex::new(r"do(n't)*").unwrap();
    // then we append 'do' at the beginning
    let mut instructions : Vec<&str> = stuff.captures_iter(text).map(|m| m.get(0).unwrap().as_str()).collect();
    let elements : Vec<&str> = stuff.split(text).collect();

    instructions.insert(0, "do");

    let total: i32 = instructions
    .iter()
    .zip(elements.iter())
    .filter(|(instruct, _)| **instruct == "do")
    .map(|(&_, &element)|  part_one(element))
    .sum();

    total
}

#[test]
fn test_part_one() {
    let test_case = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"; 
    let test_case_2 = "mul(4*";
    let test_case_3 = "mul(6,9!";
    let test_case_4 = "?(12,34)";
    let test_case_5 = "mul ( 2 , 4 )";


    let result: i32 = part_one(test_case);
    let result2: i32 = part_one(test_case_2);
    let result3: i32 = part_one(test_case_3);
    let result4: i32 = part_one(test_case_4);
    let result5: i32 = part_one(test_case_5);

    assert_eq!(161, result);
    assert_eq!(0, result2);
    assert_eq!(0, result3);
    assert_eq!(0, result4);
    assert_eq!(0, result5);
}

#[test]
fn test_part_two() {
    let test_case = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let result: i32 = part_two(&test_case);
    assert_eq!(48, result);
}