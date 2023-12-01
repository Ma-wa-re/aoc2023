mod constants;

fn main() {
    println!("Part 1!");
    part_1();
}

fn part_1() {
    let mut sum_total: u32 = 0;

    let lines = constants::CODE.split("\n");

    for line in lines {
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
        sum_total += number;
    }

    println!("Sum of calibration values: {}", sum_total);
}