pub fn run() {
    let lines = include_str!("input");
    println!("Part 1!");
    part_1(lines);
    println!("Part 2!");
    part_2(lines);
}

fn calc_number(line: &str) -> u32 {
    let mut num_chars: Vec<char> = Vec::new();
    let char_vec: Vec<char> = line.chars().collect();

    for value in char_vec {
        if value.is_numeric() {
            num_chars.push(value);
        }
    }

    let first_num: &char = &num_chars[0];
    let last_num: &char = &num_chars[num_chars.len()-1];
    let mut s: String = String::new();
    s.push(*first_num);
    s.push(*last_num);
    let number: u32 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    number
}

fn part_1(lines: &str) {
    let mut sum_total: u32 = 0;

    for line in lines.lines() {
        sum_total += calc_number(&line);
    }

    println!("Sum of calibration values: {}", sum_total);
}

fn part_2(lines: &str) {
    let mut sum_total: u32 = 0;

    for line in lines.lines() {
        // Replace string rep of numbers to one that also has the number
        // I had to see how others did this tho :(
        let line = line.replace("one", "o1e");
		let line = line.replace("two", "t2o");
		let line = line.replace("three", "t3e");
		let line = line.replace("four", "f4r");
		let line = line.replace("five", "f5e");
		let line = line.replace("six", "s6x");
		let line = line.replace("seven", "s7n");
		let line = line.replace("eight", "e8t");
		let line = line.replace("nine", "n9e");
		let line = line.replace("zero", "z0o");

        sum_total += calc_number(&line);
    }

    println!("Sum of calibration values: {}", sum_total);
}