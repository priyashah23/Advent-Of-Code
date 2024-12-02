use std::io::{self, BufRead};
use std::fs::File;


fn main()  -> io::Result<()>{
    let file = File::open("/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/aoc/src/files/input2.txt")?;
    let reader = io::BufReader::new(file);

    let mut safe_total = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();
        let iter = current_line.split_whitespace();

        let mut mistakes: i32 = 0;
        let mut tuple = (-1, "BASE");
        let mut safe = true;

        for value in iter {
            let item = value.parse::<i32>().unwrap();
            if tuple.1 == "BASE" && tuple.0 == -1 {
                tuple.0 = item;
                continue;
            } else {
                if (tuple.0 - item).abs() > 3 {
                    if mistakes >= 1 {
                        safe = false;
                        break;
                    }
                    mistakes += 1;
                    continue;
                }
                if tuple.0 == item {
                    if mistakes >= 1 {
                        safe = false;
                        break;
                    }
                    mistakes += 1;
                    continue;
                }
                if tuple.0 > item {
                    if tuple.1 == "BASE" {
                        tuple.1 = "DECREASE";
                    }
                    else if tuple.1 == "INCREASE" {
                        if mistakes >= 1 {
                            safe = false;
                            break;
                        }
                        mistakes += 1;
                        continue;
                    }
                } 
                if tuple.0 < item {
                    if tuple.1 == "BASE" {
                        tuple.1 = "INCREASE";
                    }
                    else if tuple.1 == "DECREASE" {
                        if mistakes >= 1 {
                            safe = false;
                            break;
                        }
                        mistakes += 1;
                        continue;
                    }
                }
                tuple.0 = item;
            }
        }
        if safe == true {
            safe_total += 1;
        }
    }
    println!("Safe: {safe_total}");

    Ok(())
}