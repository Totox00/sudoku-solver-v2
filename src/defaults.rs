use crate::board::{Cell, Region};

pub const fn default_cell(size: usize) -> u16 {
    match size {
        4 => 0b1111_0,
        6 => 0b11_1111_0,
        9 => 0b1_1111_1111_0,
        _ => panic!("Default cell with specified size does not exist"),
    }
}

pub fn default_regions(size: usize) -> Vec<Region> {
    let out: Vec<Region> = match size {
        4 => calc_region(2, 2),
        6 => calc_region(3, 2),
        9 => calc_region(3, 3),
        _ => panic!("Default regions for specified size does not exist"),
    };
    out
}

fn calc_region(width: usize, height: usize) -> Vec<Region> {
    calc_region_offsets(height, width)
        .iter()
        .map(|(x, y)| calc_region_contents(width, height, *x, *y))
        .collect()
}

fn calc_region_offsets(width: usize, height: usize) -> Vec<(usize, usize)> {
    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x * height, y * width)))
        .collect()
}

fn calc_region_contents(
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
) -> Vec<Cell> {
    (0..width)
        .flat_map(|x| {
            (0..height).map(move |y| Cell {
                col: x + offset_x,
                row: y + offset_y,
            })
        })
        .collect()
}

pub fn default_region_bounds(size: usize) -> (usize, usize) {
    match size {
        4 => (2, 2),
        6 => (3, 2),
        9 => (3, 3),
        _ => panic!("Default regions for specified size does not exist"),
    }
}
