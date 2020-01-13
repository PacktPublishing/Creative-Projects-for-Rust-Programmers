use crate::instructions::{parse_instruction, Instruction};
use std::fs::File;
use std::io::{Error, ErrorKind, Result, Write};

pub fn translate_program_to_c(program: &[u8], target_path: &str) -> Result<()> {
    let mut file = File::create(target_path)?;

    match std::fs::write(target_path, "") {
        Ok(_) => eprintln!("Compiled to {}.", target_path),
        Err(err) => eprintln!("Failed to write to file {}: ({})", target_path, err),
    }

    let mut ip = 2;
    writeln!(file, "#include <stdio.h>")?;
    writeln!(file, "#include <string.h>")?;
    writeln!(file, "unsigned char memory[];")?;
    writeln!(
        file,
        "unsigned short bytes_to_u16_le(unsigned int address) {{"
    )?;
    writeln!(
        file,
        "    return (unsigned short)(memory[address] + (memory[address + 1] << 8));"
    )?;
    writeln!(file, "}}")?;
    writeln!(
        file,
        "void u16_to_bytes_le(unsigned int address, unsigned short operand) {{"
    )?;
    writeln!(file, "    memory[address] = operand & 0xFF;")?;
    writeln!(file, "    memory[address + 1] = operand >> 8;")?;
    writeln!(file, "}}")?;
    writeln!(file, "int main() {{")?;
    writeln!(file, "    unsigned short acc = 0;")?;
    loop {
        let instruction = match parse_instruction(&program[ip..]) {
            Ok(instruction) => instruction.1,
            Err(_) => return Err(Error::new(ErrorKind::Other, "Invalid instruction.")),
        };
        if translate_instruction_to_c(&mut file, instruction, &mut ip)? {
            break;
        }
    }
    writeln!(file, "}}")?;
    writeln!(file, "unsigned char memory[] = {{")?;
    for byte in program {
        writeln!(file, "    {}, ", byte)?;
    }
    writeln!(file, "}};")?;
    Ok(())
}

fn translate_instruction_to_c(
    file: &mut File,
    instruction: Instruction,
    ip: &mut usize,
) -> Result<bool> {
    use Instruction::*;
    match instruction {
        Terminate(operand) => {
            writeln!(file, "    addr_{}: return {};", *ip, operand)?;
            *ip += 2;
            return Ok(true);
        }
        Set(operand) => {
            writeln!(file, "    addr_{}: acc = {};", *ip, operand)?;
            *ip += 3;
        }
        Load(address) => {
            writeln!(
                file,
                "    addr_{}: acc = bytes_to_u16_le({});",
                *ip, address
            )?;
            *ip += 3;
        }
        Store(address) => {
            writeln!(file, "    addr_{}: u16_to_bytes_le({}, acc);", *ip, address)?;
            *ip += 3;
        }
        IndirectLoad(address) => {
            writeln!(
                file,
                "    addr_{}: acc = bytes_to_u16_le(bytes_to_u16_le({}));",
                ip, address
            )?;
            *ip += 3;
        }
        IndirectStore(address) => {
            writeln!(
                file,
                "    addr_{}: u16_to_bytes_le(bytes_to_u16_le({}), acc);",
                ip, address
            )?;
            *ip += 3;
        }
        Input(length) => {
            writeln!(file, "    addr_{}: {{", *ip)?;
            writeln!(file, "        char buf[{}];", length + 1)?;
            writeln!(file, "        int len;")?;
            writeln!(file, "        scanf(\"%{}s\", buf);", length)?;
            writeln!(file, "        len = strlen(buf);")?;
            writeln!(file, "        memcpy(memory + acc, buf, len);")?;
            writeln!(
                file,
                "        memset(memory + acc + len, ' ', {} - len);",
                length
            )?;
            writeln!(file, "    }}")?;
            *ip += 2;
        }
        Output(length) => {
            writeln!(file, "    addr_{}: for (int i = 0; i < {}; i++) {{ putchar(memory[acc + i] ? memory[acc + i] : ' '); }}", *ip, length)?;
            *ip += 2;
        }
        Add(address) => {
            writeln!(
                file,
                "    addr_{}: acc += bytes_to_u16_le({});",
                *ip, address
            )?;
            *ip += 3;
        }
        Subtract(address) => {
            writeln!(
                file,
                "    addr_{}: acc -= bytes_to_u16_le({});",
                *ip, address
            )?;
            *ip += 3;
        }
        Multiply(address) => {
            writeln!(
                file,
                "    addr_{}: acc *= bytes_to_u16_le({});",
                *ip, address
            )?;
            *ip += 3;
        }
        Divide(address) => {
            writeln!(
                file,
                "    addr_{}: acc /= bytes_to_u16_le({});",
                *ip, address
            )?;
            *ip += 3;
        }
        Remainder(address) => {
            writeln!(
                file,
                "    addr_{}: acc %= bytes_to_u16_le({});",
                *ip, address
            )?;
            *ip += 3;
        }
        Jump(address) => {
            writeln!(file, "    addr_{}: goto addr_{};", *ip, address)?;
            *ip += 3;
        }
        JumpIfZero(address) => {
            writeln!(file, "    addr_{}: if (!acc) goto addr_{};", *ip, address)?;
            *ip += 3;
        }
        JumpIfNonZero(address) => {
            writeln!(file, "    addr_{}: if (acc) goto addr_{};", *ip, address)?;
            *ip += 3;
        }
        JumpIfPositive(address) => {
            writeln!(
                file,
                "    addr_{}: if ((short)acc > 0) goto addr_{};",
                ip, address
            )?;
            *ip += 3;
        }
        JumpIfNegative(address) => {
            writeln!(
                file,
                "    addr_{}: if ((short)acc < 0) goto addr_{};",
                *ip, address
            )?;
            *ip += 3;
        }
        JumpIfNonPositive(address) => {
            writeln!(
                file,
                "    addr_{}: if ((short)acc <= 0) goto addr_{};",
                ip, address
            )?;
            *ip += 3;
        }
        JumpIfNonNegative(address) => {
            writeln!(
                file,
                "    addr_{}: if ((short)acc >= 0) goto addr_{};",
                ip, address
            )?;
            *ip += 3;
        }
        LoadByte(address) => {
            writeln!(file, "    addr_{}: acc = memory[{}];", *ip, address)?;
            *ip += 3;
        }
        StoreByte(address) => {
            writeln!(file, "    addr_{}: memory[{}] = acc & 0xFF;", *ip, address)?;
            *ip += 3;
        }
        IndirectLoadByte(address) => {
            writeln!(
                file,
                "    addr_{}: acc = memory[bytes_to_u16_le({})];",
                ip, address
            )?;
            *ip += 3;
        }
        IndirectStoreByte(address) => {
            writeln!(
                file,
                "    addr_{}: memory[bytes_to_u16_le({})] = acc & 0xFF;",
                *ip, address
            )?;
            *ip += 3;
        }
        Byte(_) => {
            *ip += 1;
        }
    }
    Ok(false)
}
