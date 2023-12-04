use std::collections::HashSet;

pub fn run() {
    let games: Vec<Game> = parse_input();
    println!("Part 1");
    part_1(&games)
}

#[derive(Debug)]
struct Game {
    number: usize,
    winning_numbers: HashSet<usize>,
    player_numbers: HashSet<usize>
}

impl Game {
    fn points(&self) -> usize {
        
        let num_of_win: usize =self.winning_numbers.intersection(&self.player_numbers).count();

        if num_of_win > 0 {
            let mut point: usize = 1;
            for _ in 0..num_of_win-1 {
                point *= 2;
            }
            point
        } else {
            0
        }
    }
}

fn parse_input() -> Vec<Game>{
    let mut games: Vec<Game> = Vec::new();

    for game_lines in include_str!("input").lines().enumerate() {
        let (winning_str, player_str) = game_lines.1.split_once(':').unwrap().1.trim().split_once('|').unwrap();
        
        let mut winning_numbers: HashSet<usize> = HashSet::new();
        let mut player_numbers: HashSet<usize> = HashSet::new();

        for number in winning_str.trim().split(' ') {
            if !number.is_empty() {
                winning_numbers.insert(number.parse::<usize>().unwrap());
            }
        }

        for number in player_str.trim().split(' ') {
            if !number.is_empty() {
                player_numbers.insert(number.parse::<usize>().unwrap());
            }
        }
        
        games.push(Game { 
            number: game_lines.0+1,
            winning_numbers,
            player_numbers
        });
    }

    games
}

fn part_1(games: &Vec<Game>) {
    let mut total_points: usize = 0;

    for game in games {
        println!("Game {}: {}", game.number, game.points());
        total_points += game.points()
    }

    println!("Total points: {}", total_points);
}