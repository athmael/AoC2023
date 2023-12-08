use std::fs;

fn main() {
    let input = read_input();

    let x : u32 = input.iter()
        .map(|line| parse_game(line))
        .filter(|&game| game.1)
        .map(|game|game.0)
        .sum();

    println!("{}", x);
}

fn parse_game(s : &str) -> (u32, bool) {

    let mut ss = s.split(":");
    let gidstr = ss.next().unwrap();
    let clrs = ss.next().unwrap();

    let gid = gidstr.chars().filter(char::is_ascii_digit).collect::<String>().parse::<u32>().unwrap();
    let y = clrs.split(";")
        .map(|s| s.split(",").map(|s| to_cubes(s)).all(|x| is_possible(x)))
        .all(|x| x);

    (gid, y)
}

#[test]
fn test_parse_game() {
    assert_eq!(parse_game("Game 56: 13 green; 14 blue, 12 red"), (56, true));
    assert_eq!(parse_game("Game 56: 14 green; 2 blue, 4 red"), (56, false));
    assert_eq!(parse_game("Game 56: 4 green; 15 blue, 4 red"), (56, false));
    assert_eq!(parse_game("Game 56: 4 green; 2 blue, 13 red"), (56, false));
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green
}

fn to_cubes(s : &str) -> (Color, u32) {

    let mut ss = s.trim().split_ascii_whitespace();
    let n = ss.next().unwrap().parse::<u32>().unwrap();
    let clr = match ss.next().unwrap() {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        _ => {
            panic!();
            Color::Red
        }
    };

    (clr, n)
}

#[test]
fn test_to_cubes() {
    assert_eq!(to_cubes(" 1 red "), (Color::Red, 1));
    assert_eq!(to_cubes("2 green "), (Color::Green, 2));
    assert_eq!(to_cubes("4 blue"), (Color::Blue, 4));
}

fn is_possible(case : (Color, u32)) -> bool {
    match case.0 {
        Color::Red => case.1 <= 12,
        Color::Green => case.1 <= 13,
        Color::Blue => case.1 <= 14,
    }
}

fn read_input() -> Vec<String> {
    let x = fs::read_to_string("input").expect("No input");
    let lines:Vec<_> = x.lines().map(|x| x.to_string()).collect();

    return lines;
}


