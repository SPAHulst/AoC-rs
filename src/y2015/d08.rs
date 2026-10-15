use crate::y2015::io::get_contents;

#[allow(clippy::collapsible_match)]
fn in_memory_length(s: &str) -> usize {
    let mut chars = s.chars();

    // Skip opening "
    chars.next();

    let mut length = 0;

    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => {
                if chars.next().unwrap() == 'x' {
                    chars.next();
                    chars.next();
                }
            }
            _ => {}
        }
        length += 1;
    }

    length
}

fn encoded_length(s: &str) -> usize {
    2 + s
        .chars()
        .map(|c| match c {
            '"' | '\\' => 2,
            _ => 1,
        })
        .sum::<usize>()
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d08.txt");

    let total: usize = contents
        .lines()
        .map(|line| line.len() - in_memory_length(line))
        .sum();

    println!("{total}");
}
pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d08.txt");

    let total: usize = contents
        .lines()
        .map(|line| encoded_length(line) - line.len())
        .sum();

    println!("{total}");
}
