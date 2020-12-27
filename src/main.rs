#![allow(dead_code)]
#![allow(unused_imports)]

use druid::widget::*;
use druid::*;
use regex::internal::Input;
use std::sync::Arc;

use prose_converter::http_parser::*;
use prose_converter::prose_interpreter::*;

use prose_converter::prose_interpreter::expr::*;

use nom::character::complete::digit1;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
//use nom::combinator::*;
//use nom::error::*;
//use nom::number::complete::*;
use nom::branch::alt;
use nom::number::complete::double;
use nom::*;

use std::str;
use std::str::*;

const INPUT: &str = "!BP1:VAR| The quick brown fox jumps over the lazy dog.

!BP2:VAR|Hello World!

!ML:VAR{Line One
Line Two}

!E1:EXPR|2 + 2

Hey, do you know any good boilerplate text sayings?

How about '!BP1'?

Nice! I prefer the classic '!BP2' though.

OK, but can this thing handle multi line?

It sure can! Look:
!ML


Pop Quiz! What is 2 + 2?

It is !E1";

#[derive(Clone, Data, Lens)]
struct AppState {
    pub pre_format_text: String,
    pub post_format_text: String,
}

fn build_ui() -> impl Widget<AppState> {
    Padding::new(
        5.0,
        Flex::column()
            .with_flex_child(
                Flex::row()
                    .with_flex_child(
                        Label::dynamic(|data, _| format!("{}", data))
                            .expand_width()
                            .expand_height()
                            .lens(AppState::pre_format_text),
                        1.0,
                    )
                    .with_flex_child(
                        Label::dynamic(|data, _| format!("{}", data))
                            .expand_width()
                            .expand_height()
                            .lens(AppState::post_format_text),
                        1.0,
                    ),
                1.0,
            )
            .with_flex_child(
                Button::new("Convert").on_click(|_ctx, data: &mut AppState, _env| {
                    data.post_format_text = convert_prose(&data.pre_format_text);
                }),
                1.0,
            ),
    )
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_ui)
        .window_size((1000.0, 600.0))
        .title("Prose Converter");

    let app_state = AppState {
        pre_format_text: INPUT.to_string(),
        post_format_text: convert_prose(&INPUT).to_string(),
    };

    named!(exclaim, tag!("!"));
    named!(name, take_until!(":"));
    named!(type_ind, tag!(":"));
    named!(var, tag!("VAR|"));
    named!(val, take_until!("\n"));
    //let exclaim = tag!("!");
    let var_parser = tuple((exclaim, name, type_ind, var, val));
    //let res = var_parser(b"!BP1:VAR|The quick brown fox jumps over the lazy dog.\n");
    // match var_parser(b"!BP1:VAR|The quick brown fox jumps over the lazy dog.\n") {
    //     Ok((inp, (_, name, _, _, val))) => println!(
    //         "Remaining Input: {}\nResult Name: {}\nResult Value: {}",
    //         str::from_utf8(inp).unwrap(),
    //         str::from_utf8(name).unwrap(),
    //         str::from_utf8(val).unwrap()
    //     ),
    //     Err(_) => println!("Error: "),
    // };
    named!(exact_is_float(&str) -> f64,
        exact!(map_res!(recognize!(double), f64::from_str))
    );

    match exact_is_float("42") {
        Ok((inp, num)) => println!("Remaining Input: '{}', Result Value: {}", inp, num),
        Err(_) => println!("Error: "),
    };
    match exact_is_float("4Z") {
        Ok((inp, num)) => println!("Remaining Input: '{}', Result Value: {}", inp, num),
        Err(_) => println!("Error: "),
    };

    let expr = expression::Expression::from_string("2 + 2");

    AppLauncher::with_window(window)
        .delegate(MyAppDelegate {})
        .launch(app_state)
}

struct MyAppDelegate {}

impl<T: Data> AppDelegate<T> for MyAppDelegate {
    fn window_removed(&mut self, _id: WindowId, _data: &mut T, _env: &Env, _ctx: &mut DelegateCtx) {
        std::process::exit(0x0000);
    }
}
