use std::io::{self, BufRead};
use std::fs::File;

fn main()  -> io::Result<()>{
    let file = File::open("/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/aoc/src/files/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut list_one: Vec<i64> = Vec::new();    
    let mut list_two: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let current_line = line?;
        let mut iter = current_line.split_whitespace();
        
        match iter.next() {
            Some(x) => {
                let item = x.parse::<i64>().unwrap();
                list_one.push(item);
            },
            None => {},
        };

        match iter.next() {
            Some(y) => {
                let item = y.parse::<i64>().unwrap();
                list_two.push(item);
            },
            None => {},
        };
    }

    list_one.sort();
    list_two.sort();

    let mut total_distance: i64 = 0;
    let mut index= 0;
    for line in list_one.iter() {
        let distance = list_two.get(index).unwrap() - line;
        total_distance += distance.abs();
        index+=1;
    }
    print!("{}", total_distance);
    Ok(())
}