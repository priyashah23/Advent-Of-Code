use std::io::{self, BufRead};
use std::fs::File;

fn main()  -> io::Result<()>{
    let file = File::open("")?;
    let reader = io::BufReader::new(file);

    let mut safe_total = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();
        let iter = current_line.split_whitespace();
        
        let mut tuple = (-1, "BASE");
        let mut safe = true;

        for value in iter {
            let item = value.parse::<i32>().unwrap();
            if tuple.1 == "BASE" && tuple.0 == -1 {
                tuple.0 = item;
                continue;
            } else {
                if (tuple.0 - item).abs() > 3 {
                    safe = false;
                    break;
                }
                if tuple.0 == item {
                    safe = false;
                    break;
                }
                if tuple.0 > item {
                    if tuple.1 == "BASE" {
                        tuple.1 = "DECREASE";
                    }
                    else if tuple.1 == "INCREASE" {
                        safe = false;
                        break;
                    }
                } 
                if tuple.0 < item {
                    if tuple.1 == "BASE" {
                        tuple.1 = "INCREASE";
                    }
                    else if tuple.1 == "DECREASE" {
                        safe = false;
                        break;
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