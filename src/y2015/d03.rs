use crate::y2015::io::get_contents;
use std::collections::HashSet;

fn new_position(direction: char, position: (i32, i32)) -> (i32, i32) {
    let change = match direction {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, 1),
        'v' => (0, -1),
        _ => (0, 0),
    };

    (position.0 + change.0, position.1 + change.1)
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d03.txt");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut position: (i32, i32) = (0, 0);
    for direction in contents.chars() {
        let new_position = new_position(direction, position);
        visited.insert(new_position);
        position = new_position;
    }

    let unique_visits = visited.len();

    println!("Amount of houses with at least one present: {unique_visits}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d03.txt");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut santa: (i32, i32) = (0, 0);
    let mut robo: (i32, i32) = (0, 0);

    let mut santas_turn: bool = true;
    for direction in contents.chars() {
        let deliverer = match santas_turn {
            true => &mut santa,
            false => &mut robo,
        };

        let new_position = new_position(direction, *deliverer);
        visited.insert(new_position);
        *deliverer = new_position;

        santas_turn = !santas_turn;
    }

    let unique_visits = visited.len();

    println!("Amount of houses with at least one present: {unique_visits}");
}
