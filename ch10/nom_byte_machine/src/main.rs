extern crate nom;
use nom::branch::alt;
//use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
//use nom::bytes::complete::take;
use nom::combinator::map;
//use nom::error::ErrorKind;
//use nom::multi::many0;
use nom::number::complete::le_u16;
use nom::number::complete::le_u8;
use nom::sequence::preceded;
//use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
//use nom::Needed;

fn input_line(buffer: &mut [u8]) {
    let mut text = String::new();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    for i in 0..text.len().min(buffer.len()) {
        buffer[i] = text.as_bytes()[i].into();
    }
    for i in text.len()..buffer.len() {
        buffer[i] = 0;
    }
}

struct RegisterSet {
    ip: u16,
    acc: u16,
}

fn get_le_word(slice: &[u8], address: u16) -> u16 {
    slice[address as usize] as u16 + slice[address as usize + 1] as u16 * 256
}

fn set_le_word(slice: &mut [u8], address: u16, value: u16) {
    slice[address as usize] = (value % 256) as u8;
    slice[address as usize + 1] = (value / 256) as u8;
}

fn get_byte(slice: &[u8], address: u16) -> u16 {
    slice[address as usize] as u16
}

fn set_byte(slice: &mut [u8], address: u16, value: u16) {
    slice[address as usize] = (value % 256) as u8;
}

fn execute_instruction(
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
            r.ip += 3;
            r.acc = operand;
        }
        Load(address) => {
            r.ip += 3;
            r.acc = get_le_word(process, address);
        }
        Store(address) => {
            r.ip += 3;
            set_le_word(process, address, r.acc);
        }
        IndirectLoad(address) => {
            r.ip += 3;
            r.acc = get_le_word(process, get_le_word(process, address));
        }
        IndirectStore(address) => {
            r.ip += 3;
            set_le_word(process, get_le_word(process, address), r.acc);
        }
        Input(length) => {
            r.ip += 2;
            let address = r.acc as usize;
            input_line(&mut process[address..address + length as usize]);
        }
        Output(length) => {
            r.ip += 2;
            let address = r.acc as usize;
            for &byte in &process[address..address + length as usize] {
                print!("{}", if byte == 0 { ' ' } else { byte as char });
            }
        }
        Add(address) => {
            r.ip += 3;
            r.acc = r.acc.wrapping_add(get_le_word(process, address));
        }
        Subtract(address) => {
            r.ip += 3;
            r.acc = r.acc.wrapping_sub(get_le_word(process, address));
        }
        Multiply(address) => {
            r.ip += 3;
            r.acc = r.acc.wrapping_mul(get_le_word(process, address));
        }
        Divide(address) => {
            r.ip += 3;
            r.acc = r.acc.wrapping_div(get_le_word(process, address));
        }
        Remainder(address) => {
            r.ip += 3;
            r.acc = r.acc.wrapping_rem(get_le_word(process, address));
        }
        Jump(address) => {
            r.ip += 3;
            r.ip = address;
        }
        JumpIfZero(address) => {
            r.ip += 3;
            if r.acc == 0 {
                r.ip = address;
            }
        }
        JumpIfNonZero(address) => {
            r.ip += 3;
            if r.acc != 0 {
                r.ip = address;
            }
        }
        JumpIfPositive(address) => {
            r.ip += 3;
            if (r.acc as i16) > 0 {
                r.ip = address;
            }
        }
        JumpIfNegative(address) => {
            r.ip += 3;
            if (r.acc as i16) < 0 {
                r.ip = address;
            }
        }
        JumpIfNonPositive(address) => {
            r.ip += 3;
            if r.acc as i16 <= 0 {
                r.ip = address;
            }
        }
        JumpIfNonNegative(address) => {
            r.ip += 3;
            if r.acc as i16 >= 0 {
                r.ip = address;
            }
        }
        LoadByte(address) => {
            r.ip += 3;
            r.acc = get_byte(process, address);
        }
        StoreByte(address) => {
            r.ip += 3;
            set_byte(process, address, r.acc);
        }
        IndirectLoadByte(address) => {
            r.ip += 3;
            r.acc = get_byte(process, get_le_word(process, address));
        }
        IndirectStoreByte(address) => {
            r.ip += 3;
            set_byte(process, get_le_word(process, address), r.acc);
        }
    }
    return None;
}

