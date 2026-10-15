use crate::y2015::io::get_contents;

fn parse_containers(s: &str) -> Vec<u8> {
    s.lines().map(|num| num.parse::<u8>().unwrap()).collect()
}

fn count_combinations(containers: &[u8], target: usize) -> usize {
    let mut dp = vec![0usize; target + 1];
    dp[0] = 1;

    for &volume in containers {
        let volume = usize::from(volume);

        for j in (volume..=target).rev() {
            dp[j] += dp[j - volume];
        }
    }

    dp[target]
}

fn count_min_combinations(containers: &[u8], target: usize, container_count: usize) -> usize {
    let mut dp = vec![vec![0usize; target + 1]; container_count + 1];
    dp[0][0] = 1;

    for &volume in containers {
        let volume = usize::from(volume);

        for k in (1..container_count).rev() {
            for j in (volume..=target).rev() {
                dp[k][j] += dp[k - 1][j - volume];
            }
        }
    }

    let min_k = dp.iter().position(|row| row[target] > 0);
    match min_k {
        Some(k) => dp[k][150],
        None => 0,
    }
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d17.txt");
    let containers = parse_containers(&contents);

    let combinations = count_combinations(&containers, 150);

    println!("Amount of combinations for 150 liters of eggnog: {combinations}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d17.txt");
    let containers = parse_containers(&contents);

    let combinations = count_min_combinations(&containers, 150, containers.len());

    println!("Min of containers needed for 150 liters of eggnog: {combinations}");
}
