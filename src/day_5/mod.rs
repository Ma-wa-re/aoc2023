pub fn run() {
    // Parse input
    parse_input()
}

fn parse_input() {
    let input_lines: Vec<&str> = include_str!("testinput").lines().collect();

    let seeds: Vec<&str> = input_lines[0]
        .strip_prefix("seeds:")
        .unwrap()
        .trim()
        .split(" ")
        .collect();

    println!("{:?}", seeds);

    let mut soil: bool = false;
    let mut fertilizer: bool = false;
    let mut water: bool = false;
    let mut light: bool = false;
    let mut temperature: bool = false;
    let mut humidity: bool = false;
    let mut location: bool = false;

    for line in input_lines[2..].into_iter() {
        if line.is_empty() {
            soil = false;
            fertilizer = false;
            water = false;
            light = false;
            temperature = false;
            humidity = false;
            location = false;
            continue;
        }

        if *line == "seed-to-soil map:" {
            soil = true;
            continue;
        } else if *line == "soil-to-fertilizer map" {
            fertilizer = true;
            continue;
        } else if *line == "fertilizer-to-water map:" {
            water = true;
            continue;
        } else if *line == "water-to-light map:" {
            light = true;
            continue;
        } else if *line == "light-to-temperature map:" {
            temperature = true;
            continue;
        } else if *line == "temperature-to-humidity map:" {
            humidity = true;
            continue;
        } else if *line == "humidity-to-location map:" {
            location = true;
            continue;
        }

        if soil {
            println!("Soil: {}", line);
        } else if fertilizer {
            println!("Fertilizer: {}", line);
        } else if water {
            println!("Water: {}", line);
        } else if light {
            println!("Light: {}", line);
        } else if temperature {
            println!("Temperature: {}", line);
        } else if humidity {
            println!("Humidity: {}", line);
        } else if location {
            println!("Location: {}", line);
        }
    }
}