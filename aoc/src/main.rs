use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

static VALUE: i32 = 2020; 

fn main() -> std::io::Result<()> {
    let file = File::open("/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/aoc/src/source-files/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut complement_map = HashMap::new();
    // let mut total:i32 = 0;

    for line in reader.lines() {
        let current_value:i32  = line?.parse::<i32>().unwrap();
        let complement: i32 = VALUE - current_value;
        // check if complement is in hashmap
        if complement_map.contains_key(&complement) {
            print!("Values Found {complement}, {current_value}\n");
            complement_map.get(&complement);
            let total: i32 = complement * current_value;
            print!("{total}");
        }
        else {
            complement_map.insert(current_value, complement);
        }
    }
    Ok(())
}

