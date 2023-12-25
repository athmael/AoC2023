use std::{fs, iter, str::FromStr};
use std::collections::{HashSet, HashMap, vec_deque, VecDeque};
use itertools::Itertools;

fn main() {
    let input = read_input();

    let mut g = Game::new();
    g.parse(input.iter());

    let locations = g.map(&g.seeds);

    let minloc = locations.iter().map(|r| r.0).min().unwrap();
    dbg!(minloc);
    //let locations = g.seeds.iter().map(|s| g.map(*s)).collect_vec();
    //dbg!(&locations);

    //dbg!(locations.iter().min().unwrap());
}



#[derive(Debug)]
    struct Range {
    a : u64,    //dst
    b : u64,    //src
    c : u64     //len
}

impl Range {
    fn map(&self, irange : (u64, u64)) -> Option<((u64, u64), Vec<(u64, u64)>)> {
        if irange.0 + irange.1 > self.b && irange.0 < self.b + self.c {
            if irange.0 >= self.b && irange.0 + irange.1 <= self.b + self.c {
                // Fully contained
                return Some(((self.a + irange.0 - self.b, irange.1), vec![]));
            }
            else if irange.0 < self.b && irange.0 + irange.1 > self.b + self.c {
                // Middle
                return Some(((self.a, self.c), vec![
                    (irange.0, self.b - irange.0),
                    (self.b + self.c, irange.0 + irange.1 - (self.b + self.c))
                    ]));
            }
            else if irange.0 < self.b && irange.0 + irange.1 <= self.b + self.c {
                // Lower
                return Some(((self.a, irange.1 - (self.b - irange.0)), vec![
                    (irange.0, self.b - irange.0)
                    ]));
            }
            else {
                // Upper
                return Some(((self.a + irange.0 - self.b, self.b + self.c - irange.0), vec![
                    (self.b + self.c, irange.0 + irange.1 - (self.b + self.c))
                    ]));
            }
        }
        return None;
    }
}


#[test]
fn test_Range_map() {
    let sut = Range {a:100, b:10, c:20};

    if let Some(r1) = sut.map((10,20)) {
        assert_eq!(r1.0.0, 100);
        assert_eq!(r1.0.1, 20);

        assert!(r1.1.is_empty());
    }
    else {
        assert!(false);
    }

    if let Some(r1) = sut.map((5,30)) {
        assert_eq!(r1.0.0, 100);
        assert_eq!(r1.0.1, 20);

        assert_eq!(r1.1.len(), 2);
        assert_eq!(r1.1.get(0), Some(&(5u64,5u64)));
        assert_eq!(r1.1.get(1), Some(&(30u64,5u64)));
    }
    else {
        assert!(false);
    }

    if let Some(r1) = sut.map((5,20)) {
        assert_eq!(r1.0.0, 100);
        assert_eq!(r1.0.1, 15);

        assert_eq!(r1.1.len(), 1);
        assert_eq!(r1.1.get(0), Some(&(5u64,5u64)));
    }
    else {
        assert!(false);
    }

    if let Some(r1) = sut.map((15,20)) {
        assert_eq!(r1.0.0, 105);
        assert_eq!(r1.0.1, 15);

        assert_eq!(r1.1.len(), 1);
        assert_eq!(r1.1.get(0), Some(&(30u64,5u64)));
    }
    else {
        assert!(false);
    }

    if let Some(r1) = sut.map((5,5)) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Some(r1) = sut.map((30,5)) {
        assert!(false);
    }
    else {
        assert!(true);
    }
}



struct Game {
    seeds : Vec<(u64,u64)>,
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
        let mut n = vec![];

        while let Some(l) = lines.next() {
            if l.is_empty() {
                return lines;
            }

            let mut seeds = l.split_ascii_whitespace();
            seeds.next();

            while let Some(seed) = seeds.next() {
                n.push(seed.parse::<u64>().unwrap());

                if n.len() == 2 {
                    self.seeds.push((*n.get(0).unwrap(), *n.get(1).unwrap()));
                    n.clear();
                }
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

    fn map_range(value : &Vec<(u64, u64)>, recipe : &Vec<Range>) -> Vec<(u64,u64)> {
        let mut inr : VecDeque<(u64, u64)> = VecDeque::from(value.to_owned());

        let mut our = vec![];

        'outer: loop {
            if inr.is_empty() { break; }

            let curr = inr.pop_front().unwrap();

            for r in recipe {
                if let Some(m) = r.map(curr) {
                    our.push(m.0);
                    inr.extend(m.1);
                    continue 'outer;
                }
            }
            our.push(curr);
        }
        return our;
    }

    fn map(&self, ranges : &Vec<(u64, u64)>) -> Vec<(u64,u64)> {
        let mut iv = Self::map_range(ranges, &self.seed_to_soil);
        iv = Self::map_range(&iv, &self.soil_to_fertilizer);
        iv = Self::map_range(&iv, &self.fertilizer_to_water);
        iv = Self::map_range(&iv, &self.water_to_light);
        iv = Self::map_range(&iv, &self.light_to_temperature);
        iv = Self::map_range(&iv, &self.temperature_to_humidity);
        iv = Self::map_range(&iv, &self.humidity_to_location);

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
