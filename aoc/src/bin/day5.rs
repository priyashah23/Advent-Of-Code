use std::collections::{HashMap, HashSet};

fn main() {
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/files/input5.txt";
    let content = std::fs::read_to_string(file).unwrap();

    let result = part_one(&content);
    println!("{result}")
}

fn part_one(text: &str) -> i32 {

 let mut instruction_map: HashMap<&str, Vec<&str>>= HashMap::new();
 let mut total = 0;

    for line in text.lines() {
        // first we do something until we encounter a ''
        if line.is_empty() {
            continue;
        }

        if line.contains('|') {
            let mut splitter = line.split('|');
            let (X, Y) = (splitter.next().unwrap(), splitter.next().unwrap());

            if let Some(vec) = instruction_map.get_mut(&X) {
                vec.push(&Y);
            } else {
                instruction_map.insert(X, vec![Y]);
            }
        } else {
            let order  = line.split(',').collect::<Vec<&str>>();
            let copy = order.clone();
            let valid = check_valid(order, &instruction_map);
            if valid {
                let value = copy[copy.len().div_ceil(2) - 1];
                let num = value.parse::<i32>().unwrap();
                total += num;
            }
        }  
    }
    total
}

fn check_valid(order: Vec<&str>, map:&HashMap<&str, Vec<&str>>) -> bool{
    let mut is_valid = true;
    
    for i in 0..order.len() {
        let num = order.get(i).unwrap();
        let range = &order[i + 1..order.len()];
        if range.is_empty() {
            is_valid = true;
            break;
        }
        let hashset_1: HashSet<&str> = range.iter().cloned().collect();
        
        if let Some(x) = map.get(num)  { 
            let vector = x;
            let hashset_2: HashSet<&str> = vector.iter().cloned().collect();
            let intersection: HashSet<_> = hashset_1.intersection(&hashset_2).cloned().collect();
            let is_subset = hashset_1.is_subset(&intersection);
            if !is_subset {
                is_valid = false;
                break;
            }
        
        } else {
            // the number does not exist in the order of instructions thus is wrong
            is_valid = false;
            break;
        }

    }
    is_valid
}


#[test]
fn part_one_test() {
    let test = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

let result = part_one(test);
assert_eq!(result, 143);

}