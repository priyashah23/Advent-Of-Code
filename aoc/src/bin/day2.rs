
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

fn main(){
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/aoc/src/files/input2.txt";
    let content = std::fs::read_to_string(file).unwrap();

   let total_safe = content.lines()
    .map(|line| line.split_whitespace().map(str::to_string).collect())
    .filter(|vector| is_safe(vector))
    .count();

    print!("{total_safe}");
}