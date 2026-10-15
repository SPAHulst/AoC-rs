use crate::y2015::io::get_contents;

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d01.txt");

    let mut floor = 0;

    for character in contents.chars() {
        if character == '(' {
            floor += 1;
        } else if character == ')' {
            floor -= 1;
        }
    }

    println!("Final floor {floor}")
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d01.txt");

    let mut floor = 0;

    let position = contents.chars().enumerate().find(|(_, character)| {
        if *character == '(' {
            floor += 1;
        } else if *character == ')' {
            floor -= 1;
        }

        floor < 0
    });

    let position = 1 + position.unwrap().0;

    println!("First entered the basement at position {position}")
}
