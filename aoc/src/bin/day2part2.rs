use std::io::{self, BufRead};
use std::fs::File;

fn is_safe(levels: &Vec<String>) -> bool {
    let mut status = "BASE";
    let mut prev_number = -1;

    for item in levels {
        let num = item.parse::<i32>().unwrap();
        if prev_number == -1 {
            prev_number = num;
            continue;
        }
        else {
            if (prev_number - num).abs() > 3 {
                return false;
            }
            if prev_number == num {
                return false;
            }
            if prev_number > num {
                if status == "BASE" {
                    status = "DECREASE";
                }
                else if status == "INCREASE" {
                    return false;
                }
            } 
            if prev_number < num {
                if status == "BASE" {
                    status = "INCREASE";
                }
                else if status == "DECREASE" {
                    return false;
                }
            }
            prev_number = num;
        }
    }
    return true;
}

fn main()  -> io::Result<()>{
    let file = File::open("")?;
    let reader = io::BufReader::new(file);

    let mut safe_total = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();
        let vector : Vec<String> = current_line.split_whitespace().map(str::to_string).collect();
        let safe: bool = is_safe(&vector);
        println!("{:?}:Safe: {safe}", vector);
        if safe == true {
            safe_total += 1;
        }
        else {
            let mut dampner_safe = false;
            for i in 0..vector.len() {
                let mut temp_levels = vector.clone();
                temp_levels.remove(i);
                if is_safe(&temp_levels) {
                    dampner_safe = true;
                    break;
                }
            }
            if dampner_safe == true {
                safe_total += 1;
            }
        }
    }

    println!("Safe: {safe_total}");
    Ok(())
}