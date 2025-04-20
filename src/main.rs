use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
};

struct Resource {
    cur_val: f32,
    max_val: f32,
}

impl Resource {
    fn add_or_max(&mut self, delta: f32) {
        let mut new_val = self.cur_val + delta;
        if new_val > self.max_val {
            new_val = self.max_val;
        }
        if new_val < 0.0 {
            new_val = 0.0;
        }
        self.cur_val = new_val;
    }
}

struct Resources {
    energy: Resource,
    circles: f32,
    squares: f32,
}

enum UiEvent {
    BuyCircle(f32),
    SurveySurroundings,
    EmbarkLocation,
}

enum Location {
    AtBase,
    Placeholder(u32),
}

impl Location {
    fn generate_location(_state: &GameState) -> Location {
        Location::Placeholder(0)
    }
}

struct GameState {
    screen_width: f32,
    screen_height: f32,

    last_tick: f64,
    tick_duration: f64,

    event_queue: Vec<UiEvent>, // TODO: Consider staticly allocated backing

    resources: Resources,
    cur_location: Location,
    scouted_locations: Vec<Location>,
}

impl GameState {
    fn idle_tick(&mut self) {
        // Placeholder, increment squares by circle count
        self.resources.squares += self.resources.circles;

        // Handle Energy
        //   Currently, when at base, recharge 1.0 unit per tick
        match self.cur_location {
            Location::AtBase => {
                self.resources.energy.add_or_max(1.0);
            }
            Location::Placeholder(_val) => {
                // Currently, do nothing
            }
        }
    }

    fn survey_surroundings(&mut self) {
        // TODO: Check current unlocks for possible locations
        let location = Location::generate_location(self);
        self.scouted_locations.push(location);
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
        ui.tree_node(hash!(), "First Landing", |ui| {
            ui.separator();
            Group::new(hash!("survey"), Vec2::new(200., 65.)).ui(ui, |ui| {
                ui.label(Vec2::new(5., 5.), "Survey Surroundings");
                ui.label(Vec2::new(5., 20.), &format!("{} Energy", 100));
                if ui.button(Vec2::new(5., 38.), "Ping") {
                    return_event = Some(UiEvent::SurveySurroundings);
                }
            });
        });
        ui.separator();
        ui.tree_node(hash!(), "Placeholder Functions", |ui| {
            if ui.button(None, "Buy Circle") {
                return_event = Some(UiEvent::BuyCircle(1.0));
            }
        });
        ui.separator();
        ui.tree_node(hash!(), "Embark Locations", |ui| {
            for location in &state.scouted_locations {
                match location {
                    Location::AtBase => {
                        warn!("At Base should not show up in scouted locations");
                    }
                    Location::Placeholder(val) => {
                        if ui.button(None, format!("Embark {}", val)) {
                            return_event = Some(UiEvent::EmbarkLocation);
                        }
                    }
                }
            }
        });
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
        ui.label(
            None,
            format!(
                "Batteries: [{:.3} / {:.3}]",
                state.resources.energy.cur_val, state.resources.energy.max_val
            )
            .as_str(),
        );
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
        screen_width: 1920.0,
        screen_height: 1080.0,
        last_tick: get_time(),
        tick_duration: 1.0f64,
        event_queue: vec![],
        cur_location: Location::AtBase,
        scouted_locations: vec![],
        resources: Resources {
            energy: Resource {
                cur_val: 0.0,
                max_val: 100.0,
            },
            circles: 0.0,
            squares: 0.0,
        },
    };

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

    loop {
        // Input handling
        while let Some(event) = state.event_queue.pop() {
            match event {
                UiEvent::BuyCircle(new_delta) => {
                    state.resources.circles += new_delta;
                }
                UiEvent::SurveySurroundings => {
                    if state.resources.energy.cur_val >= 100.0 {
                        // TODO: Abstract cost of surveying
                        state.resources.energy.add_or_max(-100.0);
                        info!("Surveying Surroundings...");
                        state.survey_surroundings();
                    }
                }
                UiEvent::EmbarkLocation => todo!(),
            }
        }

        // TODO: Re-check screensize periodically OR find event based system

        // Logic
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

        // Advance
        next_frame().await
    }
}
