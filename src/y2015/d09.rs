use crate::y2015::io::get_contents;

type DistancePairs<'a> = Vec<(&'a str, &'a str, u16)>;
type DistanceMatrix = Vec<Vec<u16>>;

fn parse_distance_pairs<'a>(s: &'a str) -> (usize, DistancePairs<'a>) {
    let lines = s.lines();

    let mut distance_pairs = Vec::new();
    let mut line_count: u8 = 0;

    for line in lines {
        let mut parts = line.split_whitespace();

        let from = parts.next().unwrap();
        parts.next(); // skip to
        let to = parts.next().unwrap();
        parts.next(); // skip =
        let distance: u16 = parts.next().unwrap().parse().unwrap();

        distance_pairs.push((from, to, distance));
        line_count += 1;
    }

    // Problem concerns a complete graph, so we can compute the amount of city from the line count
    // If there are C cities, then the amount of lines is equal to (C-1) + ... + 1 = C(C-1)/2
    // If there are L lines, then the amount of cities is equal to (1 + (sqrt(1 + 8 * L))) / 2
    let city_count = ((1.0 + (1.0 + 8.0 * line_count as f32).sqrt()) / 2.0) as usize;

    (city_count, distance_pairs)
}

fn fill_distance_matrix(city_count: usize, distance_pairs: &DistancePairs) -> DistanceMatrix {
    let mut distances = vec![vec![0u16; city_count]; city_count];

    let mut row: usize = 0;
    let mut col: usize = 1;

    for distance_pair in distance_pairs {
        let (_, _, distance) = distance_pair;

        distances[row][col] = *distance;
        distances[col][row] = *distance;

        col += 1;
        // Due to symmetry, we only need to go over the upper triangle part of the matrix to fill it
        if col == city_count {
            row += 1;
            col = row + 1;
        }
    }

    distances
}

fn compute_route_distance(route: &[usize], distance_matrix: &DistanceMatrix) -> u16 {
    route
        .windows(2)
        .map(|pair| distance_matrix[pair[0]][pair[1]])
        .sum()
}

fn find_min_routes(
    route: &mut Vec<usize>,
    remaining: &mut Vec<usize>,
    distance_matrix: &DistanceMatrix,
    shortest_distance: &mut u16,
) {
    if remaining.is_empty() {
        let distance = compute_route_distance(route, distance_matrix);
        *shortest_distance = (*shortest_distance).min(distance);
        return;
    }

    for i in 0..remaining.len() {
        let city = remaining.remove(i);
        route.push(city);

        find_min_routes(route, remaining, distance_matrix, shortest_distance);

        route.pop();
        remaining.insert(i, city);
    }
}

fn find_max_routes(
    route: &mut Vec<usize>,
    remaining: &mut Vec<usize>,
    distance_matrix: &DistanceMatrix,
    longest_distance: &mut u16,
) {
    if remaining.is_empty() {
        let distance = compute_route_distance(route, distance_matrix);
        *longest_distance = (*longest_distance).max(distance);
        return;
    }

    for i in 0..remaining.len() {
        let city = remaining.remove(i);
        route.push(city);

        find_max_routes(route, remaining, distance_matrix, longest_distance);

        route.pop();
        remaining.insert(i, city);
    }
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d09.txt");
    let (city_count, distance_pairs) = parse_distance_pairs(&contents);
    let distances = fill_distance_matrix(city_count, &distance_pairs);

    let mut route = Vec::new();
    let mut remaining = (0..city_count).collect();
    let mut shortest_distance = u16::MAX;

    find_min_routes(
        &mut route,
        &mut remaining,
        &distances,
        &mut shortest_distance,
    );

    println!("Shortest route has a length of {shortest_distance}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d09.txt");
    let (city_count, distance_pairs) = parse_distance_pairs(&contents);
    let distances = fill_distance_matrix(city_count, &distance_pairs);

    let mut route = Vec::new();
    let mut remaining = (0..city_count).collect();
    let mut longest_distance = 0;

    find_max_routes(
        &mut route,
        &mut remaining,
        &distances,
        &mut longest_distance,
    );

    println!("Longest route has a length of {longest_distance}");
}
