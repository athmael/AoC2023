use std::iter::zip;
use std::{fs, iter, str::FromStr};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn main() {
    let input = read_input();

    let times = extract_numbers(input.get(0).unwrap());
    let distances = extract_numbers(input.get(1).unwrap());

    let mut prod = 1;

    for (t, d) in zip(times, distances) {
        prod *= find_options(t, d);
    }

    dbg!(prod);

}

fn find_options(tmax : u32, d : u32) -> u32 {
    return (1..tmax).filter(|t| t * (tmax - t) > d).count() as u32;
}

fn extract_numbers(line : &str) -> Vec<u32> {
    let mut ls = line.split_ascii_whitespace();
    return ls.filter_map(|s| s.parse::<u32>().ok()).collect_vec();
}

fn read_input() -> Vec<String> {
    let x = fs::read_to_string("input").expect("No input");
    let lines:Vec<_> = x.lines().map(|x| x.to_string()).collect();

    return lines;
}
