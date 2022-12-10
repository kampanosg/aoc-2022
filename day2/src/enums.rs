#[derive(Debug)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Result {
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

}
