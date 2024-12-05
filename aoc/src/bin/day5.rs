use std::collections::{HashMap, HashSet};

fn main() {
    // let file = "";
    // let content = std::fs::read_to_string(file).unwrap();

    
    // for line in content.lines() {
    //     let (left, right) = line.split_once("   ").unwrap();
    //     left_list.push(left.parse::<i64>().unwrap());
    //     right_list.push(right.parse::<i64>().unwrap());
    // }   
}

fn part_one(text: &str) {

// Now we need to make a hash map... { number | valid numbers that can come before it}
 let mut instruction_map: HashMap<&str, Vec<&str>>= HashMap::new();

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
            let valid = check_valid(order, &instruction_map);  
        }  
    }

}

fn check_valid(order: Vec<&str>, map:&HashMap<&str, Vec<&str>>) {
    for i in 0..order.len() {
        let num = order.get(i).unwrap();
        let range = &order[i + 1..order.len()];
        if range.is_empty() {
            break
            // return true
        }
        let hashset_1: HashSet<&str> = range.iter().cloned().collect();
        
        if let Some(x) = map.get(num)  { 
            let vector = x;
            let hashset_2: HashSet<&str> = vector.iter().cloned().collect();
            let intersection = hashset_1.intersection(&hashset_2);
            println!("{:?}, {:?}, {:?}", hashset_1, hashset_2, intersection);
        } else {
            break;
        }

    }

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

part_one(test);

}