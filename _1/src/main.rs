use std::fs;

fn main() {
    let input = read_input();
    let sum : u32 = input.into_iter().map(|x| extract_value(&x)).sum();

    println!("{}", sum);
}

fn extract_value(s : &str) -> u32 {
    //let a = s.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap();
    //let b = s.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap();

    let mut a :u32 = 0;
    for i in 0 .. s.len() {
        let ss = s.get(i..).unwrap();
        if is_digit(ss) {
            a = to_digit(ss);
            break;
        }
    }

    let mut b : u32 = 0;
    for i in (0 .. s.len()).rev() {
        let ss = s.get(i..).unwrap();
        if is_digit(ss) {
            b = to_digit(ss);
            break;
        }
    }

    let x = a * 10 + b;
    println!("{} -> {}", s, x);
    return x;
}

const SDIGITS : [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn is_digit(s : &str) -> bool {
    if s.len() > 0 && s.chars().next().unwrap().is_ascii_digit() {
        return true;
    }
    else {
        return SDIGITS.into_iter().find(|x| s.starts_with(x)) != None;
    }
}

fn to_digit(s : &str) -> u32 {
    if s.len() > 0 && s.chars().next().unwrap().is_ascii_digit() {
        return s.chars().next().unwrap().to_digit(10).unwrap();
    }
    else {
        return SDIGITS.into_iter().position(|x| s.starts_with(x)).unwrap() as u32 + 1;
    }
}

fn read_input() -> Vec<String> {
    let x = fs::read_to_string("input").expect("No input");
    let lines:Vec<_> = x.lines().map(|x| x.to_string()).collect();

    return lines;
}