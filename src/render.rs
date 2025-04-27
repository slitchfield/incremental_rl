use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
};

use crate::game::GameScreen;
use crate::game::GameState;
use crate::game::Location;
use crate::game::Tile;
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
            if ui.button(None, "Empty button") {
                info!("Pressed empty button!");
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
        for (name, resource) in &state.resources {
            if state.unlocked_resources.contains(name) {
                ui.label(None, &resource.display(name));
            }
        }
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

    // Render the tilemap!
    let local_tilemap;
    if let Some(tilemap) = &state.embark_state.tilemap {
        local_tilemap = tilemap;
    } else {
        todo!()
    }

    let tile_width = 7.5f32;
    let tile_height = 7.5f32;
    let center_x = state.screen_width / 2.0;
    let center_y = height * state.screen_height / 2.0;
    let upper_left_x = center_x - tile_width * local_tilemap.width / 2.0;
    let upper_left_y = center_y - tile_height * local_tilemap.height / 2.0;

    for r in 0..(local_tilemap.height as u32) {
        let tile_y = upper_left_y + (r as f32) * tile_height;
        for c in 0..(local_tilemap.width as u32) {
            let tile_x = upper_left_x + (c as f32) * tile_width;

            let tile_index: usize = (r * (local_tilemap.width as u32) + c).try_into().unwrap();
            match &local_tilemap.tiles[tile_index] {
                Tile::Wall => {
                    draw_rectangle(tile_x, tile_y, tile_width, tile_height, BLACK);
                }
                Tile::Empty => {
                    draw_rectangle(tile_x, tile_y, tile_width, tile_height, WHITE);
                }
                Tile::Resource(tileresource) => {
                    draw_rectangle(tile_x, tile_y, tile_width, tile_height, tileresource.color);
                }
            }
        }
    }

    let player_x = state.embark_state.player_x as f32; // In tiles
    let player_y = state.embark_state.player_y as f32;

    let r = tile_width / 2.0;
    // In pixels; account for (x,y) referring to center in draw_circle, not upper left
    let x = upper_left_x + player_x * tile_width + r;
    let y = upper_left_y + player_y * tile_height + r;
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
                state.resources.get("energy").unwrap().cur_val,
                state.resources.get("energy").unwrap().max_val
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
        GameScreen::Embark => {
            clear_background(DARKGREEN);
            draw_embark_screen(state);
            None
        }
    }
}
