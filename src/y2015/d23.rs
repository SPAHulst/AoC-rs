use crate::y2015::io::get_contents;

enum Register {
    A,
    B,
}

enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i8),
    Jie(Register, i8),
    Jio(Register, i8),
}

fn parse_register(s: &str) -> Register {
    match s.trim_end_matches(',') {
        "a" => Register::A,
        "b" => Register::B,
        _ => unreachable!(),
    }
}

fn parse_program(s: &str) -> Vec<Instruction> {
    let mut program = Vec::new();

    for line in s.lines() {
        let mut parts = line.split_whitespace();

        let instruction = match parts.next().unwrap() {
            "hlf" => Instruction::Hlf(parse_register(parts.next().unwrap())),
            "tpl" => Instruction::Tpl(parse_register(parts.next().unwrap())),
            "inc" => Instruction::Inc(parse_register(parts.next().unwrap())),
            "jmp" => Instruction::Jmp(parts.next().unwrap().parse().unwrap()),
            "jie" => {
                let register = parse_register(parts.next().unwrap());
                let offset = parts.next().unwrap().parse().unwrap();
                Instruction::Jie(register, offset)
            }
            "jio" => {
                let register = parse_register(parts.next().unwrap());
                let offset = parts.next().unwrap().parse().unwrap();
                Instruction::Jio(register, offset)
            }
            _ => unreachable!(),
        };

        program.push(instruction);
    }

    program
}

fn register_to_index(register: &Register) -> usize {
    match register {
        Register::A => 0,
        Register::B => 1,
    }
}

fn run_program(program: &[Instruction], part_two: bool) -> [u32; 2] {
    let mut computer = match part_two {
        true => [1, 0],
        false => [0, 0],
    };
    let mut instruction_counter: i8 = 0;

    loop {
        if instruction_counter < 0 || instruction_counter as usize >= program.len() {
            break;
        }

        match &program[instruction_counter as usize] {
            Instruction::Hlf(register) => {
                computer[register_to_index(register)] /= 2;
            }
            Instruction::Tpl(register) => {
                computer[register_to_index(register)] *= 3;
            }
            Instruction::Inc(register) => {
                computer[register_to_index(register)] += 1;
            }
            Instruction::Jmp(offset) => {
                instruction_counter += offset;
                continue;
            }
            Instruction::Jie(register, offset) => {
                if computer[register_to_index(register)] % 2 == 0 {
                    instruction_counter += offset;
                    continue;
                }
            }
            Instruction::Jio(register, offset) => {
                if computer[register_to_index(register)] == 1 {
                    instruction_counter += offset;
                    continue;
                }
            }
        }

        instruction_counter += 1;
    }

    computer
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d23.txt");
    let program = parse_program(&contents);

    let registers = run_program(&program, false);

    println!(
        "Value of registers \n A: {} \n B: {}",
        registers[0], registers[1]
    );
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d23.txt");
    let program = parse_program(&contents);

    let registers = run_program(&program, true);

    println!(
        "Value of registers \n A: {} \n B: {}",
        registers[0], registers[1]
    );
}