// /*
fn execute_program(program: &[u8]) -> Result<u8, ()> {
    //let process_size_parsed = le_u16(program)?;
    fn get_process_size(program: &[u8]) -> Result<u16, ()> {
        match le_u16(program) {
            Ok(ok) => Ok(ok.1),
            Err(Err::Incomplete(_)) => Err(()),
            Err(Err::Error((_, _))) => Err(()),
            Err(Err::Failure((_, _))) => Err(()),
        }
    }
    let process_size_parsed: u16 = match get_process_size(program) {
        Ok(ok) => ok,
        Err(_) => return Err(()),
    };

    let mut process = vec![0u8; process_size_parsed as usize];
    process[0..program.len()].copy_from_slice(&program);

    //let mut process = Vec::from(program);
    //process.resize_with(process_size_parsed.1 as usize, Default::default);

    let mut registers = RegisterSet { ip: 2, acc: 0 };
    loop {
        let instruction = match parse_instruction(&process[registers.ip as usize..]) {
            Ok(instruction) => instruction.1,
            Err(_) => return Err(()),
        };
        println!(
            "Ip: {} Acc: {} Instr: {:?}",
            registers.ip, registers.acc, instruction
        );
        match execute_instruction(&mut process, &mut registers, instruction) {
            Some(return_code) => {
                return Ok(return_code);
            }
            _ => {}
        };
    }
}

/*
fn parse_opcode(input: &[u8]) -> IResult<&[u8], u8> {
    if input.len() == 0 {
        return Err(Err::Incomplete(Needed::Size(2)));
    }
    if input[0] != 0u8 {
        return Err(Err::Error((input, ErrorKind::Not)));
    }
    return Ok((&input[1..], 0u8));
}
 */

/*
#[derive(Debug)]
enum Instruction {
    Terminate(u8),
    Set(u8),
    Load(u8),
    Store(u8),
}

fn parse_terminate(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(
        tag(&[0u8]),
        map(take(1usize), |bytes: &[u8]| {
            Instruction::Terminate(bytes[0])
        }),
    )(input)
}

fn parse_load(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(
        tag(&[2u8]),
        map(take(1usize), |bytes: &[u8]| Instruction::Load(bytes[0])),
    )(input)
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    alt((parse_terminate, parse_load))(input)
}

fn parse_program(input: &[u8]) -> IResult<&[u8], Vec<Instruction>> {
    many0(parse_instruction)(input)
}
*/

// /*
#[derive(Debug)]
enum Instruction {
    Terminate(u8),
    Set(u16),
    Load(u16),
    Store(u16),
    IndirectLoad(u16),
    IndirectStore(u16),
    Input(u8),
    Output(u8),
    Add(u16),
    Subtract(u16),
    Multiply(u16),
    Divide(u16),
    Remainder(u16),
    Jump(u16),
    JumpIfZero(u16),
    JumpIfNonZero(u16),
    JumpIfPositive(u16),
    JumpIfNegative(u16),
    JumpIfNonPositive(u16),
    JumpIfNonNegative(u16),
    LoadByte(u16),
    StoreByte(u16),
    IndirectLoadByte(u16),
    IndirectStoreByte(u16),
}

fn parse_terminate(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x00"), map(le_u8, Instruction::Terminate))(input)
}

fn parse_set(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x01"), map(le_u16, Instruction::Set))(input)
}

fn parse_load(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x02"), map(le_u16, Instruction::Load))(input)
}

fn parse_store(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x03"), map(le_u16, Instruction::Store))(input)
}

fn parse_indirect_load(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x04"), map(le_u16, Instruction::IndirectLoad))(input)
}

fn parse_indirect_store(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x05"), map(le_u16, Instruction::IndirectStore))(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x06"), map(le_u8, Instruction::Input))(input)
}

fn parse_output(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x07"), map(le_u8, Instruction::Output))(input)
}

fn parse_add(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x08"), map(le_u16, Instruction::Add))(input)
}

fn parse_subtract(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x09"), map(le_u16, Instruction::Subtract))(input)
}

fn parse_multiply(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x0A"), map(le_u16, Instruction::Multiply))(input)
}

fn parse_divide(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x0B"), map(le_u16, Instruction::Divide))(input)
}

fn parse_remainder(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x0C"), map(le_u16, Instruction::Remainder))(input)
}

fn parse_jump(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x0D"), map(le_u16, Instruction::Jump))(input)
}

fn parse_jump_if_zero(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x0E"), map(le_u16, Instruction::JumpIfZero))(input)
}

fn parse_jump_if_nonzero(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x0F"), map(le_u16, Instruction::JumpIfNonZero))(input)
}

