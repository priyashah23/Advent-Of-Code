use core::num;
use std::collections::HashMap;

fn main(){

}

fn part_one(content: &str) {
    let mut id_block_map: HashMap<i32, i32> = HashMap::new();
    let mut num_of_spaces: Vec<i32> = Vec::new();
    let mut id :i32 = 0;

    for (i, num) in content.chars().enumerate() {
        // even means we add the number of blocks 
        if i % 2 == 0 {
            id_block_map.insert(id, num as i32 - 0x30);
            id += 1;
        } else {
            num_of_spaces.push(num as i32 - 0x30);
        }
    }

    let mut final_file_space: Vec<i32> = Vec::new();
    let mut max_id = id - 1;
    let mut spaces_index = 0;
    let mut start_id = 0;

    while max_id >= 0 && spaces_index < num_of_spaces.len(){
         // first the base case where we have the blocks of 0

        let mut blocks_filled = 0;
        let freq = id_block_map.get(&start_id).unwrap();
        while blocks_filled < *freq {
            final_file_space.push(start_id);
            blocks_filled += 1;
        }
        id_block_map.remove(&start_id);
        
        // read next number of spaces
        let number_of_spaces = num_of_spaces[spaces_index];
        let mut spaces_filled = 0;

        while spaces_filled < number_of_spaces {
            // Check if the current ID has enough blocks
            if let Some(num_of_blocks) = id_block_map.get_mut(&max_id) {
                if *num_of_blocks > 0 {
                    // Use a block
                    final_file_space.push(max_id);
                    *num_of_blocks -= 1;
                    spaces_filled += 1;
                } else {
                    // Move to the next ID
                    id_block_map.remove(&max_id);
                    max_id -= 1;
                }
            } else {
                max_id -= 1;
            }
        }
        start_id += 1;
        spaces_index += 1;
        println!("{:?}", id_block_map);
        println!("{:?}", final_file_space);
    }
    
    println!("{:?}", final_file_space)


}


#[test]
fn test_case() {
    let example = "2333133121414131402";
    part_one(example);
}