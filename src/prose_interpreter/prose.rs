use lazy_static::*;
use regex::Regex;

#[derive(Clone)]
pub struct Variable {
    name: String,
    value: String,
}

pub fn convert_prose(input_var: &str) -> String {
    lazy_static! {
        static ref SLVar: Regex = Regex::new(r"!(?P<name>\w+):VAR\|\s*(?P<val>[^\r\n]+)").unwrap();
        static ref MLVar: Regex = Regex::new(r"!(?P<name>\w+):VAR\{\s*(?P<val>[^\}]+)").unwrap();
    }

    let mut vars: Vec<Variable> = Vec::new();

    for m in SLVar.captures_iter(input_var) {
        vars.push(Variable {
            name: String::from(&m["name"]),
            value: String::from(&m["val"]),
        })
    }
    for m in MLVar.captures_iter(input_var) {
        vars.push(Variable {
            name: String::from(&m["name"]),
            value: String::from(&m["val"]),
        })
    }
    // for line in input_var.split("\n") {
    //     match RE(line) {
    //         None => (),
    //         Some(x) => vars.push(Variable {
    //             name: String::from(&x[0]),
    //             value: String::from(&x[1]),
    //         }), //Note: index 0 is the entire pattern so 1 is the first subpattern.
    //     };
    // }

    let mut output: String = String::from(input_var);
    for var in vars {
        let pattern: Regex =
            Regex::new((format!(r"!({})[^:]", var.name.as_str())).as_str()).unwrap();
        output = pattern
            .replace_all(output.as_str(), var.value.as_str())
            .to_string();
    }

    output
}
