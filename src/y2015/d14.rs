use crate::y2015::io::get_contents;
use std::cmp::max;

enum State {
    Flying,
    Resting,
}

#[derive(Clone, Copy)]
struct Reindeer {
    speed: u16,
    fly_period: u16,
    rest_period: u16,
}

struct PointRacer {
    reindeer: Reindeer,
    state: State,
    time_flown: u16,
    time_rested: u16,
    distance: u16,
    points: u16,
}

fn parse_racers(s: &str) -> Vec<Reindeer> {
    let lines = s.lines();

    let mut racers = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();

        let speed = parts.nth(3).unwrap().parse().unwrap();
        let fly_period = parts.nth(2).unwrap().parse().unwrap();
        let rest_period = parts.nth(6).unwrap().parse().unwrap();

        let racer = Reindeer {
            speed,
            fly_period,
            rest_period,
        };

        racers.push(racer);
    }

    racers
}

fn race(racer: &Reindeer, end_time: u16) -> u16 {
    let mut distance: u16 = 0;
    let mut time: u16 = 0;
    let mut time_flown: u16 = 0;
    let mut time_rested: u16 = 0;

    let mut state = State::Flying;

    while time < end_time {
        match state {
            State::Flying => {
                distance += racer.speed;
                time_flown += 1;

                if time_flown == racer.fly_period {
                    state = State::Resting;
                    time_flown = 0;
                }
            }
            State::Resting => {
                time_rested += 1;

                if time_rested == racer.rest_period {
                    state = State::Flying;
                    time_rested = 0;
                }
            }
        }

        time += 1;
    }

    distance
}

fn point_race(reindeer: &[Reindeer], end_time: u16) -> u16 {
    let mut point_racers: Vec<PointRacer> = reindeer
        .iter()
        .copied()
        .map(|racer| PointRacer {
            reindeer: racer,
            state: State::Flying,
            time_flown: 0,
            time_rested: 0,
            distance: 0,
            points: 0,
        })
        .collect();

    let mut time: u16 = 0;

    while time < end_time {
        for racer in point_racers.iter_mut() {
            match racer.state {
                State::Flying => {
                    racer.distance += racer.reindeer.speed;
                    racer.time_flown += 1;

                    if racer.time_flown == racer.reindeer.fly_period {
                        racer.state = State::Resting;
                        racer.time_flown = 0;
                    }
                }
                State::Resting => {
                    racer.time_rested += 1;

                    if racer.time_rested == racer.reindeer.rest_period {
                        racer.state = State::Flying;
                        racer.time_rested = 0;
                    }
                }
            }
        }

        let max_distance = point_racers
            .iter()
            .map(|racer| racer.distance)
            .max()
            .unwrap();

        for racer in point_racers.iter_mut() {
            if racer.distance == max_distance {
                racer.points += 1;
            }
        }

        time += 1;
    }

    point_racers.iter().map(|racer| racer.points).max().unwrap()
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d14.txt");
    let racers = parse_racers(&contents);
    let end_time: u16 = 2503;

    let mut furthest_distance = 0;

    for racer in &racers {
        let distance = race(racer, end_time);
        furthest_distance = max(distance, furthest_distance);
    }

    println!("Distance traveled by winning reindeer: {furthest_distance}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d14.txt");
    let racers = parse_racers(&contents);
    let end_time: u16 = 2503;

    let winning_score = point_race(&racers, end_time);

    println!("Score of the winning reindeer: {winning_score}");
}
