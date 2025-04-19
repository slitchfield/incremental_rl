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
    last_tick: f64,
    tick_duration: f64,
    event_queue: Vec<UiEvent>, // TODO: Consider staticly allocated backing
    resources: Resources,
}

impl GameState {
    fn idle_tick(&mut self) {
        // Placeholder, increment squares by circle count
        self.resources.squares += self.resources.circles;
    }
}

fn draw_idle_screen(state: &GameState) -> Option<UiEvent> {
    let mut return_event = None;

    let gutter = 10.0;
    let main_width = 0.8;
    let res_width = 0.2;
    let height = 0.9;
    // Main window
    draw_rectangle(
        gutter,
        gutter,
        main_width * state.screen_width - 2.0 * gutter,
        height * state.screen_height - 2.0 * gutter,
        LIGHTGRAY,
    );
    widgets::Window::new(
        hash!(),
        vec2(gutter, gutter),
        vec2(
            main_width * state.screen_width - 2.0 * gutter,
            height * state.screen_height - 2.0 * gutter,
        ),
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
        main_width * state.screen_width + gutter,
        gutter,
        res_width * state.screen_width - 2.0 * gutter,
        height * state.screen_height - 2.0 * gutter,
        LIGHTGRAY,
    );
    widgets::Window::new(
        hash!(),
        vec2(main_width * state.screen_width + gutter, gutter),
        vec2(
            res_width * state.screen_width - 2.0 * gutter,
            height * state.screen_height - 2.0 * gutter,
        ),
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

fn draw_status_bar(state: &GameState) {
    let gutter = 10.0;
    let _main_width = 0.8;
    let _res_width = 0.2;
    let height = 0.9;

    draw_rectangle(
        gutter,
        height * state.screen_height + gutter,
        state.screen_width - 2.0 * gutter,
        (1.0 - height) * state.screen_height - 2.0 * gutter,
        LIGHTGRAY,
    );
    widgets::Window::new(
        hash!(),
        vec2(gutter, height * state.screen_height + gutter),
        vec2(
            state.screen_width - 2.0 * gutter,
            (1.0 - height) * state.screen_height - 2.0 * gutter,
        ),
    )
    .movable(false)
    .label("Status Window")
    .ui(&mut root_ui(), |ui| {
        ui.label(None, "Status Window Lable");
        let tick_timer = get_time() - state.last_tick;
        ui.label(
            None,
            format!("[{:.3} / {:.3}]", tick_timer, state.tick_duration).as_str(),
        );
    });
}

#[macroquad::main("Unnamed Incremental Roguelike")]
async fn main() {
    info!("Starting preamble");

    let mut state: GameState = GameState {
        cursor_x: 0.0,
        cursor_y: 0.0,
        screen_width: 1920.0,
        screen_height: 1080.0,
        last_tick: get_time(),
        tick_duration: 15.0f64,
        event_queue: vec![],
        resources: Resources {
            circles: 0.0,
            squares: 0.0,
        },
    };

    request_new_screen_size(state.screen_width, state.screen_height);
    next_frame().await;
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

        let cur_time = get_time();
        if (cur_time - state.last_tick) >= state.tick_duration {
            state.idle_tick();

            // Set last tick time
            state.last_tick = cur_time;
        }

        // Render
        clear_background(BLACK);

        // draw_idle_screen contains ui elements, which can possibly return events from buttons
        if let Some(event) = draw_idle_screen(&state) {
            state.event_queue.push(event);
        }

        draw_status_bar(&state);

        draw_circle(state.cursor_x - 30.0, state.cursor_y - 30.0, 15.0, YELLOW);

        // Advance
        next_frame().await
    }
}
