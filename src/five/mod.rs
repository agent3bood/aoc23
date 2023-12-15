use std::time::Instant;

pub struct Sol {
    seeds: Option<Vec<u64>>,
    seeds_ranges: Option<Vec<(u64, u64)>>,
    pub seed_soil: Vec<(u64, u64, u64)>,
    soil_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_water: Vec<(u64, u64, u64)>,
    water_light: Vec<(u64, u64, u64)>,
    light_temperature: Vec<(u64, u64, u64)>,
    temperature_humidity: Vec<(u64, u64, u64)>,
    humidity_location: Vec<(u64, u64, u64)>,
}

impl Sol {
    pub fn new() -> Sol {
        Sol {
            seeds: None,
            seeds_ranges: None,
            seed_soil: Default::default(),
            soil_fertilizer: Default::default(),
            fertilizer_water: Default::default(),
            water_light: Default::default(),
            light_temperature: Default::default(),
            temperature_humidity: Default::default(),
            humidity_location: Default::default(),
        }
    }

    fn find_for_val(r: &Vec<(u64, u64, u64)>, val: u64) -> u64 {
        for e in r.iter() {
            if (val >= e.1) && (val < e.1 + e.2) {
                return (e.0 + e.2) - ((e.1 + e.2) - val);
            }
        }
        val
    }

    /// ```
    /// use advent23::five::*;
    /// let mut sol = Sol::new();
    /// sol.seed_soil =vec![(50, 98, 2), (52, 50, 48)];
    /// assert_eq!(sol.find_soil(79), 81);
    /// ```
    pub fn find_soil(&self, seed: u64) -> u64 {
        Sol::find_for_val(&self.seed_soil, seed)
    }

    fn find_fertilizer(&self, soil: u64) -> u64 {
        Sol::find_for_val(&self.soil_fertilizer, soil)
    }

    fn find_water(&self, fertilizer: u64) -> u64 {
        Sol::find_for_val(&self.fertilizer_water, fertilizer)
    }

    fn find_light(&self, water: u64) -> u64 {
        Sol::find_for_val(&self.water_light, water)
    }

    fn find_temperature(&self, light: u64) -> u64 {
        Sol::find_for_val(&self.light_temperature, light)
    }

    fn find_humidity(&self, temperature: u64) -> u64 {
        Sol::find_for_val(&self.temperature_humidity, temperature)
    }

    fn find_location(&self, humidity: u64) -> u64 {
        Sol::find_for_val(&self.humidity_location, humidity)
    }
}

fn process(sol: &Sol) -> u64 {
    let mut ans = u64::MAX;
    match (&sol.seeds, &sol.seeds_ranges) {
        (Some(seeds), None) => {
            for seed in seeds.iter() {
                let soil = sol.find_soil(*seed);
                let fertilizer = sol.find_fertilizer(soil);
                let water = sol.find_water(fertilizer);
                let light = sol.find_light(water);
                let temperature = sol.find_temperature(light);
                let humidity = sol.find_humidity(temperature);
                let location = sol.find_location(humidity);
                if location < ans {
                    ans = location;
                }
            }
        }
        (None, Some(seeds)) => {
            let mut c = 0;
            for e in seeds {
                c += 1;
                println!("Processing section {} of {}", c, seeds.len());
                let start = e.0;
                let range = e.1;
                for seed in start..start + range {
                    let soil = sol.find_soil(seed);
                    let fertilizer = sol.find_fertilizer(soil);
                    let water = sol.find_water(fertilizer);
                    let light = sol.find_light(water);
                    let temperature = sol.find_temperature(light);
                    let humidity = sol.find_humidity(temperature);
                    let location = sol.find_location(humidity);
                    if location < ans {
                        ans = location;
                    }
                }
            }
        }
        _ => panic!(),
    }

    ans
}

fn build_sol_seeds(input: &str, sol: &mut Sol) {
    sol.seeds = Some(vec![]);
    let seeds = input.lines().next().unwrap().replace("seeds:", "");
    for seed in seeds.split_whitespace() {
        sol.seeds
            .as_mut()
            .unwrap()
            .push(seed.parse::<u64>().unwrap());
    }
}

fn build_sol_seeds_part2(input: &str, sol: &mut Sol) {
    println!("build_sol_seeds_part2");
    let mut start = None;
    let seeds = input.lines().next().unwrap().replace("seeds:", "");
    sol.seeds_ranges = Some(vec![]);

    for n in seeds.split_whitespace() {
        let n = n.parse::<u64>().unwrap();
        match start {
            None => start = Some(n),
            Some(s) => {
                sol.seeds_ranges.as_mut().unwrap().push((s, n));
                start = None;
            }
        }
    }
}

