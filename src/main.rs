use macroquad::prelude::*;

struct GameState {
    cursor_x: f32,
    cursor_y: f32,
    screen_width: f32,
    screen_height: f32,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut state: GameState = GameState {
        cursor_x: 0.0,
        cursor_y: 0.0,
        screen_width: 800.0,
        screen_height: 600.0,
    };
    request_new_screen_size(state.screen_width, state.screen_height);
    if screen_width() != state.screen_width {
        println!("Could not hit requested screen width");
    }
    if screen_height() != state.screen_height {
        println!("Could not hit requested screen height");
    }

    loop {
        // Input handling
        if is_key_down(KeyCode::Up) {
            state.cursor_y -= 1.0;
            if state.cursor_y < 0.0 {
                state.cursor_y = 0.0
            }
        }
        if is_key_down(KeyCode::Down) {
            state.cursor_y += 1.0;
            if state.cursor_y > screen_width() {
                state.cursor_y = screen_width();
            }
        }
        if is_key_down(KeyCode::Left) {
            state.cursor_x -= 1.0;
            if state.cursor_x < 0.0 {
                state.cursor_x = 0.0
            }
        }
        if is_key_down(KeyCode::Right) {
            state.cursor_x += 1.0;
            if state.cursor_x > screen_height() {
                state.cursor_x = screen_height();
            }
        }
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(state.cursor_x - 30.0, state.cursor_y - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
