use macroquad::prelude::*;
use ::rand::prelude::*;

const RESOLUTION: i32 = 20;

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
    let mut rng = ::rand::thread_rng();

    let cols: i32 = 1 + screen_width() as i32 / RESOLUTION;
    let rows: i32 = 1 + screen_height() as i32 / RESOLUTION;

    let mut fields: Vec<Vec<i32>> = vec![vec![Default::default(); cols as usize]; rows as usize];

    println!("cols {}; rows {}", cols - 1, rows - 1);
    println!("len {};{}", fields[0].len(), fields.len());

    for i in 0..cols {
        for j in 0..rows {
            fields[j as usize][i as usize] = rng.gen_range(0..=1);
        }
    }

    loop {
        clear_background(GRAY);

        for i in 0..cols {
            for j in 0..rows {
                draw_circle((i * RESOLUTION) as f32, (j * RESOLUTION) as f32, RESOLUTION as f32 * 0.15, 
                    Color::from_rgba(
                        (fields[j as usize][i as usize] * 255) as u8, 
                        (fields[j as usize][i as usize] * 255) as u8, 
                        (fields[j as usize][i as usize] * 255) as u8, 255)
                );
            }
        }

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
                    fields[j as usize][i as usize] * 8 +
                    fields[j as usize][(i + 1) as usize] * 4 +
                    fields[(j + 1) as usize][(i + 1) as usize] * 2 +
                    fields[(j + 1) as usize][i as usize] * 1;

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


        // draw_line(40.0, 40.0, 80.0, 80.0, 15.0, GREEN);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
