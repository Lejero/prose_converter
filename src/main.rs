#![allow(dead_code)]
#![allow(unused_imports)]

use druid::widget::*;
use druid::*;
use regex::internal::Input;
use std::sync::Arc;

use prose_converter::http_parser::*;
use prose_converter::prose_interpreter::*;

use nom::character::complete::digit1;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
//use nom::combinator::*;
//use nom::error::*;
//use nom::number::complete::*;
use nom::*;

use std::str;

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
    let app_state = AppState {
        pre_format_text: INPUT.to_string(),
        post_format_text: convert_prose(&INPUT).to_string(),
    };

    // println(
    //     "{}",
    //     http_parser::request_line(b"GET http://www.w3.org/pub/WWW/TheProject.html HTTP/1.1"),
    // );
    let num = "42";
    let not_num = "4Z";

    let parser = all_consuming(digit1::<&str, ()>);
    //println!("Is {} a number: {}", num, be_i32(num.as_bytes()).unwrap().1);

    match parser(num) {
        Ok((inp, res)) => println!(
            "Is {} a number: {}. It is {}. Remaining input: {}",
            num, "true", res, inp
        ),
        Err(e) => println!("Is {} a number: {}", num, "false"),
    }
    match parser(not_num) {
        Ok((inp, res)) => println!(
            "Is {} a number: {}. It is {}. Remaining input: {}",
            not_num, "true", "", ""
        ),
        Err(e) => println!("Is {} a number: {}", not_num, "false"),
    }

    named!(exclaim, tag!("!"));
    named!(name, take_until!(":"));
    named!(type_ind, tag!(":"));
    named!(var, tag!("VAR|"));
    named!(val, take_until!("\n"));
    //let exclaim = tag!("!");
    let var_parser = tuple((exclaim, name, type_ind, var, val));
    //let res = var_parser(b"!BP1:VAR|The quick brown fox jumps over the lazy dog.\n");
    match var_parser(b"!BP1:VAR|The quick brown fox jumps over the lazy dog.\n") {
        Ok((inp, (_, name, _, _, val))) => println!(
            "Remaining Input: {}\nResult Name: {}\nResult Value: {}",
            str::from_utf8(inp).unwrap(),
            str::from_utf8(name).unwrap(),
            str::from_utf8(val).unwrap()
        ),
        Err(_) => println!("Error: "),
    };

    let expr = prose_interpreter::expression::Expression::from_string("2 + 2");

    AppLauncher::with_window(
        WindowDesc::new(build_ui)
            .window_size((1000.0, 600.0))
            .title("Test App"),
    )
    .launch(app_state)
}