fn parse_jump_if_positive(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x10"), map(le_u16, Instruction::JumpIfPositive))(input)
}

fn parse_jump_if_negative(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x11"), map(le_u16, Instruction::JumpIfNegative))(input)
}

fn parse_jump_if_nonpositive(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x12"), map(le_u16, Instruction::JumpIfNonPositive))(input)
}

fn parse_jump_if_nonnegative(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x13"), map(le_u16, Instruction::JumpIfNonNegative))(input)
}

fn parse_load_byte(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x14"), map(le_u16, Instruction::LoadByte))(input)
}

fn parse_store_byte(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x15"), map(le_u16, Instruction::StoreByte))(input)
}

fn parse_indirect_load_byte(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x16"), map(le_u16, Instruction::IndirectLoadByte))(input)
}

fn parse_indirect_store_byte(input: &[u8]) -> IResult<&[u8], Instruction> {
    preceded(tag("\x17"), map(le_u16, Instruction::IndirectStoreByte))(input)
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    alt((
        alt((
            parse_terminate,
            parse_set,
            parse_load,
            parse_store,
            parse_indirect_load,
            parse_indirect_store,
            parse_input,
            parse_output,
            parse_add,
            parse_subtract,
            parse_multiply,
            parse_divide,
            parse_remainder,
            parse_jump,
            parse_jump_if_zero,
            parse_jump_if_nonzero,
            parse_jump_if_positive,
            parse_jump_if_negative,
            parse_jump_if_nonpositive,
            parse_jump_if_nonnegative,
        )),
        alt((
            parse_load_byte,
            parse_store_byte,
            parse_indirect_load_byte,
            parse_indirect_store_byte,
        )),
    ))(input)
}

