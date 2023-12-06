use std::collections::HashSet;

pub fn run() {
    let games: Vec<Game> = parse_input();
    println!("Part 1");
    part_1(&games);
    
    println!("Part 2");
    part_2(&games)
}

#[derive(Debug, Clone)]
struct Game {
    number: usize,
    winning_numbers: HashSet<usize>,
    player_numbers: HashSet<usize>,
    count: usize
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

    fn number_of_wins(&self) -> usize {
        self.winning_numbers.intersection(&self.player_numbers).count()
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
            count: 1,
            winning_numbers,
            player_numbers
        });
    }

    games
}

fn part_1(games: &Vec<Game>) {
    let mut total_points: usize = 0;

    for game in games {
        // println!("Game {}: {}", game.number, game.points());
        total_points += game.points()
    }

    println!("Total points: {}", total_points);
}

fn part_2(games: &Vec<Game>) {
    let mut games_mut = games.to_vec();
    
    for game in games.iter().enumerate() {
        let num_of_wins: usize = game.1.number_of_wins();

        if num_of_wins > 0 {
            for i in 1..num_of_wins+1 {
                games_mut[game.0+i].count += 1*games_mut[game.0].count;
            }
        }
    }

    let mut total: usize = 0;
    for game in games_mut {
        total += game.count;
    }

    println!("Number of cards: {}", total);
}