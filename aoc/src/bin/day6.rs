#[derive(Debug)]
struct Guard {
    position: (i32, i32),
    direction: (i32, i32)
}

fn main() {
    let file = "/Users/priyashah/Documents/Personal Programming Project/Advent-Of-Code/files/input6.txt";
    let content = std::fs::read_to_string(file).unwrap();
    let result = part_one(&content);
    println!("{result}");
}

fn part_one(content: &str) -> i32 { 
    let mut grid: Vec<Vec<char>> = content
    .lines() // Ignore lines with errors
    .map(|line| line.chars().collect())
    .collect();

    let mut guard = find_guard_pos(&grid);
    let mut possible_directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    
    while !is_at_boundary(&guard, &grid) {
        // check our next position by checking our direction
        let next_pos = (guard.position.0 + guard.direction.0, guard.position.1 + guard.direction.1);
        // check if we have encountered an obstacle
        if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            possible_directions.rotate_left(1);
            guard.direction.0 = possible_directions[0].0;
            guard.direction.1 = possible_directions[0].1;
        } else {
            // set current grid square to be x 
            grid[guard.position.0 as usize][guard.position.1 as usize] = 'X';
            guard.position.0 = next_pos.0;
            guard.position.1 = next_pos.1;
            // set next pos to be current pos
        }
    }
    // set final position
    grid[guard.position.0 as usize][guard.position.1 as usize] = 'X';

    return find_distinct_positions(&grid);
}

fn is_at_boundary(guard: &Guard, grid: &Vec<Vec<char>>) -> bool {
    //TODO - we might need to check the next position... otherwise we will reach a problem
    return guard.position.0 + guard.direction.0 < 0 || guard.position.1 + guard.direction.1 < 0 || (guard.position.0 + guard.direction.0) as usize >= grid.len() || (guard.position.1 + guard.direction.1) as usize >= grid.len()
}

// fn part_two(grid: &Vec<Vec<char>>) {
//     // use the original X positions as possible places for obstacle MAKE SURE ORIGNIAL GUARD POSITION NOT INCLUDED
//     // check if guard is looping... .
    // Optimizations:
    // 1. Put obstacles only on the path from part1, not on the whole field (speedup x4)
    // 2. Remember visited states only on turns, not on every step (speedup x3)
    // 3. Start loop detection from the new obstacle, not from the initial guard position (speedup x3)
    // 4. Jump to the next obstable using precomputed obstacle positions (speedup x3)
    // 5. Track only visited states on turns from UP directions (speedup x2)

// }

fn find_guard_pos(grid: &Vec<Vec<char>>) -> Guard {
    let width = grid.len();
    let height = grid[0].len();
    let mut pos:(i32, i32) = (0, 0);

    for x in 0..width {
        for y in 0..height {
            if grid[x][y] == '^' {
                pos.0 = x as i32;
                pos.1 = y as i32;
                break;
            }
        }
    }

    return Guard {
        position: pos,
        direction : (-1, 0)
    }
}

fn find_distinct_positions(grid: &Vec<Vec<char>>) -> i32 {
    let width = grid.len();
    let height = grid[0].len();

    let mut total = 0;
    for x in 0..width {
        for y in 0..height {
            if grid[x][y] == 'X' {
                total += 1
            }
        }
    }
    total
}

#[test]
fn test() {
    let map = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
    let result  =part_one(map);
    assert_eq!(result, 41);

}