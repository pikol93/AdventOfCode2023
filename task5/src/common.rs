use std::str::FromStr;

#[derive(Debug)]
pub struct Map {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_to_soil: Vec<Map>,
    pub soil_to_fertilizer: Vec<Map>,
    pub fertilizer_to_water: Vec<Map>,
    pub water_to_light: Vec<Map>,
    pub light_to_temperature: Vec<Map>,
    pub temperature_to_humidity: Vec<Map>,
    pub humidity_to_location: Vec<Map>,
}

impl Almanac {
    pub fn map_seeds_to_location(&self) -> impl Iterator<Item=u64> + '_ {
        self.seeds
            .iter()
            .copied()
            .map(|seed| convert_using_map(seed, &self.seed_to_soil))
            .map(|soil| convert_using_map(soil, &self.soil_to_fertilizer))
            .map(|fertilizer| convert_using_map(fertilizer, &self.fertilizer_to_water))
            .map(|water| convert_using_map(water, &self.water_to_light))
            .map(|light| convert_using_map(light, &self.light_to_temperature))
            .map(|temperature| convert_using_map(temperature, &self.temperature_to_humidity))
            .map(|humidity| convert_using_map(humidity, &self.humidity_to_location))
    }
}

pub fn read_input(text: &str) -> Almanac {
    let (_, remaining) = text.split_once("seeds:").unwrap();
    let (seeds_str, remaining) = remaining.split_once("seed-to-soil map:").unwrap();
    let (seed_to_soil_str, remaining) = remaining.split_once("soil-to-fertilizer map:").unwrap();
    let (soil_to_fertilizer_str, remaining) =
        remaining.split_once("fertilizer-to-water map:").unwrap();
    let (fertilizer_to_water_str, remaining) = remaining.split_once("water-to-light map:").unwrap();
    let (water_to_light_str, remaining) =
        remaining.split_once("light-to-temperature map:").unwrap();
    let (light_to_temperature_str, remaining) = remaining
        .split_once("temperature-to-humidity map:")
        .unwrap();
    let (temperature_to_humidity_str, humidity_to_location_str) =
        remaining.split_once("humidity-to-location map:").unwrap();

    let seeds = read_seeds(seeds_str);
    let seed_to_soil = read_maps(seed_to_soil_str);
    let soil_to_fertilizer = read_maps(soil_to_fertilizer_str);
    let fertilizer_to_water = read_maps(fertilizer_to_water_str);
    let water_to_light = read_maps(water_to_light_str);
    let light_to_temperature = read_maps(light_to_temperature_str);
    let temperature_to_humidity = read_maps(temperature_to_humidity_str);
    let humidity_to_location = read_maps(humidity_to_location_str);

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn read_seeds(text: &str) -> Vec<u64> {
    text.trim()
        .split(' ')
        .filter_map(|x| u64::from_str(x).ok())
        .collect()
}

fn read_maps(text: &str) -> Vec<Map> {
    text.trim()
        .lines()
        .map(|line| {
            let mut iter = line
                .trim()
                .splitn(3, ' ')
                .map(|x| u64::from_str(x).unwrap());

            Map {
                destination_range_start: iter.next().unwrap(),
                source_range_start: iter.next().unwrap(),
                range_length: iter.next().unwrap(),
            }
        })
        .collect()
}

fn convert_using_map(value: u64, maps: &[Map]) -> u64 {
    maps.iter()
        .filter(|map| value >= map.source_range_start)
        .filter(|map| value < map.source_range_start + map.range_length)
        .map(|map| map.destination_range_start + (value - map.source_range_start))
        .next()
        .unwrap_or(value)
}
