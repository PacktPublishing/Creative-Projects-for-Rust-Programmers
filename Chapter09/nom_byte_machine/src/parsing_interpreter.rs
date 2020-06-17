use crate::instructions::{get_process_size, parse_instruction, Instruction};

pub fn parse_program(program: &[u8]) -> Result<Vec<Instruction>, ()> {
    let process_size_parsed = match get_process_size(program) {
        Ok(ok) => ok as usize,
        Err(_) => return Err(()),
    };
    let mut parsed_program = vec![Instruction::Byte(0); process_size_parsed];
    let mut ip = 2;
    loop {
        match parse_instruction(&program[ip..]) {
            Ok(instruction) => {
                parsed_program[ip] = instruction.1;
                ip += instruction.1.len();
                if let Instruction::Terminate(_) = instruction.1 {
                    break;
                }
            }
            Err(_) => return Err(()),
        };
    }
    for ip in ip..program.len() {
        parsed_program[ip] = Instruction::Byte(program[ip]);
    }
    Ok(parsed_program)
}

struct ParsedRegisterSet {
    ip: usize,
    acc: u16,
}

pub fn execute_parsed_program(parsed_program: &mut Vec<Instruction>) -> u8 {
    let mut registers = ParsedRegisterSet { ip: 2, acc: 0 };
    loop {
        if let Some(return_code) = execute_parsed_instruction(parsed_program, &mut registers) {
            return return_code;
        };
    }
}

fn input_parsed_line(buffer: &mut [Instruction]) {
    let mut text = String::new();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    for i in 0..text.len().min(buffer.len()) {
        buffer[i] = Instruction::Byte(text.as_bytes()[i]);
    }
    for i in text.len()..buffer.len() {
        buffer[i] = Instruction::Byte(0);
    }
}

fn get_parsed_le_word(process: &[Instruction], address: u16) -> u16 {
    if let Instruction::Byte(byte0) = process[address as usize] {
        if let Instruction::Byte(byte1) = process[address as usize + 1] {
            return u16::from(byte0) + (u16::from(byte1) << 8);
        }
    }
    0
}

fn set_parsed_le_word(process: &mut [Instruction], address: u16, word: u16) {
    process[address as usize] = Instruction::Byte(word as u8);
    process[address as usize + 1] = Instruction::Byte((word >> 8) as u8);
}

fn get_parsed_byte(process: &[Instruction], address: u16) -> u8 {
    if let Instruction::Byte(byte) = process[address as usize] {
        byte
    } else {
        0
    }
}

fn set_parsed_byte(process: &mut [Instruction], address: u16, byte: u8) {
    process[address as usize] = Instruction::Byte(byte);
}

fn execute_parsed_instruction(
    process: &mut [Instruction],
    r: &mut ParsedRegisterSet,
) -> Option<u8> {
    use Instruction::*;
    let instruction = process[r.ip as usize];
    //println!("Ip: {} Acc: {} Instr: {:?}", r.ip, r.acc, instruction);
    match instruction {
        Terminate(operand) => {
            r.ip += 2;
            return Some(operand);
        }
        Set(operand) => {
            r.acc = operand;
            r.ip += 3;
        }
        Load(address) => {
            r.acc = get_parsed_le_word(process, address);
            r.ip += 3;
        }
        Store(address) => {
            set_parsed_le_word(process, address, r.acc);
            r.ip += 3;
        }
        IndirectLoad(address) => {
            r.acc = get_parsed_le_word(process, get_parsed_le_word(process, address));
            r.ip += 3;
        }
        IndirectStore(address) => {
            set_parsed_le_word(process, get_parsed_le_word(process, address), r.acc);
            r.ip += 3;
        }
        Input(length) => {
            let address = r.acc as usize;
            input_parsed_line(&mut process[address..address + length as usize]);
            r.ip += 2;
        }
        Output(length) => {
            let address = r.acc as usize;
            for &instruction in &process[address..address + length as usize] {
                if let Byte(byte) = instruction {
                    print!("{}", if byte == 0 { ' ' } else { byte as char });
                }
            }
            r.ip += 2;
        }
        Add(address) => {
            r.acc = r.acc.wrapping_add(get_parsed_le_word(process, address));
            r.ip += 3;
        }
        Subtract(address) => {
            r.acc = r.acc.wrapping_sub(get_parsed_le_word(process, address));
            r.ip += 3;
        }
        Multiply(address) => {
            r.acc = r.acc.wrapping_mul(get_parsed_le_word(process, address));
            r.ip += 3;
        }
        Divide(address) => {
            r.acc = r.acc.wrapping_div(get_parsed_le_word(process, address));
            r.ip += 3;
        }
        Remainder(address) => {
            r.acc = r.acc.wrapping_rem(get_parsed_le_word(process, address));
            r.ip += 3;
        }
        Jump(address) => {
            r.ip = address as usize;
        }
        JumpIfZero(address) => {
            if r.acc == 0 {
                r.ip = address as usize;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNonZero(address) => {
            if r.acc != 0 {
                r.ip = address as usize;
            } else {
                r.ip += 3;
            }
        }
        JumpIfPositive(address) => {
            if (r.acc as i16) > 0 {
                r.ip = address as usize;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNegative(address) => {
            if (r.acc as i16) < 0 {
                r.ip = address as usize;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNonPositive(address) => {
            if r.acc as i16 <= 0 {
                r.ip = address as usize;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNonNegative(address) => {
            if r.acc as i16 >= 0 {
                r.ip = address as usize;
            } else {
                r.ip += 3;
            }
        }
        LoadByte(address) => {
            r.acc = u16::from(get_parsed_byte(process, address));
            r.ip += 3;
        }
        StoreByte(address) => {
            set_parsed_byte(process, address, r.acc as u8);
            r.ip += 3;
        }
        IndirectLoadByte(address) => {
            r.acc = u16::from(get_parsed_byte(
                process,
                get_parsed_le_word(process, address),
            ));
            r.ip += 3;
        }
        IndirectStoreByte(address) => {
            set_parsed_byte(process, get_parsed_le_word(process, address), r.acc as u8);
            r.ip += 3;
        }
        Byte(_) => {
            r.ip += 1;
        }
    }
    None
}