// */
fn main() {
    /*
    println!(
        "le_u16:[{:#?}]",
        match le_u16(b"abd") {
            Ok(ok) => ok.0,
            Err(error) => match error {
                Err::Incomplete(_needed) => b"1",
                Err::Error((_rest, _kind)) => b"2",
                Err::Failure(_err) => b"3",
            },
        }
    );
    */

    let prog = vec![
        187, 2, // 0: 699
        // Let the user input the digits of the limit number.
        1, 28, 1, // 2, 0: set digits
        6, 5, // 5, 0: input 5
        // Initialize digit pointer.
        1, 28, 1, // 7, 0: set digits
        3, 33, 1, // 10, 0: store pos
        // If the digit is less than 0, parsing is ended.
        // 13, 0: before_parsing_number
        22, 33, 1, // 13, 0: indirect_load_byte pos
        9, 37, 1, // 16, 0: subtract ascii_zero
        17, 73, 0, // 19, 0: jump_if_negative after_parsing_number
        // If the digit is greater than 9, parsing is ended.
        22, 33, 1, // 22, 0: indirect_load_byte pos
        9, 37, 1, // 25, 0: subtract ascii_zero
        9, 35, 1, // 28, 0: subtract number_base
        19, 73, 0, // 31, 0: jump_if_nonnegative after_parsing_number
        // Multiply by 10 the current limit.
        2, 22, 1, // 34, 0: load limit
        10, 35, 1, // 37, 0: multiply number_base
        3, 22, 1, // 40, 0: store limit
        // Add next digit to current limit.
        22, 33, 1, // 43, 0: indirect_load_byte pos
        9, 37, 1, // 46, 0: subtract ascii_zero
        8, 22, 1, // 49, 0: add limit
        3, 22, 1, // 52, 0: store limit
        // Increment digit pointer
        2, 33, 1, // 55, 0: load pos
        8, 39, 1, // 58, 0: add one
        3, 33, 1, // 61, 0: store pos
        // If pos points to itself, the digit buffer is ended.
        1, 33, 1, // 64, 0: set pos
        9, 33, 1, // 67, 0: subtract pos
        15, 13, 0, // 70, 0: jump_if_nonzero before_parsing_number
        // 73, 0: after_parsing_number
        2, 41, 1, // 73, 0: load two
        3, 24, 1, // 76, 0: store i
        // 79, 0: before_computing_primes
        2, 24, 1, // 79, 0: load i
        9, 22, 1, // 82, 0: subtract limit
        19, 157, 0, // 85, 0: jump_if_nonnegative after_computing_primes
        1, 43, 1, // 88, 0: set primes
        8, 24, 1, // 91, 0: add i
        3, 33, 1, // 94, 0: store pos
        22, 33, 1, // 97, 0: indirect_load_byte pos
        15, 145, 0, // 100, 0: jump_if_nonzero after_setting_multiples
        2, 24, 1, // 103, 0: load i
        8, 24, 1, // 106, 0: add i
        3, 26, 1, // 109, 0: store j
        // 112, 0: before_setting_multiples
        9, 22, 1, // 112, 0: subtract limit
        19, 145, 0, // 115, 0: jump_if_nonnegative after_setting_multiples
        1, 43, 1, // 118, 0: set primes
        8, 26, 1, // 121, 0: add j
        3, 33, 1, // 124, 0: store pos
        2, 39, 1, // 127, 0: load one
        5, 33, 1, // 130, 0: indirect_store pos
        2, 26, 1, // 133, 0: load j
        8, 24, 1, // 136, 0: add i
        3, 26, 1, // 139, 0: store j
        13, 112, 0, // 142, 0: jump before_setting_multiples
        // 145, 0: after_setting_multiples
        2, 24, 1, // 145, 0: load i
        8, 39, 1, // 148, 0: add one
        3, 24, 1, // 151, 0: store i
        13, 79, 0, // 154, 0: jump before_computing_primes
        // 157, 0: after_computing_primes
        2, 41, 1, // 157, 0: load two
        3, 24, 1, // 160, 0: store i
        // 163, 0: before_printing_primes
        2, 24, 1, // 163, 0: load i
        9, 22, 1, // 166, 0: subtract limit
        19, 20, 1, // 169, 0: jump_if_nonnegative after_printing_all_primes
        1, 43, 1, // 172, 0: set primes
        8, 24, 1, // 175, 0: add i
        3, 33, 1, // 178, 0: store pos
        4, 33, 1, // 181, 0: indirect_load pos
        15, 8, 1, // 184, 0: jump_if_nonzero after_printing_a_prime
        // Format a prime number
        2, 24, 1, // 187, 0: load i
        3, 26, 1, // 190, 0: store j
        1, 33, 1, // 193, 0: set pos
        3, 33, 1, // 196, 0: store pos
        // 199, 0: before_generating_digits
        2, 33, 1, // 199, 0: load pos
        9, 39, 1, // 202, 0: subtract one
        3, 33, 1, // 205, 0: store pos
        2, 26, 1, // 208, 0: load j
        12, 35, 1, // 211, 0: remainder number_base
        8, 37, 1, // 214, 0: add ascii_zero
        5, 33, 1, // 217, 0: indirect_store pos
        2, 26, 1, // 220, 0: load j
        11, 35, 1, // 223, 0: divide number_base
        3, 26, 1, // 226, 0: store j
        15, 199, 0, // 229, 0: jump_if_nonzero before_generating_digits
        // Clear the initial spaces.
        // 232, 0: before_clearing_spaces
        1, 28, 1, // 232, 0: set digits
        9, 33, 1, // 235, 0: subtract pos
        14, 3, 1, // 238, 0: jump_if_zero after_clearing_spaces
        2, 33, 1, // 241, 0: load pos
        9, 39, 1, // 244, 0: subtract one
        3, 33, 1, // 247, 0: store pos
        1, 32, 0, // 250, 0: set 32 // blank
        5, 33, 1, // 253, 0: indirect_store pos
        13, 232, 0, // 0, 1: jump before_clearing_spaces
        // 3, 1: after_clearing_spaces

        // Emit the prime number.
        1, 28, 1, // 3, 1: set digits
        7, 5, // 6, 1: output 5
        // 8, 1: after_printing_a_prime
        2, 24, 1, // 8, 1: load i
        8, 39, 1, // 11, 1: add one
        3, 24, 1, // 14, 1: store i
        13, 163, 0, // 17, 1: jump before_printing_primes
        // 20, 1: after_printing_all_primes
        0, 0, // 20, 1: terminate 0
        // data
        0, 0, // 22, 1: limit: word 0
        0, 0, // 24, 1: i: word 0
        0, 0, // 26, 1: j: word 0
        0, 0, 0, 0, 0, // 28, 1: digits: array 5
        0, 0, // 33, 1: pos: word 0
        10, 0, // 35, 1: number_base: word 10
        48, 0, // 37, 1: ascii_zero: word 48
        1, 0, // 39, 1: one: word 1
        2, 0, // 41, 1: two: word 2
           // 43, 1: primes: array 400
    ];
    println!("{}", execute_program(&prog).unwrap());
    // */
}
