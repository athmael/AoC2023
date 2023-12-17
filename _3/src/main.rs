use std::{fs, iter};
use itertools::Itertools;

fn main() {
    let input = read_input();

    let mut lineNumbers : Vec<Vec<NumberData>> = vec![];

    input.iter()
        .for_each(|line| lineNumbers.push(parse_line_numbers(line)));

    let mut lineSymbols : Vec<Vec<SymbolData>> = vec![];

    input.iter()
        .for_each(|line| lineSymbols.push(parse_line_symbols(line)));

    let mut result : u32 = 0;

   for (lineNo,v) in lineSymbols.iter().enumerate() {
        for ls in lineSymbols.get(lineNo).unwrap() {
            let bnd = ls.get_boundaries(lineNo);

            let numbers = get_adjacent_numbers_from_line(lineNo, &bnd, &lineNumbers);
            if numbers.len() == 2 {
                result += numbers.iter().map(|n| n.v).product::<u32>();
            }
        }
    }

   println!("{}", result);




    // let x : u32 = input.iter()
    //     .map(|line| parse_game_p2(line).1)
    //     .sum();

   // println!("{}", x);
}

fn get_adjacent_numbers_from_line<'a>(centerline : usize, adj : &Vec<Vec<usize>>, numbers : &'a Vec<Vec<NumberData>>) -> Vec<&'a NumberData> {
    let mut result : Vec<&NumberData> = vec![];

    let baseline = match centerline == 0 { true => centerline, false => centerline - 1 };

    for lineNo in baseline..(baseline+adj.len()) {
        let lnum = numbers.get(lineNo).unwrap();
        let ladj = adj.get(lineNo - baseline).unwrap();

        //let x : Vec<NumberData> = lnum.iter().filter(|&n| n.touches(ladj)).map(|&n| -> NumberData { n.clone()}).collect();

        result.extend( lnum.iter().filter(|n| n.touches(ladj)) );
    }

    return result;
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
    fn get_boundaries(&self, lineNo : usize) -> Vec<Vec<usize>> {

        let mut result : Vec<Vec<usize>> = vec![];

        let topbtml : Vec<usize> = ((self.start as isize - 1)..=(self.start as isize + 1))
            .filter(|&p| p >= 0)
            .map(|p| p as usize)
            .collect_vec();

        if lineNo > 0 {
            result.push(topbtml.clone());
        }
        result.push(topbtml.clone());
        result.push(topbtml.clone());

        return result;
    }
}

#[derive(Debug)]
enum Entity {
    Number(NumberData),
    Symbol(SymbolData)
}


fn parse_line_numbers(line : &str) -> Vec<NumberData> {
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
    return nums;
}

fn parse_line_symbols(line : &str) -> Vec<SymbolData> {
    return line.char_indices()
        .filter(|c| c.1 == '*')
        .map(|c| SymbolData{start : c.0})
       .collect();
}

fn read_input() -> Vec<String> {
    let x = fs::read_to_string("input").expect("No input");
    let lines:Vec<_> = x.lines().map(|x| x.to_string()).collect();

    return lines;
}

