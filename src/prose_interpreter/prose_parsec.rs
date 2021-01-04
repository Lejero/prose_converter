use lazy_static::*;
use nom::bytes::complete::take;
use nom::character::complete::{alphanumeric1, multispace0, multispace1, space0, space1};
use nom::number::complete::be_u16;
use nom::*;

use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub value: String,
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.value)
    }
}

pub fn pretty_print_var_vec(input: &Vec<Variable>) -> String {
    let mut output: String = "[\n".to_string();

    for x in input.iter() {
        output += format!("  ({}, {})\n", x.name, x.value).as_str();
    }
    output += "]";
    output
}

// named!(
//     robust<&str, Vec<Variable>>,
//     many0!(
//         do_parse!(
//             multispace1 >>
//             tag!("!") >>
//             name: alphanumeric1 >>
//             tag!(":VAR|") >>
//             value: alphanumeric1 >>
//             Variable {name, value}
//         ))
// );

pub fn length_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = be_u16(input)?;
    take(length)(input)
}

pub fn convert_prose(input_var: &str) -> String {
    let mut output: String = String::from(input_var);

    //Find Variable Instantiations, catalog and remove from output

    //Execute Variable Replacements

    output
}

pub fn dump_to_out<T: Debug>(res: IResult<&str, T>) {
    println!("{}", dump_to_string(res))
}
pub fn dump_to_string<T: Debug>(res: IResult<&str, T>) -> String {
    match res {
        IResult::Ok((remaining, found)) => {
            format!("Done {:?} {:?}", found, remaining)
        }
        IResult::Err(err) => match err {
            Err::Incomplete(needed) => {
                format!("Needed {:?}", needed)
            }
            Err::Error(error) => {
                format!("Error {:?}", error)
            }
            Err::Failure(fail) => {
                format!("Failure {:?}", fail)
            }
        },
    }
}

// pub fn parse_var_def(input: &str) -> IResult<&str, Variable> {}
