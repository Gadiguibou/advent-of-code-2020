use std::convert::TryInto;

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let original_instructions = include_str!("../../input.txt")
        .lines()
        .map(parse_instruction)
        .collect::<Result<Vec<_>, _>>()
        .expect("Parsing instructions failed.");

    for (index, instruction) in original_instructions.iter().enumerate() {
        if instruction.operator == Command::ACC {
            continue;
        }

        let mut instructions_copy = original_instructions.clone();

        if instruction.operator == Command::JMP {
            instructions_copy[index].operator = Command::NOP;
        } else {
            instructions_copy[index].operator = Command::JMP;
        }

        let (terminated, result) = test_instruction_set(instructions_copy);

        if terminated {
            println!("{}", result);
            return Ok(());
        }
    }

    Ok(())
}

fn test_instruction_set(instructions: Vec<Instruction>) -> (bool, i128) {
    let mut acc = 0i128;
    let mut visited = vec![false; instructions.len()];
    let mut current_instruction = 0;
    loop {
        if visited[current_instruction] {
            break (false, acc);
        } else {
            visited[current_instruction] = true;
        }

        let instruction = &instructions[current_instruction];

        match instruction.operator {
            Command::ACC => {
                acc += instruction.operand;
                current_instruction += 1;
            }
            Command::JMP => {
                current_instruction = (current_instruction as i128 + instruction.operand).try_into().unwrap();
            }
            Command::NOP => {
                current_instruction += 1;
            }
        }

        if current_instruction == instructions.len() {
            break (true, acc);
        } else if current_instruction > instructions.len() || current_instruction < 0 {
            break (false, acc);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
    ACC,
    JMP,
    NOP,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    operator: Command,
    operand: i128,
}

fn parse_instruction(line: &str) -> anyhow::Result<Instruction> {
    let mut tokens = line.split(' ');

    Ok(Instruction {
        operator: {
            match tokens.next() {
                Some("acc") => Command::ACC,
                Some("jmp") => Command::JMP,
                Some("nop") => Command::NOP,
                _ => return Err(anyhow!("Unknown instruction")),
            }
        },
        operand: {
            match tokens.next() {
                Some(token) => token.parse()?,
                None => return Err(anyhow!("Missing operand")),
            }
        },
    })
}
