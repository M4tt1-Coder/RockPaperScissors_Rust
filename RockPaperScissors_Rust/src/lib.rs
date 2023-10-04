pub mod game_renderer;
pub mod checker;

use eframe::{egui::{TopBottomPanel, CentralPanel, Label}, App};

use game_renderer::{render_header_content, render_mode_selection, user_vs_user, user_vs_computer, render_result};

use checker::who_is_the_winner;
///To represent the single playing choice
/// Rock -> 0
/// Paper -> 1
/// Scissors -> 2


#[derive(PartialEq, Debug)]
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
    pub choices_made: bool,
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
            choices_made: false,
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
        
        //render page for user vs user mode
        }else if self.playing_mode == PlayingMode::UserVsUser && !self.choices_made{
            CentralPanel::default().show(ctx, |ui|{
                user_vs_user(ui, self, frame); 
            });

        //render page for user vs computer mode
        }else if self.playing_mode == PlayingMode::UserVsComputer && !self.choices_made{
            CentralPanel::default().show(ctx, |ui|{
                user_vs_computer(ui, self, frame);                
            });

        //show the result of the current game    
        }else if self.choices_made{
            //declare how is the winner
            who_is_the_winner(self);

            CentralPanel::default().show(ctx, |ui|{
                render_result(ui, self, frame);
            });
        }
        
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}