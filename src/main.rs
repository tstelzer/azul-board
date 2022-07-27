#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use colored::Colorize;
use image::{self, GenericImage};
use itertools::join;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

fn print(xs: Vec<Vec<u32>>) {
    let tile_to_color = HashMap::from([
        (0_u32, " 0 ".black().on_truecolor(160, 70, 70).to_string()),
        (1_u32, " 1 ".black().on_truecolor(172, 124, 73).to_string()),
        (2_u32, " 2 ".black().on_truecolor(129, 186, 178).to_string()),
        (3_u32, " 3 ".black().on_truecolor(221, 209, 213).to_string()),
        (4_u32, " 4 ".black().on_truecolor(59, 64, 85).to_string()),
    ]);
    let rl = String::from("\n");
    let mut result = Vec::new();
    for row in xs {
        for field in row {
            result.push(&tile_to_color[&field]);
        }
        result.push(&rl);
    }
    print!("{}", join(&result, ""));
}

fn make_base_board(size: u32) -> Vec<Vec<u32>> {
    let mut board = Vec::new();

    let mut base: Vec<u32> = (0..size).collect();
    board.push(base.clone());
    for _ in 0..base.len() - 1 {
        base.rotate_right(1);
        let mut row = base.clone();
        board.push(row);
    }
    board
}

fn make_board() -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut board = make_base_board(5);
    board.shuffle(&mut rng);
    let rotate_by: usize = rng.gen_range(0..5);
    for mut row in board.iter_mut() {
        row.rotate_left(rotate_by);
    }
    board
}

fn mm_to_pixel(dpi: u32, mm: f32) -> f32 {
    dpi as f32 * mm / 25.4
}

fn pixel_to_mm(dpi: u32, pixel: u32) -> f32 {
    pixel as f32 * 25.4 / dpi as f32
}

fn main() {
    let alpha = 200;
    let tile_to_rgb = HashMap::from([
        (0_u32, image::Rgba([255, 255, 255, alpha])),
        (0_u32, image::Rgba([160, 70, 70, alpha])),
        (1_u32, image::Rgba([172, 124, 73, alpha])),
        (2_u32, image::Rgba([129, 186, 178, alpha])),
        (3_u32, image::Rgba([221, 209, 213, alpha])),
        (4_u32, image::Rgba([59, 64, 85, alpha])),
    ]);
    let margin = 4.32; // default printer margins in mm
    let width_mm = 297.0 - (margin * 2.0); // dina4 in mm
    let height_mm = 210.0 - (margin * 2.0); // dina4 in mm
    let dpi = 150; // default dpi, apparently?

    let width = mm_to_pixel(dpi, width_mm).round() as u32;
    let height = mm_to_pixel(dpi, height_mm).round() as u32;
    let board_width = mm_to_pixel(dpi, 105.0).round() as u32;
    let tile_width = board_width / 5;

    println!(
        "creating image {}x{} over {} with {} tile width at {} dpi",
        height, width, board_width, tile_width, dpi
    );

    let mut img: image::RgbaImage = image::RgbaImage::new(height, width);
    // white page
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }
    let x0 = 100; // px
    let y0 = 100; // px
    let mut x1 = x0;
    let mut y1 = y0;

    let board = make_board();
    print(board.clone());

    for row in board {
        for tile in row {
            println!("tile: {}, x1: {}, y1: {}", tile, x1, y1);
            for x in x1..(x1 + tile_width) {
                for y in y1..(y1 + tile_width) {
                    let pixel = img.get_pixel_mut(x, y);
                    *pixel = tile_to_rgb[&tile];
                }
            }
            // move one tile over
            x1 += tile_width;
        }
        x1 = x0;
        // move one row down
        y1 += tile_width;
    }

    img.save("test.png").unwrap();
    println!("{}", "done");
}
