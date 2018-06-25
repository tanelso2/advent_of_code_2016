use std::fs::File;
use std::io::prelude::*;

static FILE_NAME: &str = "input.txt";

fn get_input() -> std::io::Result<String> {
    let mut file = File::open(FILE_NAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let input = get_input()?;
    assert_eq!("1985", part1(&String::from("ULL\nRRDDD\nLURDL\nUUUUD\n")));
    let ret = part1(&input);
    println!("The solution to part 1 is {}", ret);
    assert_eq!("5DB3", part2(&String::from("ULL\nRRDDD\nLURDL\nUUUUD\n")));
    let ret2 = part2(&input);
    println!("The solution to part 2 is {}", ret2);
    Ok(())
}


fn part1(input: &String) -> String {
    let keypad: Vec<Vec<&str>> = vec![
        vec!["1", "2", "3"], 
        vec!["4", "5", "6"], 
        vec!["7", "8", "9"]
    ];
    let mut current_pos = (1, 1);
    let mut ret = String::from("");
    for line in input.lines() {
        current_pos = find_pos(line, current_pos, &keypad);
        let (x, y) = current_pos;
        let current_number = keypad[x as usize][y as usize];
        ret.push_str(current_number);
    }
    ret
}

fn part2(input: &String) -> String {
    let keypad: Vec<Vec<&str>> = vec![
        vec!["", "", "1", "", ""],
        vec!["", "2", "3", "4", ""],
        vec!["5", "6", "7", "8", "9"],
        vec!["", "A", "B", "C", ""],
        vec!["", "", "D", "", ""]
    ];
    let mut current_pos = (2, 0);
    let mut ret = String::from("");
    for line in input.lines() {
        current_pos = find_pos(line, current_pos, &keypad);
        let (x, y) = current_pos;
        let current_number = keypad[x as usize][y as usize];
        ret.push_str(current_number);
    }
    ret
}

fn find_pos(line: &str, starting_pos: (i32, i32), keypad: &Vec<Vec<&str>>) -> (i32, i32) {
    line.chars().fold(starting_pos, |acc, x| get_next_pos(x, acc, keypad))
}


fn get_next_pos(dir: char, current_pos: (i32, i32), keypad: &Vec<Vec<&str>>) -> (i32, i32) {
    let (dx, dy) = get_offsets(dir);
    let (x, y) = current_pos;
    let new_x = x + dx;
    let new_y = y + dy;
    if new_x < 0 || new_x as usize >= keypad.len() || new_y < 0 || new_y as usize >= keypad.len() 
        || "" == keypad[new_x as usize][new_y as usize] {
        current_pos
    } else {
        (new_x, new_y)
    }
}

fn get_offsets(dir: char) -> (i32, i32) {
    match dir {
        'U' => (-1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        'D' => (1, 0),
        _ => panic!("Something has gone horribly wrong!")
    }
}
