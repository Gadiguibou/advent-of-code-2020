use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let mut acc = 0i128;
    let instructions = include_str!("../../input.txt")
        .lines()
        .map(parse_instruction)
        .collect::<Result<Vec<_>,_>>()
        .expect("Parsing instructions failed.");

    let mut visited = vec![false; instructions.len()];

    let mut current_instruction = 0;

    let result = loop {
        if visited[current_instruction] {
            break acc
        } else {
            visited[current_instruction] = true;
        }

        match instructions[current_instruction] {
            Instruction::ACC(n) => {
                acc += n;
                current_instruction += 1;
            },
            Instruction::JMP(n) => {
                current_instruction = (current_instruction as i128 + n) as usize;
            },
            Instruction::NOP => current_instruction += 1
        }
    };

    println!("{}", result);

    Ok(())
}

enum Instruction {
    ACC(i128),
    JMP(i128),
    NOP,
}

fn parse_instruction(line: &str) -> anyhow::Result<Instruction> {
    let mut tokens = line.split(' ');

    match tokens.next() {
        Some("acc") => Ok(Instruction::ACC(
            match tokens.next() {
                Some(token) => token.parse()?,
                None => return Err(anyhow!("This failed")),
            }
        )),
        Some("jmp") => Ok(Instruction::JMP(
            match tokens.next() {
                Some(token) => token.parse()?,
                None => return Err(anyhow!("This failed")),
            }
        )),
        Some("nop") => Ok(Instruction::NOP),
        _ => Err(anyhow!("Invalid instruction.")),
    }

}