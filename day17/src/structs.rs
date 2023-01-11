use std::collections::HashMap;

#[derive(Debug)]
pub enum Wind {
    Left,
    Right,
}

pub struct Coord {
    pub x: u32,
    pub y: u32,
}

pub struct CaveState {
    pub wind_count: u32,
    pub rock_count: u64,
    pub ceiling: u32,
    pub cave: Vec<[bool; 7]>,
    pub current_rock: Coord,
    pub rock_patterns: HashMap<(usize, usize), (usize, usize, usize)>,
    pub pattern_occurence_count: usize,
}

impl CaveState {
    pub fn has_not_collided(&mut self, new_current_rock: &Coord, rock: &[Coord]) -> bool {
        rock.iter().all(|offset| {
            let x = new_current_rock.x + offset.x;
            let y = new_current_rock.y + offset.y;
            while self.cave.len() <= y as usize {
                self.cave.push([false; 7]);
            }
            x < 7 && !self.cave[y as usize][x as usize]
        })
    }
}
