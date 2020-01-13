extern crate nom;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::le_u16;
use nom::number::complete::le_u8;
use nom::sequence::preceded;
use nom::Err;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
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
    Byte(u8),
}

impl Instruction {
    pub fn len(self) -> usize {
        use Instruction::*;
        match self {
            Byte(_) => 1,
            Terminate(_) | Input(_) | Output(_) => 2,
            _ => 3,
        }
    }
}

pub fn get_process_size(program: &[u8]) -> Result<u16, ()> {
    match le_u16(program) {
        Ok(ok) => Ok(ok.1),
        Err(Err::Incomplete(_)) => Err(()),
        Err(Err::Error((_, _))) => Err(()),
        Err(Err::Failure((_, _))) => Err(()),
    }
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

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
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
