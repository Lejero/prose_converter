/**
 * TODO
 * Use function enum m
 */
use lazy_static::*;
use nom::character::complete::digit1;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
//use nom::combinator::*;
//use nom::error::*;
//use nom::number::complete::*;
use nom::branch::alt;
use nom::error::{ErrorKind, VerboseError, VerboseErrorKind};
use nom::number::complete::double;
use nom::*;
//use std::f64;
use super::operand::*;
use std::ops;
use std::result::Result;
use std::str::*;

pub enum Component {
    Operand(Operand),
    UnaryOp(fn(Operand) -> Operand),
    BinaryOp(BinaryOperator),
}

// pub enum BinaryOperator {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
// }

pub struct BinaryOperator(fn(Operand, Operand) -> Operand);

pub struct Expression {
    components: Vec<Component>,
}

named!(exact_float(&str) -> f64,
    exact!(map_res!(recognize!(double), f64::from_str))
);

// named!(exact_int(&str) -> i64,
//     exact!(map_res!(recognize!(i64!(nom::number::Endianness::Big)), i64::from_str))
// );

// named!(exact_bool(&str) -> bool,
//     exact!(alt!(
//         map!(alt!(tag!("true"), tag!("t")), |x| true),
//         map!(alt!(tag!("false"), tag!("f")), |x| true)
//     ))
// );

// named!(exact_unary_op(&str) -> i64,
//     exact!(map_res!(alt!(tdag!('!')), |x| Component::UnaryOp()))
// );
// fn exact_unary_op(input: &str) -> Component::UnaryOp {
//     match(input){
//         "!" => Component::UnaryOp()
//     }
// }

// fn exact_binary_op(input: &str) -> Result<(), BinaryOperator {
//     match (input) {
//         "+" => BinaryOperator::Add,
//         "-" => BinaryOperator::Subtract,
//         "*" => BinaryOperator::Multiply,
//         "/" => BinaryOperator::Divide,
//     }
// }

named!(exact_binary_op(&str) -> BinaryOperator,
    exact!(map_res!(recognize!(tag!("+")), |x| parse_binary_op(x)))
);

//Full
// named!(is_comp(&str) -> Component,
//     alt!(
//         map!(exact_int, |x| Component::Operand(Operand::Number(x))),
//         map!(exact_float, |x| Component::Operand(Operand::Integer(x))),
//         map!(exact_bool, |x| Component::Operand(Operand::Boolean(x))),
//         map!(exact_unary_op, |x| Component::Operand(Operand::Number(x))),
//         map!(exact_binary_op, |x| Component::Operand(Operand::Number(x)))
//     )
// );

//Tester
named!(is_comp(&str) -> Component,
    alt!(
        //map!(exact_int, |x| Component::Operand(Operand::Integer(x))) |
        map!(exact_float, |x| Component::Operand(Operand::Number(x))) |
        map!(exact_binary_op, |x| Component::BinaryOp(x))
    )
);

// fn parse_binary_op(input: &str) -> IResult<&str, BinaryOperator, (&str, ErrorKind)> {
//     match input {
//         "+" => Result::Ok((input, BinaryOperator(|l, r| l + r))),
//         // "-" => BinaryOperator::Subtract,
//         // "*" => BinaryOperator::Multiply,
//         // "/" => BinaryOperator::Divide,
//         _ => Result::Err(Err::Error((input, ErrorKind::Fix))),
//     }
// }

fn parse_binary_op(input: &str) -> Result<BinaryOperator, &str> {
    match input {
        "+" => Ok(BinaryOperator(|l, r| l + r)),
        // "-" => BinaryOperator::Subtract,
        // "*" => BinaryOperator::Multiply,
        // "/" => BinaryOperator::Divide,
        _ => Err("We're Panicing!"),
    }
}

// fn parse_binary_op(input: &str) -> BinaryOperator {
//     match (input) {
//         "+" => BinaryOperator(|l, r| l + r),
//         // "-" => BinaryOperator::Subtract,
//         // "*" => BinaryOperator::Multiply,
//         // "/" => BinaryOperator::Divide,
//         _ => panic!("I'm panicing!"),
//     }
// }

// fn error_parse_bin_op(input: &str) -> VerboseError<&str> {
//     VerboseError {
//         errors: vec![(
//             input,
//             VerboseErrorKind::Context(
//                 &(format!(
//                 "Could not parse input '{}'. It does not correspond to a known binary operator.",
//                 input
//             )),
//             ),
//         )],
//     }
// }

// fn is_component(input: &str) -> R

fn identify_component(input: &str) -> Option<Component> {
    match is_comp(input) {
        Ok((inp, comp)) => Some(comp),
        Err(_) => {
            println!("Error: ");
            None
        }
    }
}

//How to distinguish between components? Assume there is white space between each component?

impl Expression {
    pub fn from_string(input: &str) -> Self {
        let mut components: Vec<Component> = Vec::new();

        for comp in input.split(" ").filter(|s| s.len() > 0) {
            match identify_component(comp) {
                Some(c) => components.push(c),
                None => {}
            }
        }

        Expression { components }
    }
}
