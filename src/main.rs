pub mod game;
pub mod render;

use game::GameState;
use game::UiEvent;

use render::render_frame;

use macroquad::prelude::*;

#[macroquad::main("Unnamed Incremental Roguelike")]
async fn main() {
    info!("Starting preamble");

    let mut state: GameState = GameState::default();

    let mut platform_event_queue: Vec<UiEvent> = vec![];

    request_new_screen_size(state.screen_width, state.screen_height);
    next_frame().await;
    if (screen_width() - state.screen_width).abs() <= 1.0 {
        warn!("Could not hit requested screen width");
        warn!(
            "\tRequested {}, Received {}",
            state.screen_width,
            screen_width()
        );
    }
    if (screen_height() - state.screen_height).abs() <= 1.0 {
        warn!("Could not hit requested screen height");
        warn!(
            "\tRequested {}, Received {}",
            state.screen_height,
            screen_height()
        );
    }

    let mut frame_counter: usize = 0usize;

    loop {
        // Input handling

        // Check if screen size has changed every .1 seconds
        // TODO: Check and enforce framerate
        if frame_counter % 6 == 0usize {
            platform_event_queue.push(UiEvent::Resize(screen_width(), screen_height()));
        }

        // TODO: Gather all pressed keys to pass?
        if is_key_pressed(KeyCode::Q) {
            platform_event_queue.push(UiEvent::KeyPress(KeyCode::Q));
        }
        if is_key_pressed(KeyCode::I) {
            platform_event_queue.push(UiEvent::KeyPress(KeyCode::I));
        }
        if is_key_pressed(KeyCode::Right) {
            platform_event_queue.push(UiEvent::KeyPress(KeyCode::Right));
        }
        if is_key_pressed(KeyCode::Left) {
            platform_event_queue.push(UiEvent::KeyPress(KeyCode::Left));
        }
        if is_key_pressed(KeyCode::Up) {
            platform_event_queue.push(UiEvent::KeyPress(KeyCode::Up));
        }
        if is_key_pressed(KeyCode::Down) {
            platform_event_queue.push(UiEvent::KeyPress(KeyCode::Down));
        }
        if is_key_pressed(KeyCode::Space) {
            platform_event_queue.push(UiEvent::KeyPress(KeyCode::Space));
        }

        state.process_inputs(&mut platform_event_queue);
        if state.exit_requested {
            break;
        }

        // Logic
        state.process_frame();

        // Render
        // draw_idle_screen contains ui elements, which can possibly return events from buttons
        if let Some(event) = render_frame(&state) {
            platform_event_queue.push(event);
        }

        // Advance
        frame_counter += 1;
        next_frame().await
    }
}
