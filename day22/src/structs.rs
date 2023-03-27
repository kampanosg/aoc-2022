pub enum Move {
    Up(u8),
    Rotate(Turn),
}

pub enum Turn {
    L,
    R,
}

pub enum Block {
    Available,
    Wall,
    Empty,
}