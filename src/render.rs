use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
};

use crate::game::GameScreen;
use crate::game::GameState;
use crate::game::Location;
use crate::game::UiEvent;

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
                    Location::Embark(val) => {
                        if ui.button(None, format!("Embark {:?}", val)) {
                            return_event = Some(UiEvent::EmbarkLocation(*location));
                            // Copies if necessary
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

fn draw_embark_screen(state: &GameState) {
    // Draw RL screen
    let gutter = 10.0;
    let main_width = 1.0;
    let _res_width = 0.2;
    let height = 0.9;

    draw_rectangle(
        gutter,
        gutter,
        main_width * state.screen_width - 2.0 * gutter,
        height * state.screen_height - 2.0 * gutter,
        LIGHTGRAY,
    );

    let cur_location = state.cur_location;

    let embark_params;
    if let Location::Embark(embark_val) = cur_location {
        embark_params = embark_val;
    } else {
        error!("Entered draw_embark_screen without a valid inner embark state");
        unimplemented!(); // Handle error case
    }
    
    let x = state.embark_state.player_x as f32;
    let y = state.embark_state.player_y as f32;
    let r = embark_params.dims.x as f32;
    draw_circle(x, y, r, RED);
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
            format!(
                "[{:.3} / {:.3}]\t|\t{:.6}s/frame => {:.6} fps",
                tick_timer,
                state.tick_duration,
                get_frame_time(),
                1.0 / get_frame_time()
            )
            .as_str(),
        );
    });
}

pub fn render_frame(state: &GameState) -> Option<UiEvent> {
    draw_status_bar(state);

    match state.game_mode {
        GameScreen::_Title => todo!(),
        GameScreen::Idle => {
            clear_background(BLACK);
            draw_idle_screen(state)
        }
        GameScreen::Embark(_location) => {
            clear_background(DARKGREEN);
            draw_embark_screen(state);
            None
        }
    }
}
