fn increment_password(password: &str) -> String {
    let mut password_bytes = password.as_bytes().to_vec();

    let mut i = password.len() - 1;
    loop {
        if password_bytes[i] == b'z' {
            password_bytes[i] = b'a';
            i -= 1;
        } else {
            password_bytes[i] += 1;
            break;
        }
    }

    String::from_utf8(password_bytes).unwrap()
}

fn contains_increasing_straight(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes
        .windows(3)
        .any(|window| window[1] == window[0] + 1 && window[2] == window[1] + 1)
}

fn contains_forbidden_letters(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.iter().any(|&c| matches!(c, b'i' | b'o' | b'l'))
}

fn contains_unique_pairs(s: &str) -> bool {
    let pairs = s.as_bytes().windows(2);
    let mut first_pair = None;

    for pair in pairs {
        if pair[0] != pair[1] {
            continue;
        }

        match first_pair {
            Some(first) if first != pair[0] => return true,
            None => first_pair = Some(pair[0]),
            _ => {}
        }
    }

    false
}

pub fn one_star() {
    let mut password = String::from("cqjxjnds");

    while contains_forbidden_letters(&password)
        || !contains_increasing_straight(&password)
        || !contains_unique_pairs(&password)
        || password == "cqjxjnds"
    {
        password = increment_password(&password);
    }

    println!("New password: {password}");
}

pub fn two_star() {
    let mut password = String::from("cqjxxyzz");

    while contains_forbidden_letters(&password)
        || !contains_increasing_straight(&password)
        || !contains_unique_pairs(&password)
        || password == "cqjxxyzz"
    {
        password = increment_password(&password);
    }

    println!("New password: {password}");
}
