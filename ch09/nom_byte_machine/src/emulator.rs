use crate::instructions::{get_process_size, parse_instruction, Instruction};

fn input_line(buffer: &mut [u8]) {
    let mut text = String::new();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    for i in 0..text.len().min(buffer.len()) {
        buffer[i] = text.as_bytes()[i];
    }
    for i in text.len()..buffer.len() {
        buffer[i] = 0;
    }
}

pub struct RegisterSet {
    ip: u16,
    acc: u16,
}

fn get_le_word(slice: &[u8], address: u16) -> u16 {
    u16::from(slice[address as usize]) + (u16::from(slice[address as usize + 1]) << 8)
}

fn set_le_word(slice: &mut [u8], address: u16, value: u16) {
    slice[address as usize] = value as u8;
    slice[address as usize + 1] = (value >> 8) as u8;
}

fn get_byte(slice: &[u8], address: u16) -> u16 {
    u16::from(slice[address as usize])
}

fn set_byte(slice: &mut [u8], address: u16, value: u16) {
    slice[address as usize] = value as u8;
}

pub fn execute_instruction(
    process: &mut [u8],
    r: &mut RegisterSet,
    instruction: Instruction,
) -> Option<u8> {
    use Instruction::*;
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
            r.acc = get_le_word(process, address);
            r.ip += 3;
        }
        Store(address) => {
            set_le_word(process, address, r.acc);
            r.ip += 3;
        }
        IndirectLoad(address) => {
            r.acc = get_le_word(process, get_le_word(process, address));
            r.ip += 3;
        }
        IndirectStore(address) => {
            set_le_word(process, get_le_word(process, address), r.acc);
            r.ip += 3;
        }
        Input(length) => {
            let address = r.acc as usize;
            input_line(&mut process[address..address + length as usize]);
            r.ip += 2;
        }
        Output(length) => {
            let address = r.acc as usize;
            for &byte in &process[address..address + length as usize] {
                print!("{}", if byte == 0 { ' ' } else { byte as char });
            }
            r.ip += 2;
        }
        Add(address) => {
            r.acc = r.acc.wrapping_add(get_le_word(process, address));
            r.ip += 3;
        }
        Subtract(address) => {
            r.acc = r.acc.wrapping_sub(get_le_word(process, address));
            r.ip += 3;
        }
        Multiply(address) => {
            r.acc = r.acc.wrapping_mul(get_le_word(process, address));
            r.ip += 3;
        }
        Divide(address) => {
            r.acc = r.acc.wrapping_div(get_le_word(process, address));
            r.ip += 3;
        }
        Remainder(address) => {
            r.acc = r.acc.wrapping_rem(get_le_word(process, address));
            r.ip += 3;
        }
        Jump(address) => {
            r.ip = address;
        }
        JumpIfZero(address) => {
            if r.acc == 0 {
                r.ip = address;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNonZero(address) => {
            if r.acc != 0 {
                r.ip = address;
            } else {
                r.ip += 3;
            }
        }
        JumpIfPositive(address) => {
            if (r.acc as i16) > 0 {
                r.ip = address;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNegative(address) => {
            if (r.acc as i16) < 0 {
                r.ip = address;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNonPositive(address) => {
            if r.acc as i16 <= 0 {
                r.ip = address;
            } else {
                r.ip += 3;
            }
        }
        JumpIfNonNegative(address) => {
            if r.acc as i16 >= 0 {
                r.ip = address;
            } else {
                r.ip += 3;
            }
        }
        LoadByte(address) => {
            r.acc = get_byte(process, address);
            r.ip += 3;
        }
        StoreByte(address) => {
            set_byte(process, address, r.acc);
            r.ip += 3;
        }
        IndirectLoadByte(address) => {
            r.acc = get_byte(process, get_le_word(process, address));
            r.ip += 3;
        }
        IndirectStoreByte(address) => {
            set_byte(process, get_le_word(process, address), r.acc);
            r.ip += 3;
        }
        Byte(_) => {
            r.ip += 1;
        }
    }
    None
}

pub fn execute_program(program: &[u8]) -> Result<u8, ()> {
    let process_size_parsed: u16 = match get_process_size(program) {
        Ok(ok) => ok,
        Err(_) => return Err(()),
    };

    let mut process = vec![0u8; process_size_parsed as usize];
    process[0..program.len()].copy_from_slice(&program);

    let mut registers = RegisterSet { ip: 2, acc: 0 };
    loop {
        let instruction = match parse_instruction(&process[registers.ip as usize..]) {
            Ok(instruction) => instruction.1,
            Err(_) => return Err(()),
        };
        //println!(
        //    "Ip: {} Acc: {} Instr: {:?}",
        //    registers.ip, registers.acc, instruction
        //);
        if let Some(return_code) = execute_instruction(&mut process, &mut registers, instruction) {
            return Ok(return_code);
        }
    }
}
