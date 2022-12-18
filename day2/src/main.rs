use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn from_string(move_char: char) -> Move {
        return match move_char {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Invalid Move Parsed"),
        };
    }

    fn win_map(move_1: Move) -> Move {
        return match move_1 {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        };
    }

    fn lose_map(move_1: Move) -> Move {
        return match move_1 {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        };
    }
}

struct GameRound {
    move_1: Move,
    move_2: Move,
    points_1: i32,
    points_2: i32,
}

fn parse_to_rounds_1() -> Vec<GameRound> {
    let mut input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_file.push("input");

    let mut vec: Vec<GameRound> = Vec::new();

    let file = match File::open(input_file) {
        Ok(res) => res,
        Err(_) => panic!("Could Not Read File"),
    };

    let reader = BufReader::new(file);

    for line_res in reader.lines() {
        let line: String = match line_res {
            Ok(line) => line,
            Err(_) => panic!("Problem Reading Line!"),
        };

        let player_1_move: Move = Move::from_string(line.chars().nth(0).unwrap());
        let player_2_move: Move = Move::from_string(line.chars().nth(2).unwrap());

        let mut player_1_score: i32 = player_1_move.clone() as i32;
        let mut player_2_score: i32 = player_2_move.clone() as i32;

        if player_1_move == player_2_move {
            player_1_score += 3;
            player_2_score += 3;
        } else {
            if Move::win_map(player_1_move.clone()) == player_2_move {
                // player 1 wins
                player_1_score += 6;
            } else if Move::win_map(player_2_move.clone()) == player_1_move {
                // player 2 won
                player_2_score += 6;
            }
        }

        println!(
            "Player 1 Move : {:?}   Player 2 Move : {:?}    Player 1 Score: {}  Player 2 Score {}",
            player_1_move, player_2_move, player_1_score, player_2_score
        );
        let current_round: GameRound = GameRound {
            move_1: player_1_move,
            move_2: player_2_move,
            points_1: player_1_score,
            points_2: player_2_score,
        };

        vec.push(current_round);
    }

    return vec;
}

fn parse_to_rounds_2() -> Vec<GameRound> {
    let mut input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_file.push("input");

    let mut vec: Vec<GameRound> = Vec::new();

    let file = match File::open(input_file) {
        Ok(res) => res,
        Err(_) => panic!("Could Not Read File"),
    };

    let reader = BufReader::new(file);

    for line_res in reader.lines() {
        let line: String = match line_res {
            Ok(line) => line,
            Err(_) => panic!("Problem Reading Line!"),
        };

        let player_1_move: Move = Move::from_string(line.chars().nth(0).unwrap());
        let player_2_move: Move = Move::from_string(line.chars().nth(2).unwrap());

        let mut player_1_score: i32 = player_1_move.clone() as i32;
        let mut player_2_score: i32 = player_2_move.clone() as i32;

        if player_1_move == player_2_move {
            player_1_score += 3;
            player_2_score += 3;
        } else {
            if Move::win_map(player_1_move.clone()) == player_2_move {
                // player 1 wins
                player_1_score += 6;
            } else if Move::win_map(player_2_move.clone()) == player_1_move {
                // player 2 won
                player_2_score += 6;
            }
        }

        println!(
            "Player 1 Move : {:?}   Player 2 Move : {:?}    Player 1 Score: {}  Player 2 Score {}",
            player_1_move, player_2_move, player_1_score, player_2_score
        );
        let current_round: GameRound = GameRound {
            move_1: player_1_move,
            move_2: player_2_move,
            points_1: player_1_score,
            points_2: player_2_score,
        };

        vec.push(current_round);
    }

    return vec;
}

fn main() {
    let rounds: Vec<GameRound> = parse_to_rounds_1();
    let mut player_2_total: i32 = 0;

    for round in rounds {
        player_2_total += round.points_2;
    }

    println!("Part 1 - Player 2 Total: {}", player_2_total);
}
