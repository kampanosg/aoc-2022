enum Hand {
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
    
    fn transform(v: &str) -> Option<Hand> {
        match v {
            "A" | "X" => return Some(Rock),
            "B" | "Y" => return Some(Paper),
            "C" | "Z" => return Some(Scissors),
            _ => None,
        }
    }

}
