use lazy_static::*;
use nom::character::complete::digit1;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
//use nom::combinator::*;
//use nom::error::*;
//use nom::number::complete::*;
use nom::branch::alt;
use nom::number::complete::double;
use nom::*;
//use std::f64;
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

pub enum Component {
    Operand(Operand),
    UnaryOp(fn(Operand) -> Operand),
}

pub struct Expression {
    components: Vec<Component>,
}

named!(is_float(&str) -> f64,
    map_res!(recognize!(double), f64::from_str)
);

// named!(is_int(&str) -> i64,
//     map_res!(recognize!(i64!(nom::number::Endianness::Big)), i64::from_str)
// );

named!(is_component(&str) -> Component,
    map_res!(is_float, Operand::Number)
);

fn identify_component(input: &str) -> Option<Component> {
    match is_component(input) {
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
