use std::{fs, iter, str::FromStr};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn main() {
    let input = read_input();

    let mut extraCards = HashMap::<usize, u64>::new();

    let x : u64 = input.iter().enumerate().map(|l| {
        let c = Card::from_str(l.1).unwrap();
        let x = match extraCards.get(&l.0) {
            Some(n) => n + 1,
            _ => 1
        };

        let wincount = c.get_win_count();

        println!("{} has {} cards and {} wins", l.0, x, wincount);

        for i in (l.0+1)..(l.0+c.get_win_count() as usize +1)  {
            //let mut h= &extraCards;

            let nx = match extraCards.get(&i) { Some(n) => n, _ => &0}.to_owned();
            extraCards.insert(i, nx + x);

            println!(".. {} got {} new cards, {} in total", i, x, nx+x);
        }

        return x;
    }).sum();

    println!("{}", x);
}


struct Card {
    winning : Vec<u32>,
    current : Vec<u32>
}

impl Card {
    fn get_numbers(s:&str) -> Vec<u32> {
        //dbg!(s);
        return s.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect_vec();
    }

    fn get_win_count(&self) -> u32 {
        let hw : HashSet<_> = self.winning.clone().drain(..).collect();
        let hc : HashSet<_> = self.current.clone().drain(..).collect();

        let x = hw.intersection(&hc).count() as u32;
        return x;
        //dbg!(x);
        // return match x {
        //     0 => 0,
        //     _ => 1 << (x-1)
        // };

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

