use std::{fs, iter, str::FromStr};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn main() {
    let input = read_input();

    let mut g = Game::new();
    g.parse(input.iter());

    let locations = g.seeds.iter().map(|s| g.map(*s)).collect_vec();
    dbg!(&locations);

    dbg!(locations.iter().min().unwrap());
}



#[derive(Debug)]
    struct Range {
    a : u64,
    b : u64,
    c : u64
}

impl Range {
    fn map(&self, n : u64) -> Option<u64> {
        if n >= self.b && n - self.b < self.c {
            return Some(self.a + n - self.b);
        }
        return None;
    }
}

struct Game {
    seeds : Vec<u64>,
    seed_to_soil : Vec<Range>,
    soil_to_fertilizer : Vec<Range>,
    fertilizer_to_water : Vec<Range>,
    water_to_light : Vec<Range>,
    light_to_temperature : Vec<Range>,
    temperature_to_humidity : Vec<Range>,
    humidity_to_location : Vec<Range>,

    //parse : &'a dyn FnMut(&mut Game<'a>, &str) -> ()
}




impl Game {
    fn new() -> Self {
        let mut game = Game {
            seeds : vec![],
            seed_to_soil : vec![],
            soil_to_fertilizer : vec![],
            fertilizer_to_water : vec![],
            water_to_light : vec![],
            light_to_temperature : vec![],
            temperature_to_humidity : vec![],
            humidity_to_location : vec![],

            //parse : &|_g, _s| {}
        };
        //game.parse = &Game::parse_seeds;

        return game;
    }

    fn parse<'a, I>(&mut self, mut lines : I)
    where
        I : Iterator<Item=&'a String>
    {
        lines = self.parse_seeds(lines);
        lines = Self::parse_map(lines, &mut self.seed_to_soil);
        lines = Self::parse_map(lines, &mut self.soil_to_fertilizer);
        lines = Self::parse_map(lines, &mut self.fertilizer_to_water);
        lines = Self::parse_map(lines, &mut self.water_to_light);
        lines = Self::parse_map(lines, &mut self.light_to_temperature);
        lines = Self::parse_map(lines, &mut self.temperature_to_humidity);
        Self::parse_map(lines, &mut self.humidity_to_location);
    }

    fn parse_seeds<'a, I>(&mut self, mut lines : I) -> I
    where
        I : Iterator<Item=&'a String>
    {
        while let Some(l) = lines.next() {
            if l.is_empty() {
                return lines;
            }

            let mut seeds = l.split_ascii_whitespace();
            seeds.next();

            while let Some(seed) = seeds.next() {
                self.seeds.push(seed.parse::<u64>().unwrap());
            }
        }

        return lines;
    }

    fn parse_map<'a, I>(mut lines : I, map : &mut Vec<Range>) -> I
    where
        I : Iterator<Item=&'a String>
    {
        while let Some(l) = lines.next() {
            if l.is_empty() {
                return lines;
            }

            if l.contains("map") { continue; }
            map.push(Game::parse_range(l));
        }

        return lines;
    }

    fn parse_range(s : &String) -> Range {
        let rngprts = s.split_ascii_whitespace();

        if let Some((a,b,c)) = rngprts.map(|s| s.parse::<u64>().unwrap()).collect_tuple() {
            return Range{a,b,c};
        }
        panic!();
    }

    fn map_single(value : u64, recipe : &Vec<Range>) -> u64 {
        if let Some(n) = recipe.iter().filter_map(|r| r.map(value)).reduce(|v, a| v) {
            print!("{} -> {}", value, n);
            return n;
        }
        else {
            print!("{} -> {}", value, value);
            return value;
        }
    }

    fn map(&self, value : u64) -> u64 {
        let mut iv = Self::map_single(value, &self.seed_to_soil);
        iv = Self::map_single(iv, &self.soil_to_fertilizer);
        iv = Self::map_single(iv, &self.fertilizer_to_water);
        iv = Self::map_single(iv, &self.water_to_light);
        iv = Self::map_single(iv, &self.light_to_temperature);
        iv = Self::map_single(iv, &self.temperature_to_humidity);
        iv = Self::map_single(iv, &self.humidity_to_location);

        println!("\n-------");

        return iv;
    }


    // fn feed(line : &str) {
    // }

    // fn parse_seeds(&mut self, s : &str) -> () {

    //     self.parse = &Game::parse_seeds;
    // }

    // fn make_map_parser_fn(&'a mut self, map_no : usize) -> &'a dyn FnMut(&'a mut Game, &str) {
    //     let range : &Vec<Range> = self.seed_to_soil.as_ref();

    //     //return Game::parse_seeds;

    //     return &|g : &mut Game, _l : &str| {
    //         g.parse = &|g : &mut Game, l : &str| {
    //             if l.is_empty() == false {
    //                 range.push(Range { a: 0, b: 0, c: 0 });
    //                 return Game::parse_map(g, l);
    //             }
    //             else {
    //                 return Game::parse_map(g, l);
    //             }
    //         }
    //     };
    // }

    // fn parse_map(&mut self, s : &str) -> () {
    // }

}





fn read_input() -> Vec<String> {
    let x = fs::read_to_string("input").expect("No input");
    let lines:Vec<_> = x.lines().map(|x| x.to_string()).collect();

    return lines;
}
