use std::{f32::INFINITY, fs};

struct Map {
    data: Vec<Vec<i64>>,
}

impl Map {
    fn from_str(map_str: &str) -> Map {
        let data = map_str
            .split("\n")
            .skip(1)
            .map(|mapping| {
                mapping
                    .split_whitespace()
                    .map(|elem_str: &str| elem_str.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<_>>();
        return Map { data };
    }

    fn get(&self, n: i64) -> i64 {
        for map_element in &self.data {
            let (destination_start, source_start, range_len) =
                (map_element[0], map_element[1], map_element[2]);

            if n >= source_start && n < source_start + range_len {
                return destination_start + (n - source_start);
            }
        }
        return n;
    }
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn from_str(text: String) -> Almanac {
        let mut sections = text.split("\n\n");

        let seeds = sections
            .next()
            .expect("Unable to parse seeds")
            .split_whitespace()
            .filter(|seed_str| seed_str.chars().all(|c: char| c.is_numeric()))
            .map(|seed_str| {
                return seed_str.parse().expect(seed_str);
            })
            .collect::<Vec<i64>>();

        let mut maps = Vec::with_capacity(7);
        maps.resize_with(7, || Map::from_str(sections.next().unwrap_or_default()));

        return Almanac { seeds, maps };
    }

    fn get_location(&self, seed: i64) -> i64 {
        self.maps.iter().fold(seed, |acc: i64, map| map.get(acc))
    }

    fn get_lowest_location(&self) -> i64 {
        let mut lowest_location = INFINITY as i64;
        for seed in &self.seeds {
            let location = self.get_location(*seed);
            if (location) < lowest_location {
                lowest_location = location;
            }
        }
        return lowest_location;
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let almanac = Almanac::from_str(input);

    println!("{}", almanac.get_lowest_location());
}
