use rand::{self, Rng};

#[derive(Copy, Clone)]
pub struct Cell {
    alive: bool,
    alive_neighbors: u8,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            alive: false,
            alive_neighbors: 0,
        }
    }
    pub fn set_alive(&mut self) {
        self.alive = true;
    }
    pub fn set_dead(&mut self) {
        self.alive = false;
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn set_alive_neighbors(&mut self, alive_neighbors: u8) {
        self.alive_neighbors = alive_neighbors;
    }
    pub fn update(&mut self) {
        if self.alive {
            if self.alive_neighbors < 2 || self.alive_neighbors > 3 {
                self.set_dead();
            }
        } else {
            if self.alive_neighbors == 3 {
                self.set_alive();
            }
        }
    }
    pub fn set_random(&mut self) {
        self.alive = rand::thread_rng().gen_range(0.0..6.0) < 1.0;
    }
}