fn build_sol_sections(input: &str, sol: &mut Sol) {
    println!("build_sol_sections");
    let mut section = Section::Seeds;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line == "seed-to-soil map:" {
            section = Section::SeedSoil;
            continue;
        }
        if line == "soil-to-fertilizer map:" {
            section = Section::SoilFertilizer;
            continue;
        }
        if line == "fertilizer-to-water map:" {
            section = Section::FertilizerWater;
            continue;
        }
        if line == "water-to-light map:" {
            section = Section::WaterLight;
            continue;
        }
        if line == "light-to-temperature map:" {
            section = Section::LightTemperature;
            continue;
        }
        if line == "temperature-to-humidity map:" {
            section = Section::TemperatureHumidity;
            continue;
        }
        if line == "humidity-to-location map:" {
            section = Section::HumidityLocation;
            continue;
        }
        //
        // !!
        //
        if section == Section::SeedSoil {
            let mut parts = line.split_whitespace();
            let e1_start = parts.next().unwrap().parse::<u64>().unwrap();
            let e2_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();
            sol.seed_soil.push((e1_start, e2_start, range));
            continue;
        }
        if section == Section::SoilFertilizer {
            let mut parts = line.split_whitespace();
            let e1_start = parts.next().unwrap().parse::<u64>().unwrap();
            let e2_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();
            sol.soil_fertilizer.push((e1_start, e2_start, range));
        }
        if section == Section::FertilizerWater {
            let mut parts = line.split_whitespace();
            let e1_start = parts.next().unwrap().parse::<u64>().unwrap();
            let e2_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();
            sol.fertilizer_water.push((e1_start, e2_start, range));
            continue;
        }
        if section == Section::WaterLight {
            let mut parts = line.split_whitespace();
            let e1_start = parts.next().unwrap().parse::<u64>().unwrap();
            let e2_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();
            sol.water_light.push((e1_start, e2_start, range));
            continue;
        }
        if section == Section::LightTemperature {
            let mut parts = line.split_whitespace();
            let e1_start = parts.next().unwrap().parse::<u64>().unwrap();
            let e2_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();
            sol.light_temperature.push((e1_start, e2_start, range));
            continue;
        }
        if section == Section::TemperatureHumidity {
            let mut parts = line.split_whitespace();
            let e1_start = parts.next().unwrap().parse::<u64>().unwrap();
            let e2_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();
            sol.temperature_humidity.push((e1_start, e2_start, range));
            continue;
        }
        if section == Section::HumidityLocation {
            let mut parts = line.split_whitespace();
            let e1_start = parts.next().unwrap().parse::<u64>().unwrap();
            let e2_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range = parts.next().unwrap().parse::<u64>().unwrap();
            sol.humidity_location.push((e1_start, e2_start, range));
            continue;
        }
    }
}

#[derive(PartialOrd, PartialEq)]
enum Section {
    Seeds,
    SeedSoil,
    SoilFertilizer,
    FertilizerWater,
    WaterLight,
    LightTemperature,
    TemperatureHumidity,
    HumidityLocation,
}

pub fn part1() {
    let input = include_str!("input1.txt");
    let mut sol = Sol::new();
    build_sol_seeds(input, &mut sol);
    build_sol_sections(input, &mut sol);
    println!("{}", process(&sol));
}

pub fn part2() {
    //
    let input = include_str!("input1.txt");
    let mut sol = Sol::new();
    let mut timer = Instant::now();
    build_sol_seeds_part2(input, &mut sol);
    println!("build_sol_seeds_part2 completed in {:?}", timer.elapsed());

    timer = Instant::now();
    build_sol_sections(input, &mut sol);
    println!("build_sol_sections completed in {:?}", timer.elapsed());

    timer = Instant::now();
    println!("{}", process(&sol));
    println!("process completed in {:?}", timer.elapsed());
}

mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let mut sol = Sol::new();
        build_sol_seeds(input, &mut sol);
        build_sol_sections(input, &mut sol);
        assert_eq!(process(&sol), 35);
    }

    #[test]
    fn part2_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let mut sol = Sol::new();
        build_sol_seeds_part2(input, &mut sol);
        build_sol_sections(input, &mut sol);
        assert_eq!(process(&sol), 46);
    }
}
