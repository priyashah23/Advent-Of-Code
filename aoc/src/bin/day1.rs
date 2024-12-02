
fn main() {
    let file = "";
    let content = std::fs::read_to_string(file).unwrap();

    let mut left_list: Vec<i64> = Vec::new();    
    let mut right_list: Vec<i64> = Vec::new();

    for line in content.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        left_list.push(left.parse::<i64>().unwrap());
        right_list.push(right.parse::<i64>().unwrap());
    }

    left_list.sort();
    right_list.sort();
    part_one(&left_list, &right_list);
    part_two(&left_list, &right_list);
}

fn part_one(left_list: &Vec<i64>, right_list: &Vec<i64>) {
    let total_distance: i64 = left_list.iter()
    .zip(right_list.iter())
    .map(|(l, r) | (r - l).abs())
    .sum();

    println!("{}", total_distance);
}

fn part_two(left_list: &Vec<i64>, right_list: &Vec<i64>) {
    let mut total_similarity: i64 = 0;

    for item in left_list {
        let indices: Vec<usize> = right_list.iter().enumerate()
        .filter_map(|(i, x)| if *x == *item { Some(i) } else { None })
        .collect();

        let occurance = indices.len() as i64;
        let similarity = item * occurance;
        total_similarity += similarity;
    }
    println!("Total Similarity: {}", total_similarity);
}