fn compute_lowest_house_infinite_houses(goal: u32) -> u32 {
    let mut presents = 0;
    let mut lowest_house = 0;

    // Since house i gets at least 10 * i, we have an upper bound of goal / 10
    let upperbound = goal / 10;
    for i in 1..=upperbound {
        if i > 1 {
            presents += 1;
        }

        if i >= 4 {
            // For any number N >= 4, we only need to check up to sqrt(N) to compute all divisors
            let ldp = (i as f32).sqrt() as u32;

            for divisor in 2..=ldp {
                if i % divisor == 0 {
                    presents += divisor;
                    if divisor != i / divisor {
                        presents += i / divisor;
                    }
                }
            }
        }

        presents += i;
        presents *= 10;

        if presents >= goal {
            lowest_house = i;
            break;
        }

        presents = 0;
    }

    lowest_house
}

fn done_visiting(elf: u32, house: u32) -> bool {
    house > 50 * elf
}

fn compute_lowest_house_finite_houses(goal: u32) -> u32 {
    let mut presents = 0;
    let mut lowest_house = 0;

    // Since house i gets at least 11 * i, we have an upper bound of goal / 11
    let upperbound = goal / 11;
    for i in 1..=upperbound {
        if i > 1 && !done_visiting(1, i) {
            presents += 1;
        }

        if i >= 4 {
            // For any number N >= 4, we only need to check up to sqrt(N) to compute all divisors
            let ldp = (i as f32).sqrt() as u32;

            for divisor in 2..=ldp {
                if i % divisor == 0 {
                    if !done_visiting(divisor, i) {
                        presents += divisor;
                    }

                    let other = i / divisor;
                    if other != divisor && !done_visiting(other, i) {
                        presents += other;
                    }
                }
            }
        }

        presents += i;
        presents *= 11;

        if presents >= goal {
            lowest_house = i;
            break;
        }

        presents = 0;
    }

    lowest_house
}

pub fn one_star() {
    let goal = 34000000;
    let lowest_house = compute_lowest_house_infinite_houses(goal);

    println!("Lowest house number exceeding {goal} presents is {lowest_house}");
}

pub fn two_star() {
    let goal = 34000000;
    let lowest_house = compute_lowest_house_finite_houses(goal);

    println!("Lowest house number exceeding {goal} presents is {lowest_house}");
}
