use std::{fs, iter, str::FromStr};
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = read_input();

    let x : u32 = input.iter().map(|l| Card::from_str(l).unwrap()).map(|c| c.get_win_count()).sum();
    println!("{}", x);
}


struct Card {
    winning : Vec<u32>,
    current : Vec<u32>
}

impl Card {
    fn get_numbers(s:&str) -> Vec<u32> {
        dbg!(s);
        return s.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect_vec();
    }

    fn get_win_count(&self) -> u32 {
        let hw : HashSet<_> = self.winning.clone().drain(..).collect();
        let hc : HashSet<_> = self.current.clone().drain(..).collect();

        let x = hw.intersection(&hc).count() as u32;
        dbg!(x);
        return match x {
            0 => 0,
            _ => 1 << (x-1)
        };

    //    HashSet::<u32>::from(self.winning).intersection(HashSet::from(self.current)).count();
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let colon = s.find(':').unwrap();
        let mut lists = s[(colon+1)..].split('|');
        let mut winning = Self::get_numbers(lists.next().unwrap());
        let mut currents = Self::get_numbers(lists.next().unwrap());

        winning.sort();
        currents.sort();

        let card = Card{winning : winning, current : currents};
        return Ok(card);
    }
}


fn read_input() -> Vec<String> {
    let x = fs::read_to_string("input").expect("No input");
    let lines:Vec<_> = x.lines().map(|x| x.to_string()).collect();

    return lines;
}

