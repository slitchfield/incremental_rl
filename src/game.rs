use macroquad::prelude::*;

pub enum UiEvent {
    BuyCircle(f32),
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
}

pub struct Resources {
    pub energy: Resource,
    pub circles: f32,
    pub squares: f32,
}

pub struct EmbarkState {
    _width: u32,
    _height: u32,
    _player_x: u32,
    _player_y: u32,
}

impl Default for EmbarkState {
    fn default() -> Self {
        EmbarkState {
            _width: 100,
            _height: 100,
            _player_x: 100 / 2,
            _player_y: 100 / 2,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Location {
    AtBase,
    Embark(u32),
}

impl Location {
    fn generate_location(_state: &GameState) -> Location {
        Location::Embark(50)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameScreen {
    _Title,
    Idle,
    Embark(Location),
}

pub struct GameState {
    pub exit_requested: bool,

    pub screen_width: f32,
    pub screen_height: f32,

    pub last_tick: f64,
    pub tick_duration: f64,

    pub game_mode: GameScreen,
    pub next_game_mode: Option<GameScreen>,

    pub resources: Resources,
    pub cur_location: Location,
    pub scouted_locations: Vec<Location>,

    // Embark specific state
    pub _embark_state: EmbarkState,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            exit_requested: false,

            screen_width: 1920.0,
            screen_height: 1080.0,

            last_tick: get_time(),
            tick_duration: 1.0f64,

            game_mode: GameScreen::Idle,
            next_game_mode: None,

            cur_location: Location::AtBase,
            scouted_locations: vec![],
            resources: Resources {
                energy: Resource {
                    cur_val: 100.0,
                    max_val: 100.0,
                },
                circles: 0.0,
                squares: 0.0,
            },

            _embark_state: EmbarkState::default(),
        }
    }
}

impl GameState {
    pub fn idle_tick(&mut self) {
        // Placeholder, increment squares by circle count
        self.resources.squares += self.resources.circles;

        // Handle Energy
        //   Currently, when at base, recharge 1.0 unit per tick
        match self.cur_location {
            Location::AtBase => {
                self.resources.energy.add_or_max(1.0);
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

    pub fn process_keypress(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::I => {
                self.next_game_mode = Some(GameScreen::Idle);
            }
            KeyCode::Q => {
                self.exit_requested = true;
            }
            _ => {
                warn!("Unhandled keycode: {:?}", keycode);
            }
        }
    }

    pub fn process_inputs(&mut self, events: &mut Vec<UiEvent>) {
        while let Some(event) = events.pop() {
            match event {
                UiEvent::BuyCircle(new_delta) => {
                    self.resources.circles += new_delta;
                }
                UiEvent::EmbarkLocation(location) => {
                    // Switch to embark/roguelike mode
                    self.next_game_mode = Some(GameScreen::Embark(location));
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
                    if self.resources.energy.cur_val >= 100.0 {
                        // TODO: Abstract cost of surveying
                        self.resources.energy.add_or_max(-100.0);
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
            Some(screen) => match screen {
                GameScreen::_Title => todo!(),
                GameScreen::Idle => {
                    info!("Going back to idle...");
                    self.next_game_mode = None;
                    self.cur_location = Location::AtBase;
                    self.game_mode = GameScreen::Idle;
                }
                GameScreen::Embark(location) => {
                    info!("Beginning embark...");
                    self.next_game_mode = None;
                    self.cur_location = location;
                    self.game_mode = GameScreen::Embark(self.cur_location);
                }
            },
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
