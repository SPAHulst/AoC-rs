use crate::y2015::io::get_contents;

type HappinessPairs<'a> = Vec<(&'a str, &'a str, i16)>;
type HappinessMatrix = Vec<Vec<i16>>;

fn parse_happiness_scores<'a>(s: &'a str) -> (usize, HappinessPairs<'a>) {
    let lines = s.lines();

    let mut happiness_pairs = Vec::new();
    let mut line_count = 0;

    for line in lines {
        let mut parts = line.split_whitespace();

        let guest = parts.next().unwrap();
        parts.next(); // skip would
        let sign: i16 = match parts.next().unwrap() {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        };

        let score: i16 = parts.next().unwrap().parse().unwrap();
        let next_to = parts.nth(6).unwrap(); // skip to name

        happiness_pairs.push((guest, next_to, sign * score));
        line_count += 1;
    }

    let guest_count = ((1.0 + (1.0 + 4.0 * line_count as f32).sqrt()) / 2.0) as usize;

    (guest_count, happiness_pairs)
}

fn fill_happiness_matrix(guest_count: usize, happiness_pairs: &HappinessPairs) -> HappinessMatrix {
    let mut happinesses = vec![vec![0i16; guest_count]; guest_count];

    let mut row: usize = 0;
    let mut col: usize = 1;

    for happiness_pair in happiness_pairs {
        if col == row {
            col += 1;
        }

        let (_, _, score) = happiness_pair;

        happinesses[row][col] = *score;

        col += 1;
        if col == guest_count {
            row += 1;
            col = 0;
        }
    }

    happinesses
}

fn fill_happiness_matrix_include_self(
    guest_count: usize,
    happiness_pairs: &HappinessPairs,
) -> HappinessMatrix {
    let mut happinesses = vec![vec![0i16; guest_count + 1]; guest_count + 1];

    let mut row: usize = 0;
    let mut col: usize = 1;

    for happiness_pair in happiness_pairs {
        if col == row {
            col += 1;
        }

        let (_, _, score) = happiness_pair;

        happinesses[row][col] = *score;

        col += 1;
        if col == guest_count {
            happinesses[row][col] = 0;
            row += 1;
            col = 0;
        }
    }

    happinesses
}

fn compute_arrangement_happiness(arrangement: &[usize], happiness_matrix: &HappinessMatrix) -> i16 {
    let mut happiness = 0;
    let guest_count = arrangement.len();

    for i in 0..guest_count {
        let guest = arrangement[i];
        let left = arrangement[(i + guest_count - 1) % guest_count];
        let right = arrangement[(i + 1) % guest_count];

        happiness += happiness_matrix[guest][left];
        happiness += happiness_matrix[guest][right];
    }

    happiness
}

fn find_max_happiness(
    arrangement: &mut Vec<usize>,
    remaining: &mut Vec<usize>,
    happiness_matrix: &HappinessMatrix,
    max_happiness: &mut i16,
) {
    if remaining.is_empty() {
        let happiness = compute_arrangement_happiness(arrangement, happiness_matrix);
        *max_happiness = (*max_happiness).max(happiness);
        return;
    }

    for i in 0..remaining.len() {
        let guest = remaining.remove(i);
        arrangement.push(guest);

        find_max_happiness(arrangement, remaining, happiness_matrix, max_happiness);

        arrangement.pop();
        remaining.insert(i, guest);
    }
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d13.txt");
    let (guest_count, happiness_pairs) = parse_happiness_scores(&contents);
    let happiness_matrix = fill_happiness_matrix(guest_count, &happiness_pairs);

    let mut arrangement = Vec::new();
    let mut remaining = (0..guest_count).collect();
    let mut max_happiness = i16::MIN;

    find_max_happiness(
        &mut arrangement,
        &mut remaining,
        &happiness_matrix,
        &mut max_happiness,
    );

    println!("Happiness achieved in optimal happiness arrangement: {max_happiness}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d13.txt");
    let (guest_count, happiness_pairs) = parse_happiness_scores(&contents);
    let happiness_matrix = fill_happiness_matrix_include_self(guest_count, &happiness_pairs);

    let mut arrangement = Vec::new();
    let mut remaining = (0..=guest_count).collect();
    let mut max_happiness = i16::MIN;

    find_max_happiness(
        &mut arrangement,
        &mut remaining,
        &happiness_matrix,
        &mut max_happiness,
    );

    println!("Happiness achieved in optimal happiness arrangement: {max_happiness}");
}
