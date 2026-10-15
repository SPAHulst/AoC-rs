fn transform_sequence(s: &str) -> String {
    let mut next_sequence = String::new();
    let mut characters = s.chars();

    let mut previous_char = characters.next().unwrap();
    let mut counter: u16 = 1;

    for character in characters {
        if character == previous_char {
            counter += 1;
            continue;
        }

        next_sequence.push_str(&counter.to_string());
        next_sequence.push(previous_char);

        previous_char = character;
        counter = 1;
    }

    next_sequence.push_str(&counter.to_string());
    next_sequence.push(previous_char);

    next_sequence
}

fn look_and_say(start: &str, turns: u8) -> usize {
    let mut current_sequence = start.to_string();
    for _ in 0..turns {
        current_sequence = transform_sequence(&current_sequence);
    }
    current_sequence.len()
}

pub fn one_star() {
    let start = "1321131112";
    let turns = 40;
    let length = look_and_say(start, turns);
    println!(
        "After playing look-and-say {turns} times, the length of the final string is {length}"
    );
}

pub fn two_star() {
    let start = "1321131112";
    let turns = 50;
    let length = look_and_say(start, turns);
    println!(
        "After playing look-and-say {turns} times, the length of the final string is {length}"
    );
}
