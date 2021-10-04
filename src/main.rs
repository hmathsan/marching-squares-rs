use macroquad::prelude::*;
use noise::{NoiseFn, OpenSimplex};

const RESOLUTION: i32 = 10;
const STEP: f64 = 0.1;
const Z_OFFSET: f64 = 0.03;

fn window_config() -> Conf {
    Conf {
        window_title: "Marching Squares".to_owned(),
        window_width: 600,
        window_height: 400,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let noise = OpenSimplex::new();
    
    let cols: i32 = 1 + screen_width() as i32 / RESOLUTION;
    let rows: i32 = 1 + screen_height() as i32 / RESOLUTION;

    let mut fields: Vec<Vec<f64>> = vec![vec![Default::default(); cols as usize]; rows as usize];

    println!("cols {}; rows {}", cols - 1, rows - 1);
    println!("len {};{}", fields[0].len(), fields.len());

    println!("{:?}", noise.get([100.0, 100.0, 1.0]));

    let mut z: f64 = 0.0;
    loop {
        clear_background(GRAY);

        for i in 0..cols {
            for j in 0..rows {
                fields[j as usize][i as usize] = noise.get([i as f64 * STEP, j as f64 * STEP, z]);
            }
        }
    
        z += Z_OFFSET;
        // Draw dots
        for i in 0..cols {
            for j in 0..rows {
                draw_circle((i * RESOLUTION) as f32, (j * RESOLUTION) as f32, RESOLUTION as f32 * 0.15, 
                    Color::from_rgba(
                        (fields[j as usize][i as usize] * 255.0).ceil() as u8, 
                        (fields[j as usize][i as usize] * 255.0).ceil() as u8, 
                        (fields[j as usize][i as usize] * 255.0).ceil() as u8, 255)
                );
            }
        }

        // Draw lines
        for i in 0..cols - 1 {
            for j in 0..rows - 1 {
                let f_res = RESOLUTION as f32;
            
                let x: f32 = (i * RESOLUTION) as f32;
                let y: f32 = (j * RESOLUTION) as f32;

                let (a_x, a_y) = (x + f_res * 0.5, y);
                let (b_x, b_y) = (x + f_res, y + f_res * 0.5);
                let (c_x, c_y) = (x + f_res * 0.5, y + f_res);
                let (d_x, d_y) = (x, y + f_res * 0.5);

                let state = 
                    (fields[j as usize][i as usize].ceil() * 8.0) as i32 +
                    (fields[j as usize][(i + 1) as usize].ceil() * 4.0) as i32 +
                    (fields[(j + 1) as usize][(i + 1) as usize].ceil() * 2.0) as i32 +
                    (fields[(j + 1) as usize][i as usize].ceil() * 1.0) as i32;

                match state {
                    1 => draw_line(c_x, c_y, d_x, d_y, 1.0, WHITE),
                    2 => draw_line(b_x, b_y, c_x, c_y, 1.0, WHITE),
                    3 => draw_line(b_x, b_y, d_x, d_y, 1.0, WHITE),
                    4 => draw_line(a_x, a_y, b_x, b_y, 1.0, WHITE),
                    5 => {
                        draw_line(a_x, a_y, d_x, d_y, 1.0, WHITE);
                        draw_line(b_x, b_y, c_x, c_y, 1.0, WHITE);
                    },
                    6 => draw_line(a_x, a_y, c_x, c_y, 1.0, WHITE),
                    7 => draw_line(a_x, a_y, d_x, d_y, 1.0, WHITE),
                    8 => draw_line(a_x, a_y, d_x, d_y, 1.0, WHITE),
                    9 => draw_line(a_x, a_y, c_x, c_y, 1.0, WHITE),
                    10 => {
                        draw_line(a_x, a_y, b_x, b_y, 1.0, WHITE);
                        draw_line(c_x, c_y, d_x, d_y, 1.0, WHITE);
                    },
                    11 => draw_line(a_x, a_y, b_x, b_y, 1.0, WHITE),
                    12 => draw_line(b_x, b_y, d_x, d_y, 1.0, WHITE),
                    13 => draw_line(b_x, b_y, c_x, c_y, 1.0, WHITE),
                    14 => draw_line(c_x, c_y, d_x, d_y, 1.0, WHITE),
                    _ => {}
                }
            }
        }

        next_frame().await
    }
}
