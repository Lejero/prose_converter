use lazy_static::*;
use nom::character::complete::digit1;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
//use nom::combinator::*;
//use nom::error::*;
//use nom::number::complete::*;
use nom::branch::alt;
use nom::error::{VerboseError, VerboseErrorKind};
use nom::number::complete::double;
use nom::*;
//use std::f64;
use std::ops;
use std::str::*;

#[derive(Clone)]
pub enum Operand {
    Boolean(bool),
    Integer(i64),
    Number(f64),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Boolean(b) => write!(f, "{}", b),
            Operand::Integer(i) => write!(f, "{}", i),
            Operand::Number(n) => write!(f, "{}", n),
        }
    }
}

impl ops::Add<Self> for Operand {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self) {
            Operand::Number(left) => {
                if let Operand::Number(right) = rhs {
                    Operand::Number(left + right)
                } else {
                    panic!("if let failed for ops::Add trait on Operand struct.")
                }
            }
            _ => panic!("We're Panicing!"),
        }
    }
}
