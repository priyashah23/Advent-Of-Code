use std::io::{self, BufRead};
use std::fs::File;

fn main()  -> io::Result<()>{
    let file = File::open("")?;
    let reader = io::BufReader::new(file);

    let mut left_list: Vec<i64> = Vec::new();    
    let mut right_list: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let current_line = line?;
        let mut iter = current_line.split_whitespace();
        
        match iter.next() {
            Some(x) => {
                let item = x.parse::<i64>().unwrap();
                left_list.push(item);
            },
            None => {},
        };

        match iter.next() {
            Some(y) => {
                let item = y.parse::<i64>().unwrap();
                right_list.push(item);
            },
            None => {},
        };
    }

    let mut total_similarity: i64 = 0;

    for item in left_list {
        let indices: Vec<usize> = right_list.iter().enumerate()
        .filter_map(|(i, x)| if *x == item { Some(i) } else { None })
        .collect();

        let occurance = indices.len() as i64;
        let similarity = item * occurance;
        total_similarity += similarity;
    }
    println!("Total Similarity: {}", total_similarity);






    Ok(())
}