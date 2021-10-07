use macroquad::prelude::*;
use noise::{NoiseFn, OpenSimplex};

const RESOLUTION: i32 = 10;
const STEP: f64 = 0.1;
const Z_OFFSET: f64 = 0.01;
const LERP_T: f32 = 1.0;

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

    let interpolated = true;

    let mut fields: Vec<Vec<f64>> = vec![vec![Default::default(); cols as usize]; rows as usize];

    println!("cols {}; rows {}", cols - 1, rows - 1);
    println!("len {};{}", fields[0].len(), fields.len());

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
        if interpolated {
            draw_interpolated_squares(&cols, &rows, &fields);
        } else {
            draw_marching_squares(&cols, &rows, &fields);
        }

        next_frame().await
    }
}

fn draw_marching_squares(cols: &i32, rows: &i32, fields: &Vec<Vec<f64>>) {
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
}

fn draw_interpolated_squares(cols: &i32, rows: &i32, fields: &Vec<Vec<f64>>) {
    for i in 0..cols - 1 {
        for j in 0..rows - 1 {
            let f_res = RESOLUTION as f32;
        
            let x: f32 = (i * RESOLUTION) as f32;
            let y: f32 = (j * RESOLUTION) as f32;

            let (a_x, a_y) = (x, y);
            let (b_x, _b_y) = (x + f_res, y);
            let (c_x, c_y) = (x, y + f_res);
            let (d_x, d_y) = (x + f_res, y + f_res);

            let v_a = fields[j as usize][i as usize] as f32;
            let v_b = fields[j as usize][(i + 1) as usize] as f32;
            let v_c = fields[(j + 1) as usize][i as usize] as f32;
            let v_d = fields[(j + 1) as usize][(i + 1) as usize] as f32;

            let state = 
                (fields[j as usize][i as usize].ceil() * 8.0) as i32 +
                (fields[j as usize][(i + 1) as usize].ceil() * 4.0) as i32 +
                (fields[(j + 1) as usize][(i + 1) as usize].ceil() * 2.0) as i32 +
                (fields[(j + 1) as usize][i as usize].ceil() * 1.0) as i32;

            match state {
                1 => {
                    let (q_x, q_y) = (c_x, y + f_res * lerp(v_a, v_c, LERP_T));
                    let (p_x, p_y) = (x + f_res * lerp(v_d, v_c, LERP_T), d_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                2 => {
                    let (q_x, q_y) = (d_x, y + f_res * lerp(v_b, v_d, LERP_T));
                    let (p_x, p_y) = (x + f_res * lerp(v_c, v_d, LERP_T), d_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                3 => {
                    let (q_x, q_y) = (c_x, y + f_res * lerp(v_a, v_c, LERP_T));
                    let (p_x, p_y) = (b_x, y + f_res * lerp(v_b, v_d, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                4 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_a, v_b, LERP_T), a_y);
                    let (p_x, p_y) = (b_x, y + f_res * lerp(v_d, v_b, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                5 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_a, v_b, LERP_T), a_y);
                    let (p_x, p_y) = (c_x, y + f_res * lerp(v_a, v_c, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);

                    let (q_x, q_y) = (b_x, y + f_res * lerp(v_b, v_d, LERP_T));
                    let (p_x, p_y) = (x + f_res * lerp(v_c, v_d, LERP_T) , c_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                6 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_a, v_b, LERP_T), a_y);
                    let (p_x, p_y) = (x + f_res * lerp(v_c, v_d, LERP_T), c_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                7 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_a, v_b, LERP_T), a_y);
                    let (p_x, p_y) = (a_x, y + f_res * lerp(v_a, v_c, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                8 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_b, v_a, LERP_T), a_y);
                    let (p_x, p_y) = (a_x, y + f_res * lerp(v_c, v_a, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                9 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_b, v_a, LERP_T), a_y);
                    let (p_x, p_y) = (x + f_res * lerp(v_d, v_c, LERP_T), c_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                10 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_b, v_a, LERP_T), a_y);
                    let (p_x, p_y) = (b_x, y + f_res * lerp(v_d, v_b, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);

                    let (q_x, q_y) = (a_x, y + f_res * lerp(v_c, v_a, LERP_T));
                    let (p_x, p_y) = (x + f_res * lerp(v_d, v_c, LERP_T), c_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                11 => {
                    let (q_x, q_y) = (x + f_res * lerp(v_b, v_a, LERP_T), a_y);
                    let (p_x, p_y) = (b_x, y + f_res * lerp(v_b, v_d, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                12 => {
                    let (q_x, q_y) = (a_x, y + f_res * lerp(v_c, v_a, LERP_T));
                    let (p_x, p_y) = (b_x, y + f_res * lerp(v_d, v_b, LERP_T));
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                13 => {
                    let (q_x, q_y) = (b_x, y + f_res * lerp(v_d, v_b, LERP_T));
                    let (p_x, p_y) = (x + f_res * lerp(v_d, v_c, LERP_T), c_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                14 => {
                    let (q_x, q_y) = (a_x, y + f_res * lerp(v_c, v_a, LERP_T));
                    let (p_x, p_y) = (x + f_res * lerp(v_c, v_d, LERP_T), c_y);
                    draw_line(q_x, q_y, p_x, p_y, 1.0, WHITE);
                },
                _ => {}
            }
        }
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    ((start * (1.0 - t) + end * t) * 10.0).clamp(0.0, 1.0)
}
