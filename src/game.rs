use crate::draw::draw_field;
use crate::Cell;
use piston_window::{Context, G2d};

pub struct Game {
    field: Vec<Vec<Cell>>,
    pub width: u32,
    pub height: u32,
    waiting_time: f64,
    fps: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let field = vec![vec![Cell::new(); width as usize]; height as usize];
        Game {
            field,
            width,
            height,
            waiting_time: 0.0,
            fps: 120.0,
        }
    }
    pub fn fill_random(&mut self) {
        for row in self.field.iter_mut() {
            for cell in row.iter_mut() {
                cell.set_random();
            }
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        println!("{}", delta_time);
        self.waiting_time += delta_time;
        if self.waiting_time >= 1.0 / self.fps {
            self.waiting_time = 0.0;
            self.update_alive_neighbors();
            self.field.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    cell.update();
                });
            });
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_field(&self.field, con, g);
    }

    fn update_alive_neighbors(&mut self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let mut alive_neighbors = 0;
                for dy in -1..2 {
                    for dx in -1..2 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let yy = (y as i32 + dy).rem_euclid(self.height as i32) as usize;
                        let xx = (x as i32 + dx).rem_euclid(self.width as i32) as usize;
                        if self.field[yy][xx].is_alive() {
                            alive_neighbors += 1;
                        }
                    }
                }
                self.field[y][x].set_alive_neighbors(alive_neighbors);
            }
        }
    }
}
