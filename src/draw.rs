use crate::Cell;
use piston_window::{rectangle, Context, G2d};

pub const CELL_SIZE: f64 = 10.0;

pub fn cell_to_gui_coords(n: u32) -> f64 {
    CELL_SIZE * (n as f64)
}

pub fn draw_field(field: &Vec<Vec<Cell>>, con: &Context, g: &mut G2d) {
    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let [x, y] = [cell_to_gui_coords(x as u32), cell_to_gui_coords(y as u32)];
            if cell.is_alive() {
                rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    [x, y, CELL_SIZE, CELL_SIZE],
                    con.transform,
                    g,
                );
            }
        }
    }
}
