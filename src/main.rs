use macroquad::prelude::*;

struct Resources {
    circles: f32,
    squares: f32,
}

struct GameState {
    cursor_x: f32,
    cursor_y: f32,
    screen_width: f32,
    screen_height: f32,

    resources: Resources,
}

fn draw_idle_screen(state: &GameState) {
    // Main window
    draw_rectangle(
        10.0,
        10.0,
        0.8 * state.screen_width - 20.0,
        state.screen_height - 20.0,
        LIGHTGRAY,
    );

    // Resources Window
    draw_rectangle(
        0.8 * state.screen_width + 10.0,
        10.0,
        0.2 * state.screen_width - 20.0,
        state.screen_height - 20.0,
        LIGHTGRAY,
    );
    draw_text(
        format!("squares: {:.0}", state.resources.squares).as_str(),
        0.8 * state.screen_width + 15.0,
        15.0 + 20.0,
        20.0,
        DARKBLUE,
    );
    draw_text(
        format!("circles: {:.0}", state.resources.circles).as_str(),
        0.8 * state.screen_width + 15.0,
        15.0 + 20.0 + 5.0 + 20.0,
        20.0,
        DARKBLUE,
    );
}

#[macroquad::main("BasicShapes")]
async fn main() {
    info!("Starting preamble");

    let mut state: GameState = GameState {
        cursor_x: 0.0,
        cursor_y: 0.0,
        screen_width: 1280.0,
        screen_height: 800.0,
        resources: Resources {
            circles: 0.0,
            squares: 0.0,
        },
    };

    request_new_screen_size(state.screen_width, state.screen_height);
    if screen_width() != state.screen_width {
        warn!("Could not hit requested screen width");
        warn!(
            "\tRequested {}, Received {}",
            state.screen_width,
            screen_width()
        );
    }
    if screen_height() != state.screen_height {
        warn!("Could not hit requested screen height");
        warn!(
            "\tRequested {}, Received {}",
            state.screen_height,
            screen_height()
        );
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
            if state.cursor_y > screen_height() {
                state.cursor_y = screen_height();
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
            if state.cursor_x > screen_width() {
                state.cursor_x = screen_width();
            }
        }

        // Logic

        // Render
        clear_background(BLACK);

        draw_idle_screen(&state);

        draw_circle(state.cursor_x - 30.0, state.cursor_y - 30.0, 15.0, YELLOW);

        // Advance
        next_frame().await
    }
}
