mod game;
mod maps;
mod board;
mod posn;
use game::GameInstance;
use board::{ClassicTile};
use macroquad::prelude::*;

static HEX_RADIUS: f32 = 20.0;
static BORDER_THICKNESS : f32 = 2.0;
static SIDEWAYS_LENGTH: f32 = 17.32;
static OFFSET: f32 = 100.0;
static VERTICAL_LENGTH: f32 = 15.0;

#[macroquad::main("Geblocken Das Schwein")]
async fn main() {
    let mut g = GameInstance::classic();

    for _ in 0..5 {
        clear_background(GREEN);
        next_frame().await;
    }

    loop {
        clear_background(GREEN);
        let (board_row_count, board_col_count) = g.get_dimensions();

        let (pig_y, pig_x) = g.pig_pos().into();
        for r in 0..board_row_count {
            for c in 0..board_col_count {
                let (x, y) = logical_to_screenspace(r, c);
                if let Some(ClassicTile::Block) = g.tile_at((r, c).into()){
                    draw_hexagon(x, y, HEX_RADIUS, BORDER_THICKNESS, true, GRAY, BLACK);
                } else {
                    draw_hexagon(x, y, HEX_RADIUS, BORDER_THICKNESS, true, GRAY, GREEN);
                }
            }
        }
        
        let (pig_screen_x, pig_screen_y) = logical_to_screenspace(pig_y, pig_x);
        draw_circle(pig_screen_x, pig_screen_y, 10.0, PINK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x_pos, y_pos) = mouse_position();
            let logical_res = screenspace_to_logical(board_row_count, board_col_count, x_pos, y_pos);
            if let Some(coords) = logical_res {
                let (r, c) = coords;
                match g.block((r, c).into()) {
                    Err(msg) => println!("{}", msg),
                    _ => ()
                }
            }
        }
        next_frame().await;
    }
}

fn screenspace_to_logical(row_count: usize, col_count: usize, x: f32, y: f32) -> Option<(usize, usize)> {
    for r in 0..row_count {
        for c in 0..col_count {
            let (x_pos, y_pos) = logical_to_screenspace(r, c);
            if (x - x_pos).powi(2) + (y - y_pos).powi(2) < SIDEWAYS_LENGTH.powi(2) {
                return Option::Some((r, c));
            }
        }
    }

    Option::None

}

fn logical_to_screenspace(r: usize, c: usize) -> (f32, f32) {
    let mut x_pos = (c as f32) * SIDEWAYS_LENGTH * 2.0 + OFFSET;
    if r % 2 == 1 {
        x_pos += SIDEWAYS_LENGTH;
    }

    let y_pos = (r as f32) * VERTICAL_LENGTH * 2.0 + OFFSET;
    (x_pos, y_pos)
}
