use std::collections::{HashMap, HashSet};

use crate::y2015::io::get_contents;

fn parse_calibration(s: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut replacement_map = HashMap::new();
    let mut starting_point = "";

    for line in s.lines().filter(|line| !line.is_empty()) {
        let mut parts = line.split_whitespace();
        let element = parts.next().unwrap();

        match parts.next() {
            Some(_) => {
                let replacement = parts.next().unwrap();

                replacement_map
                    .entry(element)
                    .or_insert_with(Vec::new)
                    .push(replacement);
            }
            None => starting_point = element,
        }
    }

    (replacement_map, starting_point)
}

fn compute_distinct_molecules(start: &str, replacement_map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut distinct_molecules = HashSet::new();

    for (molecule, replacements) in replacement_map {
        for (index, _) in start.match_indices(molecule) {
            // Due to single byte characters, indexing is possible
            let head = &start[..index];
            let tail = &start[index + molecule.len()..];

            for replacement in replacements {
                let mut new_molecule =
                    String::with_capacity(head.len() + replacement.len() + tail.len());
                new_molecule.push_str(head);
                new_molecule.push_str(replacement);
                new_molecule.push_str(tail);

                distinct_molecules.insert(new_molecule);
            }
        }
    }

    distinct_molecules.len()
}

fn fewest_steps_to_medicine(medicine: &str) -> usize {
    let elements = medicine.bytes().filter(u8::is_ascii_uppercase).count();
    let rn = medicine.matches("Rn").count();
    let ar = medicine.matches("Ar").count();
    let y = medicine.matches('Y').count();

    elements - rn - ar - 2 * y - 1
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d19.txt");
    let (replacement_map, starting_point) = parse_calibration(&contents);

    let distinct_molecules = compute_distinct_molecules(starting_point, &replacement_map);

    println!("Amount of distinct molecules: {distinct_molecules}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d19.txt");
    let (_, medicine) = parse_calibration(&contents);

    let min_steps = fewest_steps_to_medicine(medicine);

    println!("Fewest steps to medicine molecule: {min_steps}");
}
