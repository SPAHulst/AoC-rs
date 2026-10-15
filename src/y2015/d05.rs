use crate::y2015::io::get_contents;

fn contains_three_vowels(s: &str) -> bool {
    let mut vowel_counter: u8 = 0;

    for character in s.chars() {
        if "aeiou".contains(character) {
            vowel_counter += 1;

            if vowel_counter >= 3 {
                return true;
            }
        }
    }

    false
}

fn contains_double_letter(s: &str) -> bool {
    let characters = s.chars();
    let next_char = s.chars().skip(1);
    let matches = characters
        .zip(next_char)
        .find(|(char1, char2)| char1 == char2);

    match matches {
        Some((_, _)) => true,
        None => false,
    }
}

fn contains_forbidden_sequence(s: &str) -> bool {
    let forbidden_sequences = ["ab", "cd", "pq", "xy"];

    for sequence in forbidden_sequences {
        if s.contains(sequence) {
            return true;
        }
    }

    false
}

fn contains_double_pair(s: &str) -> bool {
    let characters = s.chars();
    let next_char = s.chars().skip(1);

    let pairs: Vec<_> = characters.zip(next_char).collect();

    for (i, pair) in pairs.iter().enumerate() {
        for subsequent_pair in pairs.iter().skip(i + 2) {
            if pair == subsequent_pair {
                return true;
            }
        }
    }

    false
}

fn contains_letter_sandwich(s: &str) -> bool {
    let characters = s.chars();
    let sandwich_char = s.chars().skip(2);

    let matches = characters
        .zip(sandwich_char)
        .find(|(char1, char2)| char1 == char2);

    match matches {
        Some((_, _)) => true,
        None => false,
    }
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d05.txt");

    let strings = contents.lines();

    let mut nice_string_counter: u32 = 0;

    for string in strings {
        if contains_three_vowels(string)
            && contains_double_letter(string)
            && !contains_forbidden_sequence(string)
        {
            nice_string_counter += 1;
        }
    }

    println!("Amount of nice strings: {nice_string_counter}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d05.txt");

    let strings = contents.lines();

    let mut nice_string_counter: u32 = 0;

    for string in strings {
        if contains_double_pair(string) && contains_letter_sandwich(string) {
            nice_string_counter += 1;
        }
    }

    println!("Amount of nice strings: {nice_string_counter}");
}
