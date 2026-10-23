mod y2015;

use std::env;
use std::process::exit;

type Solver = fn();

struct Puzzle {
    year: u16,
    day: u8,
    one_star: Solver,
    two_star: Solver,
}

macro_rules! puzzle {
    ($year:expr, $day:expr, $one:expr, $two:expr) => {
        Puzzle {
            year: $year,
            day: $day,
            one_star: $one,
            two_star: $two,
        }
    };
}

static PUZZLES: &[Puzzle] = &[
    puzzle!(2015, 1, y2015::d01::one_star, y2015::d01::two_star),
    puzzle!(2015, 2, y2015::d02::one_star, y2015::d02::two_star),
    puzzle!(2015, 3, y2015::d03::one_star, y2015::d03::two_star),
    puzzle!(2015, 5, y2015::d05::one_star, y2015::d05::two_star),
    puzzle!(2015, 6, y2015::d06::one_star, y2015::d06::two_star),
    puzzle!(2015, 7, y2015::d07::one_star, y2015::d07::two_star),
    puzzle!(2015, 8, y2015::d08::one_star, y2015::d08::two_star),
    puzzle!(2015, 9, y2015::d09::one_star, y2015::d09::two_star),
    puzzle!(2015, 10, y2015::d10::one_star, y2015::d10::two_star),
    puzzle!(2015, 11, y2015::d11::one_star, y2015::d11::two_star),
    puzzle!(2015, 13, y2015::d13::one_star, y2015::d13::two_star),
    puzzle!(2015, 14, y2015::d14::one_star, y2015::d14::two_star),
    puzzle!(2015, 15, y2015::d15::one_star, y2015::d15::two_star),
    puzzle!(2015, 16, y2015::d16::one_star, y2015::d16::two_star),
    puzzle!(2015, 17, y2015::d17::one_star, y2015::d17::two_star),
    puzzle!(2015, 18, y2015::d18::one_star, y2015::d18::two_star),
    puzzle!(2015, 19, y2015::d19::one_star, y2015::d19::two_star),
    puzzle!(2015, 20, y2015::d20::one_star, y2015::d20::two_star),
    puzzle!(2015, 21, y2015::d21::one_star, y2015::d21::two_star),
];

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let [year, day, star] = args.as_slice() else {
        eprintln!("usage: cargo run -- <year> <day> <star>");
        exit(1);
    };

    let year: u16 = year.parse().unwrap_or_else(|_| {
        eprintln!("Invalid year: {year}");
        exit(1);
    });
    let day: u8 = day.parse().unwrap_or_else(|_| {
        eprintln!("Invalid day: {day}");
        exit(1);
    });
    let star: u8 = star.parse().unwrap_or_else(|_| {
        eprintln!("Invalid star: {star}");
        exit(1);
    });

    let Some(puzzle) = PUZZLES.iter().find(|p| p.year == year && p.day == day) else {
        eprintln!("No solution for {year} day {day}");
        exit(1);
    };

    match star {
        1 => (puzzle.one_star)(),
        2 => (puzzle.two_star)(),
        _ => {
            eprintln!("Invalid star: {star}");
            exit(1);
        }
    }
}
