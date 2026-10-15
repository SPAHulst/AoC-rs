use crate::y2015::io::get_contents;
use std::collections::HashMap;

enum Operand {
    Signal(u16),
    Wire(String),
}

enum Operation {
    Passthrough(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    LShift(Operand, u16),
    RShift(Operand, u16),
}

struct Instruction {
    operation: Operation,
    output: String,
}

fn parse_operand(s: &str) -> Operand {
    match s.parse::<u16>() {
        Ok(signal) => Operand::Signal(signal),
        Err(_) => Operand::Wire(s.to_string()),
    }
}

fn string_to_instruction(s: &str) -> Instruction {
    let mut parts = s.split_whitespace();

    let first = parts.next().unwrap();

    let operation = match first {
        "NOT" => Operation::Not(parse_operand(parts.next().unwrap())),
        operand => match parts.next().unwrap() {
            "->" => Operation::Passthrough(parse_operand(operand)),
            "AND" => Operation::And(parse_operand(operand), parse_operand(parts.next().unwrap())),
            "OR" => Operation::Or(parse_operand(operand), parse_operand(parts.next().unwrap())),
            "LSHIFT" => Operation::LShift(
                parse_operand(operand),
                parts.next().unwrap().parse().unwrap(),
            ),
            "RSHIFT" => Operation::RShift(
                parse_operand(operand),
                parts.next().unwrap().parse().unwrap(),
            ),
            _ => unreachable!(),
        },
    };

    // Skip -> if necessary
    let output = match operation {
        Operation::Passthrough(_) => parts.next().unwrap().to_string(),
        _ => {
            parts.next();
            parts.next().unwrap().to_string()
        }
    };

    Instruction { operation, output }
}

fn compute_operand(
    operand: &Operand,
    instructions: &HashMap<String, Operation>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    match operand {
        Operand::Signal(signal) => *signal,
        Operand::Wire(wire) => compute_signal(wire, instructions, cache),
    }
}

fn compute_signal(
    wire: &str,
    instructions: &HashMap<String, Operation>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(signal) = cache.get(wire) {
        return *signal;
    }

    let signal = match instructions.get(wire) {
        Some(operation) => match operation {
            Operation::Passthrough(operand) => compute_operand(operand, instructions, cache),
            Operation::And(left, right) => {
                compute_operand(left, instructions, cache)
                    & compute_operand(right, instructions, cache)
            }
            Operation::Or(left, right) => {
                compute_operand(left, instructions, cache)
                    | compute_operand(right, instructions, cache)
            }
            Operation::Not(operand) => !compute_operand(operand, instructions, cache),
            Operation::LShift(operand, shift) => {
                compute_operand(operand, instructions, cache) << shift
            }
            Operation::RShift(operand, shift) => {
                compute_operand(operand, instructions, cache) >> shift
            }
        },
        None => unreachable!(),
    };

    cache.insert(wire.to_string(), signal);

    signal
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d07.txt");

    let lines = contents.lines();

    let mut instructions: HashMap<String, Operation> = HashMap::new();

    for line in lines {
        let instruction = string_to_instruction(line);
        instructions.insert(instruction.output, instruction.operation);
    }

    let wire = "a";
    let mut cache: HashMap<String, u16> = HashMap::new();
    let signal = compute_signal(wire, &instructions, &mut cache);
    println!("Signal contained within {wire}: {signal}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d07.txt");

    let lines = contents.lines();

    let mut instructions: HashMap<String, Operation> = HashMap::new();

    for line in lines {
        let instruction = string_to_instruction(line);
        instructions.insert(instruction.output, instruction.operation);
    }

    let wire = "a";
    let mut cache: HashMap<String, u16> = HashMap::new();
    let signal_a = compute_signal(wire, &instructions, &mut cache);

    instructions.insert(
        "b".to_string(),
        Operation::Passthrough(Operand::Signal(signal_a)),
    );

    let mut cache = HashMap::new();
    let new_signal_a = compute_signal(wire, &instructions, &mut cache);
    println!("Signal contained within {wire}: {new_signal_a}");
}
