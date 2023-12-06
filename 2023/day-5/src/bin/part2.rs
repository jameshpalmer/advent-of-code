use core::f64::INFINITY;
use std::fs;

#[derive(Clone, Debug)]
struct MapElement {
    destination_start: i64,
    source_start: i64,
    range_len: i64,
}

impl MapElement {
    fn new(destination_start: i64, source_start: i64, range_len: i64) -> MapElement {
        MapElement {
            destination_start,
            source_start,
            range_len,
        }
    }
}

#[derive(Clone, Copy, Debug)]

struct Range {
    start: i64,
    range_len: i64,
}

struct Map {
    elements: Vec<MapElement>,
}

impl Map {
    fn from_str(map_str: &str) -> Map {
        let elements = map_str
            .split("\n")
            .skip(1)
            .map(|mapping| {
                let map_element = mapping
                    .split_whitespace()
                    .map(|elem_str: &str| elem_str.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                return MapElement::new(map_element[0], map_element[1], map_element[2]);
            })
            .collect::<Vec<_>>();
        return Map { elements };
    }

    fn get(&self, n: i64) -> i64 {
        for map_element in &self.elements {
            if n >= map_element.source_start && n < map_element.source_start + map_element.range_len
            {
                return map_element.destination_start + (n - map_element.source_start);
            }
        }
        return n;
    }

    fn get_range(&self, source: Range) -> Vec<(Range, Range)> {
        // We need to split the source range into any partitions that are
        // contiguous to map elements. For example, if we have a map element
        // that maps 0..10 to 20..30, and we have a source range of 5..25,
        // we need to split the source range into 5..10, 10..20, and 20..25.
        let mut partitions: Vec<Range> = Vec::new();

        let mut sorted_map_elements = self.elements.clone();
        sorted_map_elements.sort_by_key(|elem| elem.source_start);

        let mut current_start = source.start;

        if let Some(first_element) = sorted_map_elements.first() {
            if first_element.source_start > source.start {
                partitions.push(Range {
                    start: source.start,
                    range_len: if source.range_len > first_element.source_start - source.start {
                        first_element.source_start - source.start
                    } else {
                        source.range_len
                    },
                });
                current_start = first_element.source_start;
            }
        }

        for map_element in sorted_map_elements {
            if map_element.source_start + map_element.range_len <= current_start {
                continue;
            }
            if map_element.source_start >= source.start + source.range_len {
                break;
            }

            let partition_end = std::cmp::min(
                map_element.source_start + map_element.range_len - 1,
                source.start + source.range_len - 1,
            );

            partitions.push(Range {
                start: current_start,
                range_len: partition_end - current_start + 1,
            });

            current_start = partition_end + 1;

            if current_start >= source.start + source.range_len {
                break;
            }
        }

        if current_start < source.start + source.range_len {
            partitions.push(Range {
                start: current_start,
                range_len: source.start + source.range_len - current_start,
            });
        }

        let result: Vec<(Range, Range)> = partitions
            .into_iter()
            .map(|range| {
                (
                    range,
                    Range {
                        start: self.get(range.start),
                        range_len: range.range_len,
                    },
                )
            })
            .collect();

        return result;
    }
}

struct Almanac {
    seed_ranges: Vec<(i64, i64)>,
    maps: Vec<Map>,
}

impl Almanac {
    fn from_str(text: String) -> Almanac {
        let mut sections = text.split("\n\n");

        let seed_ranges = sections
            .next()
            .expect("Unable to parse seeds")
            .split_whitespace()
            .filter(|seed_str| seed_str.chars().all(|c: char| c.is_numeric()))
            .map(|seed_str| {
                return seed_str.parse().expect(seed_str);
            })
            .collect::<Vec<i64>>()
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        let mut maps = Vec::with_capacity(7);
        maps.resize_with(7, || Map::from_str(sections.next().unwrap_or_default()));

        return Almanac { seed_ranges, maps };
    }

    fn get_location_ranges(&self, seed_range: Range) -> Vec<Range> {
        let mut current_ranges = vec![seed_range];
        for map in &self.maps {
            let mut next_ranges = Vec::new();
            for range in current_ranges {
                let map_partitions = map.get_range(range);
                for (_source_range, destination_range) in map_partitions {
                    next_ranges.push(destination_range);
                }
            }
            current_ranges = next_ranges;
        }

        return current_ranges;
    }

    fn get_minimum_location(&self) -> i64 {
        let mut lowest_location = INFINITY as i64;
        for (start, range_len) in &self.seed_ranges {
            let location = self.get_location_ranges(Range {
                start: *start,
                range_len: *range_len,
            });
            let min_location = location
                .iter()
                .min_by_key(|range| range.start)
                .unwrap()
                .start;
            if min_location < lowest_location {
                lowest_location = min_location;
            }
        }
        return lowest_location;
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let almanac = Almanac::from_str(input);

    println!("{:?}", almanac.get_minimum_location());
}
