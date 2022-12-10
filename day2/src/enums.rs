#[derive(Debug, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Result {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Hand {
    pub fn transform(v: &str) -> Option<Hand> {
        match v {
            "A" | "X" => Some(Hand::Rock),
            "B" | "Y" => Some(Hand::Paper),
            "C" | "Z" => Some(Hand::Scissors),
            _ => None,
        }
    }

    pub fn bonus_points(&self) -> u64 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Result {
    pub fn points(&self) -> u64 {
        match *self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

impl Outcome {
    pub fn transform(v: &str) -> Option<Outcome> {
        match v {
            "X" => Some(Outcome::Lose),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None
        }
    }
}
