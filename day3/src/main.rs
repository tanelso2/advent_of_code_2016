extern crate itertools;

use itertools::Itertools;
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
    let ret = part1(&input);
    println!("The solution to part 1 is {}", ret);
    let ret2 = part2(&input);
    println!("The solution to part 2 is {}", ret2);
    Ok(())
}

fn is_triangle(lengths: (i32, i32, i32)) -> bool {
    let (a, b, c) = lengths;
    a + b > c && a + c > b && b + c > a
}

fn parse_line(line: &str) -> (i32, i32, i32) {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    if numbers.len() == 3 {
        (numbers[0], numbers[1], numbers[2])
    } else {
        panic!("This shouldn't happen")
    }
}

fn part1(input: &String) -> i32 {
    input
        .lines()
        .map(|x| parse_line(x))
        .filter(|x| is_triangle(*x))
        .count() as i32
}

fn part2(input: &String) -> i32 {
    input
        .lines()
        .map(|x| parse_line(x))
        .tuples::<(_, _, _)>()
        .flat_map(|x| {
            let (a, b, c) = x;
            let (a1, a2, a3) = a;
            let (b1, b2, b3) = b;
            let (c1, c2, c3) = c;
            vec![(a1, b1, c1), (a2, b2, c2), (a3, b3, c3)]
        })
        .filter(|x| is_triangle(*x))
        .count() as i32
}
