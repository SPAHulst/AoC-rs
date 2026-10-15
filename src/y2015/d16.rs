use crate::y2015::io::get_contents;

enum FactType {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

struct Fact {
    description: FactType,
    amount: u8,
}

fn parse_sue_facts(s: &str) -> Vec<Vec<Fact>> {
    let lines = s.lines();

    let mut sue_facts = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();

        parts.nth(1); // Skip Sue N:

        let mut facts = Vec::new();

        while let Some(description) = parts.next() {
            let description = description.trim_end_matches(':');
            let description = match description {
                "children" => FactType::Children,
                "cats" => FactType::Cats,
                "samoyeds" => FactType::Samoyeds,
                "pomeranians" => FactType::Pomeranians,
                "akitas" => FactType::Akitas,
                "vizslas" => FactType::Vizslas,
                "goldfish" => FactType::Goldfish,
                "trees" => FactType::Trees,
                "cars" => FactType::Cars,
                "perfumes" => FactType::Perfumes,
                _ => unreachable!(),
            };

            let amount = parts.next().unwrap().trim_end_matches(',').parse().unwrap();

            let sue_fact = Fact {
                description,
                amount,
            };

            facts.push(sue_fact);
        }

        sue_facts.push(facts);
    }

    sue_facts
}

fn expected_amount(fact: &FactType) -> u8 {
    match fact {
        FactType::Children => 3,
        FactType::Cats => 7,
        FactType::Samoyeds => 2,
        FactType::Pomeranians => 3,
        FactType::Akitas => 0,
        FactType::Vizslas => 0,
        FactType::Goldfish => 5,
        FactType::Trees => 3,
        FactType::Cars => 2,
        FactType::Perfumes => 1,
    }
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d16.txt");
    let aunt_sues = parse_sue_facts(&contents);

    for (index, aunt_sue) in aunt_sues.iter().enumerate() {
        let all_current_facts_agree = aunt_sue.iter().all(|fact| {
            let expected = expected_amount(&fact.description);
            fact.amount == expected
        });

        if all_current_facts_agree {
            let sue_index = index + 1;
            println!("Sue {sue_index} is THE aunt Sue");
        }
    }
}

fn fact_matches(fact: &Fact, expected: u8) -> bool {
    match fact.description {
        FactType::Cats | FactType::Trees => fact.amount > expected,
        FactType::Pomeranians | FactType::Goldfish => fact.amount < expected,
        _ => fact.amount == expected,
    }
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d16.txt");
    let aunt_sues = parse_sue_facts(&contents);

    for (index, aunt_sue) in aunt_sues.iter().enumerate() {
        let all_current_facts_agree = aunt_sue.iter().all(|fact| {
            let expected = expected_amount(&fact.description);
            fact_matches(fact, expected)
        });

        if all_current_facts_agree {
            let sue_index = index + 1;
            println!("Sue {sue_index} is THE aunt Sue");
        }
    }
}
