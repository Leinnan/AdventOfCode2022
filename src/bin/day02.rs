#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Result {
    Win,
    Draw,
    Lose,
}

impl Move {
    pub fn from_char(c: char) -> Move {
        match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("WRONG CHARACTER"),
        }
    }

    pub fn result_with(self, other: Move) -> Result {
        if self == other {
            return Result::Draw;
        }

        match (self, other) {
            (Move::Rock, Move::Scissors) => Result::Win,
            (Move::Paper, Move::Rock) => Result::Win,
            (Move::Scissors, Move::Paper) => Result::Win,
            (_, _) => Result::Lose,
        }
    }

    pub fn counter_move_with_result(self, expected_result: Result) -> Move {
        match (self, expected_result) {
            (_, Result::Draw) => self,
            (Move::Rock, Result::Win) => Move::Paper,
            (Move::Rock, Result::Lose) => Move::Scissors,
            (Move::Paper, Result::Win) => Move::Scissors,
            (Move::Paper, Result::Lose) => Move::Rock,
            (Move::Scissors, Result::Win) => Move::Rock,
            (Move::Scissors, Result::Lose) => Move::Paper,
        }
    }
}

const DRAW: i32 = 3;
const LOST: i32 = 0;
const WON: i32 = 6;

const STRATEGY_GUIDE: &'static str = include_str!("../day02");
// const STRATEGY_GUIDE: &'static str = "A Y
// B X
// C Z";

fn result(one: Move, two: Move) -> i32 {
    match two.result_with(one) {
        Result::Win => WON,
        Result::Draw => DRAW,
        Result::Lose => LOST,
    }
}

fn full_result(rund: (Move, Move)) -> i32 {
    let move_value = match rund.1 {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    move_value + result(rund.0, rund.1)
}

fn line_to_moves(line: &str) -> (Move, Move) {
    let chars = line.chars();
    let first = Move::from_char(chars.clone().into_iter().take(1).last().unwrap());
    let second = Move::from_char(chars.into_iter().last().unwrap());

    (first, second)
}

fn line_to_moves_part_2(line: &str) -> (Move, Move) {
    let chars = line.chars();
    let first = Move::from_char(chars.clone().into_iter().take(1).last().unwrap());
    let expected_result = match chars.into_iter().last().unwrap() {
        'X' => Result::Lose,
        'Y' => Result::Draw,
        _ => Result::Win,
    };
    let second = first.counter_move_with_result(expected_result);

    (first, second)
}

fn main() {
    {
        let total: i32 = STRATEGY_GUIDE
            .lines()
            .map(String::from)
            .map(|line| full_result(line_to_moves(&line)))
            .sum();
        println!("Part One- Total score is {}", total);
    }
    {
        let total: i32 = STRATEGY_GUIDE
            .lines()
            .map(String::from)
            .map(|line| full_result(line_to_moves_part_2(&line)))
            .sum();
        println!("Part two- Total score is {}", total);
    }
}
