use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};

struct Resources {
    circles: f32,
    squares: f32,
}

enum UiEvent {
    BuyCircle(f32),
}

struct GameState {
    cursor_x: f32,
    cursor_y: f32,
    screen_width: f32,
    screen_height: f32,
    event_queue: Vec<UiEvent>, // TODO: Consider staticly allocated backing
    resources: Resources,
}

fn draw_idle_screen(state: &GameState) -> Option<UiEvent> {
    let mut return_event = None;

    // Main window
    draw_rectangle(
        10.0,
        10.0,
        0.8 * state.screen_width - 20.0,
        state.screen_height - 20.0,
        LIGHTGRAY,
    );
    widgets::Window::new(
        hash!(),
        vec2(10.0, 10.0),
        vec2(0.8 * state.screen_width - 20.0, state.screen_height - 20.0),
    )
    .movable(false)
    .label("Main Window")
    .ui(&mut root_ui(), |ui| {
        ui.label(None, "Main Window Label");
        if ui.button(None, "Buy Circle") {
            return_event = Some(UiEvent::BuyCircle(1.0));
        }
    });

    // Resources Window
    draw_rectangle(
        0.8 * state.screen_width + 10.0,
        10.0,
        0.2 * state.screen_width - 20.0,
        state.screen_height - 20.0,
        LIGHTGRAY,
    );
    widgets::Window::new(
        hash!(),
        vec2(0.8 * state.screen_width + 10.0, 10.0),
        vec2(0.2 * state.screen_width - 20.0, state.screen_height - 20.0),
    )
    .movable(false)
    .label("Resource Window")
    .ui(&mut root_ui(), |ui| {
        ui.label(
            None,
            format!("Circles: {}", state.resources.circles).as_str(),
        );
        ui.label(
            None,
            format!("Squares: {}", state.resources.squares).as_str(),
        );
    });

    return_event
}

#[macroquad::main("Unnamed Incremental Roguelike")]
async fn main() {
    info!("Starting preamble");

    let mut state: GameState = GameState {
        cursor_x: 0.0,
        cursor_y: 0.0,
        screen_width: 1280.0,
        screen_height: 800.0,
        event_queue: vec![],
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
        while let Some(event) = state.event_queue.pop() {
            match event {
                UiEvent::BuyCircle(new_delta) => {
                    state.resources.circles += new_delta;
                }
            }
        }

        // Render
        clear_background(BLACK);

        // draw_idle_screen contains ui elements, which can possibly return events from buttons
        if let Some(event) = draw_idle_screen(&state) {
            state.event_queue.push(event);
        }

        draw_circle(state.cursor_x - 30.0, state.cursor_y - 30.0, 15.0, YELLOW);

        // Advance
        next_frame().await
    }
}
