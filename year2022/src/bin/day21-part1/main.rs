use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    str::FromStr,
};

#[derive(Debug)]
enum Func {
    Binary {
        arg1: String,
        arg2: String,
        op: String,
    },
    Constant(i64),
}

#[derive(Debug)]
struct ParseFuncError;

impl FromStr for Func {
    type Err = ParseFuncError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        match split.len() {
            1 => Ok(Func::Constant(split[0].parse().expect("Cannot parse"))),
            3 => Ok(Func::Binary {
                arg1: split[0].to_string(),
                arg2: split[2].to_string(),
                op: split[1].to_string(),
            }),
            _ => Err(ParseFuncError),
        }
    }
}

fn eval(func_name: &str, funcs: &HashMap<String, Func>) -> i64 {
    let func = funcs.get(func_name).expect("Invalid func name");
    match func {
        Func::Constant(value) => *value,
        Func::Binary { arg1, arg2, op } => {
            let arg1 = eval(arg1, funcs);
            let arg2 = eval(arg2, funcs);
            match op.as_str() {
                "+" => arg1 + arg2,
                "-" => arg1 - arg2,
                "*" => arg1 * arg2,
                "/" => arg1 / arg2,
                _ => panic!("Invalid operator: {op}"),
            }
        }
    }
}

fn main() {
    let funcs: HashMap<String, Func> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|line| {
            line.split(": ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split_line| {
            let func_name = split_line[0].clone();
            let func = Func::from_str(&split_line[1]).unwrap();
            return (func_name, func);
        })
        .collect();

    println!("{}", eval("root", &funcs));
}
