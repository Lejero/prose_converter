use lazy_static::*;
use regex::Regex;

#[derive(Clone)]
pub struct Variable {
    name: String,
    value: String,
}

pub fn convert_prose(input_var: &str) -> String {
    lazy_static! {
        //Single Line Variable Detection
        static ref SL_VAR: Regex = Regex::new(r"!(?P<name>\w+):VAR\|\s*(?P<val>[^\r\n]+)").unwrap();
        //Multi Line Variable Detection
        static ref ML_VAR: Regex = Regex::new(r"!(?P<name>\w+):VAR\{\s*(?P<val>[^\}]+)\}").unwrap();
    }

    //Prepare output as input copy
    let mut output: String = String::from(input_var);

    //Find Variable Instantiations, catalog and remove from output
    let mut vars: Vec<Variable> = Vec::new();
    for m in SL_VAR.captures_iter(input_var) {
        vars.push(Variable {
            name: String::from(&m["name"]),
            value: String::from(&m["val"]),
        });
        output = SL_VAR.replace_all(output.as_str(), r"").to_string();
    }
    for m in ML_VAR.captures_iter(input_var) {
        vars.push(Variable {
            name: String::from(&m["name"]),
            value: String::from(&m["val"]),
        });
        output = ML_VAR.replace_all(output.as_str(), r"").to_string();
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

    //Execute Variable Replacements
    for var in vars {
        let pattern: Regex = Regex::new((format!(r"!{}[^:]", var.name.as_str())).as_str()).unwrap();
        output = pattern
            .replace_all(output.as_str(), var.value.as_str())
            .to_string();
    }

    output
}
