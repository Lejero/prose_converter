#![allow(dead_code)]
#![allow(unused_imports)]

use prose_converter as pc;

use druid::widget::*;
use druid::*;

use pc::prose_interpreter::expr::*;
use pc::prose_interpreter::prose_parsec::*;
use pc::prose_interpreter::*;
use pc::TerminateOnCloseDelegate;

extern crate nom;
use nom::character::complete::{alphanumeric1, multispace0, multispace1, space0, space1};
use nom::number::complete::{double, f64};
use nom::*;

use std::fmt::Debug;
use std::vec::*;

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
    pub post_format_text_regex: String,
    pub post_format_text_parsec: String,
    pub tester_state: TesterState,
}

#[derive(Clone, Data, Lens)]
struct TesterState {
    pub ps_tester_in: String,
    pub ps_tester_out: String,
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_ui)
        .window_size((1000.0, 600.0))
        .title("Prose Converter");

    let mut app_state = AppState {
        pre_format_text: INPUT.to_string(),
        post_format_text_regex: prose_regex::convert_prose(&INPUT).to_string(),
        post_format_text_parsec: "Unimplemented".to_string(),
        tester_state: TesterState {
            ps_tester_in: " !BP1:VAR|Hello
!BP2:VAR|Goodbye"
                .to_string(),
            ps_tester_out: "NOT UPDATED".to_string(),
        },
    };
    // test parserA
    // named!(get_output<&str,&str>,
    //     tag!("!BP1:VAR|")
    // );

    //TODO: multispace0 gets me an error for some reason and multispace1 requires whitespace before the variable.
    named!(
        robust<&str, Vec<Variable>>,
        many0!(
            do_parse!(
                multispace1 >>
                tag!("!") >>
                name: alphanumeric1 >>
                tag!(":VAR|") >>
                value: alphanumeric1 >>
                (Variable {name: name.to_string(), value: value.to_string()})
            ))
    );
    //named!(wspace<&str, &str>, delimited)

    let res = robust(app_state.tester_state.ps_tester_in.as_str());
    // let ok_res: &Vec<(&str, &str)> =
    match &res {
        Ok((_, var_vec)) => {
            app_state.tester_state.ps_tester_out = format!(
                "Full Result
{0:?}
    
Vars:
{1}",
                res,
                pretty_print_var_vec(var_vec)
            );
        }
        _ => {}
    };

    //let _expr = expression::Expression::from_string("2 + 2");

    AppLauncher::with_window(window)
        .delegate(TerminateOnCloseDelegate {})
        .launch(app_state)
}

fn build_ui() -> impl Widget<AppState> {
    Padding::new(
        2.0,
        Flex::column()
            .with_flex_child(
                Flex::row()
                    .with_flex_child(
                        build_input::<AppState>().lens(AppState::pre_format_text),
                        1.0,
                    )
                    .with_spacer(10.0)
                    .with_flex_child(
                        build_regex_output::<AppState>().lens(AppState::post_format_text_regex),
                        1.0,
                    )
                    .with_spacer(10.0)
                    .with_flex_child(
                        build_parsec_output::<AppState>().lens(AppState::post_format_text_parsec),
                        1.0,
                    )
                    .with_spacer(10.0)
                    .with_flex_child(
                        build_parsec_tester_results::<AppState>()
                            .lens(lens!(AppState, tester_state)),
                        1.0,
                    ),
                1.0,
            )
            .with_flex_child(
                Align::new(
                    UnitPoint::BOTTOM,
                    Button::new("Convert").on_click(|_ctx, data: &mut AppState, _env| {
                        data.post_format_text_regex =
                            prose_regex::convert_prose(&data.pre_format_text);
                    }),
                ),
                1.0,
            ),
    )
}
fn build_input<T: Data>() -> impl Widget<String> {
    Flex::column()
        .with_child(Align::new(UnitPoint::TOP_LEFT, Label::new("Input:")))
        .with_spacer(10.0)
        .with_flex_child(
            Align::new(
                UnitPoint::TOP_LEFT,
                Label::dynamic(|data, _| format!("{}", data))
                    .with_line_break_mode(LineBreaking::WordWrap),
            ),
            1.0,
        )
}

fn build_regex_output<T: Data>() -> impl Widget<String> {
    Flex::column()
        .with_child(Align::new(UnitPoint::TOP_LEFT, Label::new("Regex Output:")))
        .with_spacer(10.0)
        .with_flex_child(
            Align::new(
                UnitPoint::TOP_LEFT,
                Label::dynamic(|data, _| format!("{}", data))
                    .with_line_break_mode(LineBreaking::WordWrap),
            ),
            1.0,
        )
}

fn build_parsec_output<T: Data>() -> impl Widget<String> {
    Flex::column()
        .with_child(Align::new(
            UnitPoint::TOP_LEFT,
            Label::new("Parsec Output:"),
        ))
        .with_spacer(10.0)
        .with_flex_child(
            Align::new(
                UnitPoint::TOP_LEFT,
                Label::dynamic(|data, _| format!("{}", data))
                    .with_line_break_mode(LineBreaking::WordWrap),
            ),
            1.0,
        )
}

fn build_parsec_tester_results<T: Data>() -> impl Widget<TesterState> {
    Flex::column()
        .with_child(Align::new(UnitPoint::TOP_LEFT, Label::new("Parsec Test:")))
        .with_spacer(10.0)
        .with_child(Align::new(UnitPoint::TOP_LEFT, Label::new("Input:")))
        .with_spacer(10.0)
        .with_flex_child(
            Align::new(
                UnitPoint::TOP_LEFT,
                Label::dynamic(|data: &TesterState, _| format!("{}", data.ps_tester_in))
                    .with_line_break_mode(LineBreaking::WordWrap),
            ),
            1.0,
        )
        .with_spacer(10.0)
        .with_child(Align::new(UnitPoint::TOP_LEFT, Label::new("Output:")))
        .with_spacer(10.0)
        .with_flex_child(
            Align::new(
                UnitPoint::TOP_LEFT,
                Label::dynamic(|data: &TesterState, _| format!("{}", data.ps_tester_out))
                    .with_line_break_mode(LineBreaking::WordWrap),
            ),
            1.0,
        )
}
