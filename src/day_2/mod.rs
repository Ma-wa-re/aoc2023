use regex::Regex;

const PART1_RULES: (u32, u32, u32) = (12, 13, 14);

pub fn run() {
    let input_lines: Vec<&str>  = include_str!("input").lines().collect();
    println!("Part 1!");
    part_1(&input_lines);
    println!("Part 2!");
    part_2(&input_lines);
}

fn part_1(input_lines: &Vec<&str>) {
    let mut possible_sum: u32 = 0;

    // Regex for getting game number and cube numbers
    let game_re = Regex::new(r"Game (?<game>\d+):.*").unwrap();
    let red_re = Regex::new(r"(?<red>\d+) red").unwrap();
    let green_re = Regex::new(r"(?<green>\d+) green").unwrap();
    let blue_re = Regex::new(r"(?<blue>\d+) blue").unwrap();
    
    for l in input_lines {
        
        // Get a game number from the regex match, it shouldn't be missing but if it is we set it to 0
        let caps = game_re.captures(l).unwrap();
        let game_number: u32 = match caps.name("game") {
            Some(val) => val.as_str().parse().unwrap(),
            None => 0,
        };

        let mut impossible: bool = false;

        // Split string by ; for each set
        for sl in l.split(";") {
            // Get red number
            let red_number: u32 = match red_re.captures(sl) {
                Some(caps) => {
                    match caps.name("red") {
                        Some(val) => val.as_str().parse().unwrap(), // We know the string is a number due to the regex so we can unwrap from the Ok with no worries of an Err
                        None => 0,
                    }
                },
                None => 0,
            };

            // Get green number
            let green_number: u32 = match green_re.captures(sl) {
                Some(caps) => {
                    match caps.name("green") {
                        Some(val) => val.as_str().parse().unwrap(), // We know the string is a number due to the regex so we can unwrap from the Ok with no worries of an Err
                        None => 0,
                    }
                },
                None => 0,
            };

            // Get blue number
            let blue_number: u32 = match blue_re.captures(sl) {
                Some(caps) => {
                    match caps.name("blue") {
                        Some(val) => val.as_str().parse().unwrap(), // We know the string is a number due to the regex so we can unwrap from the Ok with no worries of an Err
                        None => 0,
                    }
                },
                None => 0,
            };

            // Check if any in the set break the game rules
            if red_number > PART1_RULES.0 || green_number > PART1_RULES.1 || blue_number > PART1_RULES.2 {
                impossible = true;
                break;
            }
        }

        if !impossible {
            possible_sum += game_number;
        }
    }

    println!("Possible game sum: {}", possible_sum);
    
}

fn part_2(input_lines: &Vec<&str>) {
    let mut power_sum: u32 = 0;

    // Regex for getting cube numbers
    let red_re = Regex::new(r"(?<red>\d+) red").unwrap();
    let green_re = Regex::new(r"(?<green>\d+) green").unwrap();
    let blue_re = Regex::new(r"(?<blue>\d+) blue").unwrap();

    for l in input_lines {

        // Set starting vals for min number of each cube
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        
        for sl in l.split(";") {
            let red_number: u32 = match red_re.captures(sl) {
                Some(caps) => {
                    match caps.name("red") {
                        Some(val) => val.as_str().parse().unwrap(),
                        None => 0,
                    }
                },
                None => 0,
            };

            if red_number > min_red { min_red = red_number; }

            let green_number: u32 = match green_re.captures(sl) {
                Some(caps) => {
                    match caps.name("green") {
                        Some(val) => val.as_str().parse().unwrap(),
                        None => 0,
                    }
                },
                None => 0,
            };

            if green_number > min_green { min_green = green_number; }

            let blue_number: u32 = match blue_re.captures(sl) {
                Some(caps) => {
                    match caps.name("blue") {
                        Some(val) => val.as_str().parse().unwrap(),
                        None => 0,
                    }
                },
                None => 0,
            };

            if blue_number > min_blue { min_blue = blue_number; }
        }

        let min_total = min_red * min_green * min_blue;
        power_sum += min_total;

    }

    println!("Possible min game power sum: {}", power_sum);
}