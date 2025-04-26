use macroquad::prelude::*;
use std::collections::HashMap;

pub enum UiEvent {
    EmbarkLocation(Location),
    KeyPress(KeyCode),
    Quit,
    Resize(f32, f32),
    StateTransition(GameScreen),
    SurveySurroundings,
}

pub struct Resource {
    pub cur_val: f32,
    pub max_val: f32,
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

    pub fn display(&self, name: &String) -> String {
        format!("{}: {:.3} / {:.3}", name, self.cur_val, self.max_val)
    }
}

pub struct Resources {
    pub energy: Resource,
    pub iron_ore: Resource,
    pub bauxite: Resource,
    pub sandstone: Resource,
}

pub enum Tile {
    Empty,
    Wall,
}

#[derive(Default)]
pub struct TileMap {
    pub width: f32,
    pub height: f32,
    pub tiles: Vec<Tile>,
}

pub struct EmbarkState {
    pub player_x: u32,
    pub player_y: u32,
    del_x: Option<f32>,
    del_y: Option<f32>,
    pub tilemap: Option<TileMap>,
}

impl Default for EmbarkState {
    fn default() -> Self {
        EmbarkState {
            player_x: 100 / 2,
            player_y: 100 / 2,
            del_x: None,
            del_y: None,
            tilemap: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmbarkParams {
    pub seed: usize,
    pub dims: Vec2,
}

impl Default for EmbarkParams {
    fn default() -> Self {
        EmbarkParams {
            seed: 0usize,
            dims: vec2(100.0, 100.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Location {
    AtBase,
    Embark(EmbarkParams),
}

impl Location {
    fn generate_location(_state: &GameState) -> Location {
        Location::Embark(EmbarkParams::default())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameScreen {
    _Title,
    Idle,
    Embark,
}

pub struct GameState {
    pub exit_requested: bool,

    pub screen_width: f32,
    pub screen_height: f32,

    pub last_tick: f64,
    pub tick_duration: f64,

    pub game_mode: GameScreen,
    pub next_game_mode: Option<GameScreen>,
    pub next_location: Option<Location>,

    //pub resources: Resources,
    pub resources: HashMap<String, Resource>,
    pub cur_location: Location,
    pub scouted_locations: Vec<Location>,

    // Embark specific state
    pub embark_state: EmbarkState,
}

impl Default for GameState {
    fn default() -> Self {
        let mut default_resources: HashMap<String, Resource> = HashMap::new();
        default_resources.insert(
            "energy".to_string(),
            Resource {
                cur_val: 100.0,
                max_val: 100.0,
            },
        );
        default_resources.insert(
            "bauxite".to_string(),
            Resource {
                cur_val: 0.0,
                max_val: 100.0,
            },
        );
        default_resources.insert(
            "iron_ore".to_string(),
            Resource {
                cur_val: 0.0,
                max_val: 100.0,
            },
        );
        default_resources.insert(
            "sandstone".to_string(),
            Resource {
                cur_val: 0.0,
                max_val: 100.0,
            },
        );
        GameState {
            exit_requested: false,

            screen_width: 1920.0,
            screen_height: 1080.0,

            last_tick: get_time(),
            tick_duration: 1.0f64,

            game_mode: GameScreen::Idle,
            next_game_mode: None,
            next_location: None,

            cur_location: Location::AtBase,
            scouted_locations: vec![],
            resources: default_resources,

            embark_state: EmbarkState::default(),
        }
    }
}

impl GameState {
    pub fn idle_tick(&mut self) {
        // Handle Energy
        //   Currently, when at base, recharge 1.0 unit per tick
        match self.cur_location {
            Location::AtBase => {
                self.resources.get_mut("energy").unwrap().add_or_max(1.0);
            }
            Location::Embark(_val) => {
                // Currently, do nothing
            }
        }
    }

    fn survey_surroundings(&mut self) {
        // TODO: Check current unlocks for possible locations
        let location = Location::generate_location(self);
        self.scouted_locations.push(location);
    }

    fn generate_tilemap(&self) -> TileMap {
        let mut tilemap = TileMap::default();

        let embark_params;
        if let Location::Embark(params) = self.cur_location {
            embark_params = params;
        } else {
            todo!("Handle cur_location/tilemap gen disagreement");
        }

        tilemap.width = embark_params.dims.x;
        tilemap.height = embark_params.dims.y;

        for x in 0..(tilemap.width as u32) {
            for y in 0..(tilemap.height as u32) {
                if y == 0
                    || (y == (tilemap.height as u32) - 1)
                    || x == 0
                    || (x == (tilemap.width as u32) - 1)
                // || x == y
                {
                    tilemap.tiles.push(Tile::Wall);
                } else {
                    tilemap.tiles.push(Tile::Empty);
                }
            }
        }

        tilemap
    }

    // Translate received keypresses into a potential state update
    pub fn process_keypress(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::I => {
                self.next_game_mode = Some(GameScreen::Idle);
            }
            KeyCode::Q => {
                self.exit_requested = true;
            }
            KeyCode::Up => {
                self.embark_state.del_y = Some(-1.0);
            }
            KeyCode::Down => {
                self.embark_state.del_y = Some(1.0);
            }
            KeyCode::Left => {
                self.embark_state.del_x = Some(-1.0);
            }
            KeyCode::Right => {
                self.embark_state.del_x = Some(1.0);
            }
            _ => {
                warn!("Unhandled keycode: {:?}", keycode);
            }
        }
    }

    // Currently, all inputs arive as `UiEvent` variants
    //   Generally, stash desired update separately from actual state
    //   for implementation in `process_frame()`. Most state updates
    //   wrapped in an `Option<>` to indicate liveness.
    pub fn process_inputs(&mut self, events: &mut Vec<UiEvent>) {
        while let Some(event) = events.pop() {
            match event {
                UiEvent::EmbarkLocation(location) => {
                    // Switch to embark/roguelike mode
                    self.next_game_mode = Some(GameScreen::Embark);
                    self.next_location = Some(location);
                }
                UiEvent::KeyPress(key) => {
                    self.process_keypress(key);
                }
                UiEvent::Quit => {
                    self.exit_requested = true;
                }
                UiEvent::Resize(w, h) => {
                    self.screen_width = w;
                    self.screen_height = h;
                }
                UiEvent::StateTransition(next_game_screen) => {
                    self.next_game_mode = Some(next_game_screen);
                }
                UiEvent::SurveySurroundings => {
                    if self.resources.get("energy").unwrap().cur_val >= 100.0 {
                        // TODO: Abstract cost of surveying
                        self.resources.get_mut("energy").unwrap().add_or_max(-100.0);
                        info!("Surveying Surroundings...");
                        self.survey_surroundings();
                    }
                }
            }
        }
    }

    pub fn process_frame(&mut self) {
        // Started new embark! Do logic required to possibly build new location
        //   and establish state
        match self.next_game_mode {
            None => {}
            Some(screen) => {
                match screen {
                    GameScreen::_Title => todo!(),
                    GameScreen::Idle => {
                        info!("Going back to idle...");
                        self.next_game_mode = None;
                        self.cur_location = Location::AtBase;

                        self.embark_state = EmbarkState::default();

                        self.game_mode = GameScreen::Idle;
                    }
                    GameScreen::Embark => {
                        info!("Beginning embark...");
                        self.next_game_mode = None;
                        self.cur_location = self.next_location.unwrap();
                        self.next_location = None;

                        let x: f32;
                        let y: f32;
                        if let Location::Embark(params) = self.cur_location {
                            x = params.dims.x / 2.0;
                            y = params.dims.y / 2.0;
                        } else {
                            todo!("Resolve disagreement between Gamescreen::Embark and Location::Embark")
                        }

                        self.embark_state.player_x = x as u32;
                        self.embark_state.player_y = y as u32;

                        self.embark_state.tilemap = Some(self.generate_tilemap());

                        self.game_mode = GameScreen::Embark;
                    }
                }
            }
        }

        if let GameScreen::Embark = self.game_mode {
            let new_x = if let Some(del_x) = self.embark_state.del_x {
                self.embark_state.del_x = None;
                self.embark_state
                    .player_x
                    .checked_add_signed(del_x as i32)
                    .unwrap()
            } else {
                self.embark_state.player_x
            };
            let new_y = if let Some(del_y) = self.embark_state.del_y {
                self.embark_state.del_y = None;
                self.embark_state
                    .player_y
                    .checked_add_signed(del_y as i32)
                    .unwrap()
            } else {
                self.embark_state.player_y
            };

            if let Some(tilemap) = &self.embark_state.tilemap {
                let tile_index: usize =
                    (new_y * (tilemap.width as u32) + new_x).try_into().unwrap();
                if let Tile::Empty = tilemap.tiles[tile_index] {
                    //TODO: Check if new position hits a wall, and potentially deny update
                    self.embark_state.player_x = new_x;
                    self.embark_state.player_y = new_y;
                }
            } else {
                todo!()
            }
        }

        // Process the idle tick
        let cur_time = get_time();
        if (cur_time - self.last_tick) >= self.tick_duration {
            self.idle_tick();

            // Set last tick time
            self.last_tick = cur_time;
        }
    }
}
