use crate::y2015::io::get_contents;
use std::cmp::max;

enum Action {
    On,
    Off,
    Toggle,
}

struct Command {
    action: Action,
    from: (u16, u16),
    to: (u16, u16),
}

fn read_coordinates(s: &str) -> (u16, u16) {
    let mut coordinates = s.split(',');

    let x = coordinates.next().unwrap().parse().unwrap();
    let y = coordinates.next().unwrap().parse().unwrap();

    (x, y)
}

fn string_to_command(s: &str) -> Command {
    let mut parts = s.split_whitespace();

    let action = match parts.next().unwrap() {
        "toggle" => Action::Toggle,
        "turn" => match parts.next().unwrap() {
            "on" => Action::On,
            "off" => Action::Off,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    let from = read_coordinates(parts.next().unwrap());

    // Skip through
    parts.next();

    let to = read_coordinates(parts.next().unwrap());

    Command { action, from, to }
}

fn perform_command_lit(command: &Command, grid: &mut [[bool; 1000]; 1000]) {
    let (from_x, from_y) = command.from;
    let (to_x, to_y) = command.to;

    for row in &mut grid[from_x as usize..=to_x as usize] {
        for cell in &mut row[from_y as usize..=to_y as usize] {
            *cell = match command.action {
                Action::Off => false,
                Action::On => true,
                Action::Toggle => !*cell,
            };
        }
    }
}

fn perform_command_brightness(command: &Command, grid: &mut [[i32; 1000]; 1000]) {
    let (from_x, from_y) = command.from;
    let (to_x, to_y) = command.to;

    for row in &mut grid[from_x as usize..=to_x as usize] {
        for cell in &mut row[from_y as usize..=to_y as usize] {
            *cell = match command.action {
                Action::Off => max(*cell - 1, 0),
                Action::On => *cell + 1,
                Action::Toggle => *cell + 2,
            };
        }
    }
}

fn count_lights(grid: &[[bool; 1000]; 1000]) -> usize {
    grid.iter().flatten().filter(|&&cell| cell).count()
}

fn count_brightness(grid: &[[i32; 1000]; 1000]) -> i32 {
    grid.iter().flatten().sum()
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d06.txt");

    let lines = contents.lines();

    let mut grid = [[false; 1000]; 1000];

    for line in lines {
        let command = string_to_command(line);
        perform_command_lit(&command, &mut grid);
    }

    let lit_lights = count_lights(&grid);

    println!("Amount of lit lights: {lit_lights}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d06.txt");

    let lines = contents.lines();

    let mut grid = [[0; 1000]; 1000];

    for line in lines {
        let command = string_to_command(line);
        perform_command_brightness(&command, &mut grid);
    }

    let total_brightness = count_brightness(&grid);

    println!("Total brightness: {total_brightness}");
}
