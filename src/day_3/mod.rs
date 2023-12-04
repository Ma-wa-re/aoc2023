

pub fn run() {
    let mut input_lines: Vec<Vec<char>> = Vec::new();

    for line in include_str!("input").lines() {
        input_lines.push(line.trim().chars().collect())
    };

    let (symbol_locations, number_locations) = parse_input(&input_lines);

    println!("Part 1!");
    part_1(&symbol_locations, &number_locations);

    println!("Part 2!");
    part_2(&symbol_locations, &number_locations);
}

#[derive(Debug)]
struct SymbolLocation(char, usize, usize);

#[derive(Debug)]
struct NumberLocation{
    number: usize,
    line: usize,
    start_location: usize,
    end_location: usize,
}

fn parse_input(input_lines: &Vec<Vec<char>>) -> (Vec<SymbolLocation>, Vec<NumberLocation>) {
    // let char_vec: Vec<char> = vec!['!','@','#','$','%','^','&','*','+','-','/','=', '_'];
    // Get symbol locations
    let mut symbol_locations: Vec<SymbolLocation> = Vec::new();

    for input_line in input_lines.iter().enumerate() {
        for input_char in input_line.1.iter().enumerate() {
            if input_char.1.is_ascii_punctuation() && *input_char.1 != '.' && *input_char.1 != '\\'{
                symbol_locations.push(SymbolLocation(*input_char.1, input_line.0, input_char.0));
            }
        }
    }

    // Get number locations
    let mut number_locations: Vec<NumberLocation> = Vec::new();

    let mut number_string: String = String::new();
    let mut start_loc: usize = 1000;
    for input_line in input_lines.iter().enumerate() {
        for input_char in input_line.1.iter().enumerate() {
            if input_char.1.is_numeric() {
                if start_loc == 1000 {
                    start_loc = input_char.0
                }
                number_string.push(*input_char.1);
            }
            else {
                if !number_string.is_empty() {
                    number_locations.push(
                        NumberLocation { number: number_string.trim().parse().unwrap(), line: input_line.0, start_location: start_loc, end_location: input_char.0-1 }
                    );
                    number_string = String::new();
                    start_loc = 1000;
                }
            }
        }
        // If we get to the end of the line check if we have a number and if so push it then reset 
        if !number_string.is_empty() {
            number_locations.push(
                NumberLocation { number: number_string.trim().parse().unwrap(), line: input_line.0, start_location: start_loc, end_location: input_line.1.len()-1 }
            );
        }
        number_string = String::new();  
        start_loc = 1000;
    }

    return (symbol_locations, number_locations)
}

fn part_1(symbol_locations: &Vec<SymbolLocation>, number_locations: &Vec<NumberLocation>) {
    let mut part_sum: usize = 0;
    
    for symbol in symbol_locations {
        
        let line_start_num: usize = if symbol.1 == 0 {symbol.1} else {symbol.1-1};
        let char_start_num: usize = if symbol.2 == 0 {symbol.2} else {symbol.2-1};
        
        for number in number_locations {
            let line_range: Vec<usize> = (line_start_num..(symbol.1+2)).collect();

            let in_line_range = line_range.contains(&number.line);
            
            let num_char_range: Vec<usize> = (number.start_location..number.end_location+1).collect();

            let mut in_char_range: bool = false;
            
            for val in num_char_range {
                if (char_start_num..symbol.2+2).contains(&val) {
                    in_char_range = true;
                    break;
                }
            }

            if in_line_range && in_char_range {
                //println!("Part number: {} from {}", number.number, symbol.0);
                part_sum += number.number;
            }
        }
    }

    println!("Part number sum: {}", part_sum);
}

fn part_2(symbol_locations: &Vec<SymbolLocation>, number_locations: &Vec<NumberLocation>) {
    let mut gear_sum: usize = 0;

    for symbol in symbol_locations {

        if &symbol.0 != &'*' {
            continue;
        }

        let mut gears: Vec<usize> = Vec::new();
        
        let line_start_num: usize = if symbol.1 == 0 {symbol.1} else {symbol.1-1};
        let char_start_num: usize = if symbol.2 == 0 {symbol.2} else {symbol.2-1};
        
        for number in number_locations {
            let line_range: Vec<usize> = (line_start_num..(symbol.1+2)).collect();

            let in_line_range = line_range.contains(&number.line);
            
            let num_char_range: Vec<usize> = (number.start_location..number.end_location+1).collect();

            let mut in_char_range: bool = false;
            
            for val in num_char_range {
                if (char_start_num..symbol.2+2).contains(&val) {
                    in_char_range = true;
                    break;
                }
            }

            if in_line_range && in_char_range {
                gears.push(number.number)
            }
        }

        if gears.len() == 2 {
            gear_sum += gears[0] * gears[1];
        }
    }

    println!("Gear ratios sum: {}", gear_sum);
}