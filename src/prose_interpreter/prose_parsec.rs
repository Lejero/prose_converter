use lazy_static::*;
use nom::bytes::complete::take;
use nom::number::complete::be_u16;
use nom::IResult;

#[derive(Clone)]
pub struct Variable {
    name: String,
    value: String,
}

pub fn length_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = be_u16(input)?;
    take(length)(input)
}

pub fn convert_prose(input_var: &str) -> String {
    let mut output: String = String::from(input_var);

    output
}

// pub fn parse_var_def(input: &str) -> IResult<&str, Variable> {}
