use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp;

static FILE_NAME: &str = "input.txt";

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

struct Position {
    x: i32,
    y: i32,
    current_direction: Direction
}

fn turn_left(dir: &Direction) -> Direction {
    match dir {
        Direction::NORTH => Direction::WEST,
        Direction::WEST => Direction::SOUTH,
        Direction::SOUTH => Direction::EAST,
        Direction::EAST => Direction::NORTH
    }
}

fn turn_right(dir: &Direction) -> Direction {
    match dir {
        Direction::NORTH => Direction::EAST,
        Direction::EAST => Direction::SOUTH,
        Direction::SOUTH => Direction::WEST,
        Direction::WEST => Direction::NORTH
    }
}

fn main() -> std::io::Result<()> {
    let input = get_input()?;
    assert_eq!(5, part1(&String::from("R2, L3")));
    assert_eq!(2, part1(&String::from("R2, R2, R2")));
    assert_eq!(12, part1(&String::from("R5, L5, R5, R3")));
    let ret = part1(&input);
    println!("The solution to part 1 is {}", ret);
    assert_eq!(4, part2(&String::from("R8, R4, R4, R8")));
    let ret2 = part2(&input);
    println!("The solution to part 2 is {}", ret2);
    Ok(())
}

fn part1(input: &String) -> i32 {
    let instructions = input.split(", ");
    let mut position = Position { x: 0, y: 0, current_direction: Direction::NORTH};
    let final_pos = instructions.fold(&mut position, |acc, x| do_instruction(acc, x.to_string()));
    manhattan_distance(final_pos)
}

fn part2(input: &String) -> i32 {
    let position = get_first_revisted_location(input);
    manhattan_distance(&position)
}

fn get_first_revisted_location(input: &String) -> Position {
    let instructions = input.split(", ");
    let mut position: Position = Position { x: 0, y: 0, current_direction: Direction::NORTH};
    let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();
    visited_locations.insert((0, 0));
    for instruction in instructions {
        let original_loc = (position.x, position.y);
        do_instruction(&mut position, instruction.to_string());
        let current_loc = (position.x, position.y);
        let min_x = cmp::min(current_loc.0, original_loc.0);
        let max_x = cmp::max(current_loc.0, original_loc.0);
        let min_y = cmp::min(current_loc.1, original_loc.1);
        let max_y = cmp::max(current_loc.1, original_loc.1);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let location = (x, y);
                if location == original_loc {
                    // Original location is already in the set, so don't check it
                    continue;
                }
                if visited_locations.contains(&location) {
                    return Position {x: location.0, y: location.1, ..position};
                }
                visited_locations.insert(location);
            }
        }
        visited_locations.insert(current_loc);
    }
    position
}

fn do_instruction(pos: &mut Position, instruction: String) -> &mut Position {
    let (turning_direction, steps_str) = instruction.split_at(1);
    let turning_function = match turning_direction {
        "L" => turn_left,
        "R" => turn_right,
        _ => panic!("That's not a real direction!")
    };
    let steps: i32 = match steps_str.parse() {
        Ok(x) => x,
        Err(e) => panic!(e.to_string())
    };
    pos.current_direction = turning_function(&pos.current_direction);
    let (dx, dy) = get_distance_change(&pos.current_direction, steps);
    pos.x += dx;
    pos.y += dy;
    pos
}

fn get_distance_change(dir: &Direction, steps: i32) -> (i32, i32) {
    match dir {
        Direction::NORTH => (0, steps),
        Direction::EAST => (steps, 0),
        Direction::SOUTH => (0, -steps),
        Direction::WEST => (-steps, 0) 
    }
}

fn manhattan_distance(pos: &Position) -> i32 {
    pos.x.abs() + pos.y.abs()
}

fn get_input() -> std::io::Result<String> {
    let mut file = File::open(FILE_NAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}