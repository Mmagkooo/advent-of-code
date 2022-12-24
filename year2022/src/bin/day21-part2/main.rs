/**
 * Assumption: no monkey is called by two (or more) other monkeys
 */
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

fn collect_parents(
    func_name: &str,
    funcs: &HashMap<String, Func>,
    parents: &mut HashMap<String, String>,
) {
    let func = funcs.get(func_name).expect("Invalid func name");
    match func {
        Func::Constant(_) => (),
        Func::Binary { arg1, arg2, op: _ } => {
            parents.insert(arg1.to_string(), func_name.to_string());
            collect_parents(arg1, funcs, parents);

            parents.insert(arg2.to_string(), func_name.to_string());
            collect_parents(arg2, funcs, parents);
        }
    }
}

fn get_other_arg<'a>(arg1: &'a String, arg2: &'a String, caller_name: &'a String) -> &'a String {
    if caller_name == arg1 {
        return arg2;
    } else if caller_name == arg2 {
        return arg1;
    }
    panic!("Caller name {caller_name} not one of {arg1}, {arg2}");
}

fn find_value(
    func_name: &str,
    funcs: &HashMap<String, Func>,
    parents: &HashMap<String, String>,
) -> i64 {
    let func_name_string = func_name.to_string();

    let parent_name = parents.get(func_name).unwrap();
    let parent_func = funcs.get(parent_name).unwrap();
    if parent_name == "root" {
        return match parent_func {
            Func::Constant(_) => panic!("Root shouldn't be constant"),
            Func::Binary { arg1, arg2, op: _ } => {
                let other_arg = get_other_arg(arg1, arg2, &func_name_string);
                let evaluated = eval(other_arg, funcs);
                println!("Other arg in for func=root: {other_arg}; evaluated={evaluated}");
                evaluated
            }
        }
    }

    let parent_value = find_value(parent_name, funcs, parents);
    println!("func_name: {func_name}, parent_name: {parent_name}, parent_value: {parent_value}");

    match parent_func {
        Func::Constant(_) => panic!("Parent shouldn't be constant"),
        Func::Binary { arg1, arg2, op } => {
            return match op.as_str() {
                "+" => {
                    let other_arg = get_other_arg(arg1, arg2, &func_name_string);
                    parent_value - eval(other_arg, funcs)
                }
                "-" => {
                    if func_name == arg1 {
                        parent_value + eval(arg2, funcs)
                    } else if func_name == arg2 {
                        eval(arg1, funcs) - parent_value
                    } else {
                        panic!("Invalid func name");
                    }
                }
                "*" => {
                    let other_arg = get_other_arg(arg1, arg2, &func_name_string);
                    parent_value / eval(other_arg, funcs)
                }
                "/" => {
                    if func_name == arg1 {
                        parent_value * eval(arg2, funcs)
                    } else if func_name == arg2 {
                        eval(arg1, funcs) / parent_value
                    } else {
                        panic!("Invalid func name");
                    }
                }
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

    let mut parents: HashMap<String, String> = HashMap::new();
    collect_parents("root", &funcs, &mut parents);

    let humn_value = find_value("humn", &funcs, &parents);
    println!("{}", humn_value);
}
