pub mod game_renderer;

use eframe::{egui::{TopBottomPanel, CentralPanel}, App};

use game_renderer::{render_header_content, render_mode_selection};


///To represent the single playing choice
/// Rock -> 0
/// Paper -> 1
/// Scissors -> 2

pub enum Choice {
    Rock,
    Paper,
    Scissors,
    Default
}


#[derive(PartialEq, Debug)]
pub enum PlayingMode{
    UserVsUser,
    UserVsComputer,
    NotSet
}

#[derive(PartialEq, Debug)]
pub enum Winner{
    Player1,
    Player2,
    Draw,
    Default 
}

pub struct Game{
    pub round: i8,
    pub winner: Winner,// competitor 1 = 0 AND competitor 2 = 1 AND draw = 3
    pub player_1_choice: Choice,
    pub player_2_choice: Choice,
    pub finished: bool,
    pub playing_mode: PlayingMode
}

impl Game{
    pub fn new() -> Game{
        Game{
            round: 1,
            winner: Winner::Default,
            player_1_choice: Choice::Default,
            player_2_choice: Choice::Default,
            finished: false,
            playing_mode: PlayingMode::NotSet,
        }
    }
}

impl App for Game{
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        
    }

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        //render header on every page
        TopBottomPanel::top("game_information").show(ctx, |ui|{
            render_header_content(ui,  self);    
        });
        //user can select what mode he wants to play
        if self.playing_mode == PlayingMode::NotSet {
            CentralPanel::default().show(ctx, |ui| {
                render_mode_selection(ui, self, frame);
            });
        }
        
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}