use std::{fs, iter};
use itertools::Itertools;

fn main() {
    let input = read_input();

    let mut lineEntites : Vec<Vec<Entity>> = vec![];

    input.iter()
        .for_each(|line| lineEntites.push(parse_line(line)));

    let mut lineBnd : Vec<Vec<usize>> = vec![];
    lineBnd.resize(lineEntites.len() + 1, vec![]);

    for (lineNo,v) in lineEntites.iter().enumerate() {
        v.iter().for_each(
            |e| if let Entity::Symbol(sb) = e {
                sb.get_boundaries(lineNo).iter().for_each(|bnd| lineBnd.get_mut(bnd.y).unwrap().push(bnd.x))
                }
            )
    }

    let mut nbrs : Vec<u32> = vec![];

    for (lineNo,v) in lineEntites.iter().enumerate() {
        let bnd = lineBnd.get(lineNo).unwrap();

        v.iter().for_each(
            |e| if let Entity::Number(nd) = e {
                if nd.touches(bnd) { nbrs.push(nd.v); }
                }
            )
    }

    let x : u32 = nbrs.iter().sum();
    print!("{}", x)


    // let x : u32 = input.iter()
    //     .map(|line| parse_game_p2(line).1)
    //     .sum();

   // println!("{}", x);
}

#[derive(Debug)]
struct Point { x : usize, y : usize }

#[derive(Debug)]
struct NumberData {
    start : usize,
    len : usize,
    v : u32
}

impl NumberData {
    fn touches(&self, v : &Vec<usize>) -> bool {
        for &i in v {
            if i >= self.start && i < (self.start + self.len) {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug)]
struct SymbolData {
    start : usize,
}

impl SymbolData {
    fn get_boundaries(&self, lineNo : usize) -> Vec<Point> {
        let x =  ((lineNo as isize - 1)..=(lineNo as isize + 1 ))
            .cartesian_product((self.start as isize - 1)..=(self.start as isize + 1))
            .filter(|p| p.0 >= 0 && p.1 >= 0)
            .map(|p| Point{x : p.1 as usize, y : p.0 as usize})
            .collect::<Vec<Point>>();

        println!("{},{} -> {:?}", &lineNo, self.start, &x);

        return x;
    }
}

#[derive(Debug)]
enum Entity {
    Number(NumberData),
    Symbol(SymbolData)
}

fn parse_line(line : &str) -> Vec<Entity> {
    let mut nums : Vec<NumberData> = vec![];

    line.char_indices()
        .for_each(|c| {
            if char::is_ascii_digit(&c.1) {
                match nums.last_mut() {
                    None => nums.push(NumberData { start: c.0, len: 1, v: 0 }),
                    Some(nb) => {
                        if c.0 == nb.start + nb.len {
                            nb.len += 1;
                        }
                        else {
                            nums.push(NumberData { start: c.0, len: 1, v: 0 });
                        }
                    }

                }
            }
        });

    nums.iter_mut().for_each(|n: &mut NumberData| n.v = line[n.start..n.start+n.len].parse::<u32>().unwrap());
    let mut ents : Vec<Entity> = nums.into_iter().map(|n| Entity::Number(n)).collect();

    ents.append(&mut line.char_indices()
        .filter(|c| !char::is_ascii_digit(&c.1) && c.1 != '.')
        .map(|c| Entity::Symbol(SymbolData{start : c.0}))
        .collect::<Vec<Entity>>());

    return ents;
}

#[test]
fn test_parse_line() {
    let fut = parse_line("..12..");
    assert!(fut.len() == 1);
    assert!(if let Entity::Number(x) = &fut.first().unwrap() {x.v == 12} else {false} );

    let fut = parse_line("12..");
    assert!(fut.len() == 1);
    assert!(if let Entity::Number(x) = &fut.first().unwrap() {x.v == 12} else {false} );

    let fut = parse_line("..12");
    assert!(fut.len() == 1);
    assert!(if let Entity::Number(x) = &fut.first().unwrap() {x.v == 12} else {false} );

    let fut = parse_line(".13..12");
    assert!(fut.len() == 2);
    assert!(if let Entity::Number(x) = &fut.get(0).unwrap() {x.v == 13} else {false} );
    assert!(if let Entity::Number(x) = &fut.get(1).unwrap() {x.v == 12} else {false} );

    let fut = parse_line(";..");
    assert!(fut.len() == 1);
    assert!(if let Entity::Symbol(x) = &fut.first().unwrap() {x.start == 0} else {false} );

    let fut = parse_line(".*.");
    assert!(fut.len() == 1);
    assert!(if let Entity::Symbol(x) = &fut.first().unwrap() {x.start == 1} else {false} );

    let fut = parse_line(".13.!.12");
    assert!(fut.len() == 3);
    assert!(if let Entity::Number(x) = &fut.get(0).unwrap() {x.v == 13} else {false} );
    assert!(if let Entity::Number(x) = &fut.get(1).unwrap() {x.v == 12} else {false} );
    assert!(if let Entity::Symbol(x) = &fut.get(2).unwrap() {x.start == 4} else {false} );
}

fn read_input() -> Vec<String> {
    let x = fs::read_to_string("input").expect("No input");
    let lines:Vec<_> = x.lines().map(|x| x.to_string()).collect();

    return lines;
}

