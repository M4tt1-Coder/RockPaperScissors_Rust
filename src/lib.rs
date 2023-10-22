pub mod checker;
pub mod game_renderer;
pub mod pictures;

use eframe::{
    egui::{CentralPanel, TopBottomPanel},
    App,
};

///Using statement and entry point for UI elements
use game_renderer::{
    display_new_game_popup, render_header_content, render_mode_selection, render_result,
    user_vs_computer, user_vs_user,
};

///Determines who is currently the game winner
use checker::who_is_the_winner;
///To represent the single playing choice
/// Rock -> 0
/// Paper -> 1
/// Scissors -> 2

///Enum for the made choices of the player
/// 
///you can assign it like this
/// 
/// let choice = Choice::Default;
#[derive(PartialEq, Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
    Default,
}

///Enum to set the game mode
/// 
/// Declaration: 
/// 
/// let game_mode = PlayingMode::UserVsComputer;
#[derive(PartialEq, Debug)]
pub enum PlayingMode {
    UserVsUser,
    UserVsComputer,
    NotSet,
}

///Enum represents winner which was set in the checker
/// 
/// Declaration: 
/// 
/// let winner = Winner::Player1;
#[derive(PartialEq, Debug)]
pub enum Winner {
    Player1,
    Player2,
    Draw,
    Default,
}

///The game struct ...
/// 
/// ...includes all necessary information as properties to create the workflow of the game.
pub struct Game {
    pub round: i8,//counts the round which the player played
    pub winner: Winner, // competitor 1 = 0 AND competitor 2 = 1 AND draw = 3
    pub player_1_choice: Choice,//choice from the first user !always a human!
    pub player_2_choice: Choice,//decision from player 2 !can be the computer -> depends on the gamemode!
    pub choices_made: bool,//boolean to control the program's logic
    pub finished: bool,//stops the game spinner on the top right corner; prevents further processing in choice making if all choices were made
    pub playing_mode: PlayingMode,//represents the current mode of the game
}

///Implementations for the game
/// 
/// "new" => for the creating of a new game
impl Game {
    pub fn new() -> Game {
        Game {
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

///Apply the App trait from eframe to the game struct to use it for the framework
/// App trait: https://docs.rs/eframe/latest/eframe/
/// 
/// Does all the important things to manipulate the rendering UI
impl App for Game {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    ///Endpoint where all components are rendered
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        //render header on every page
        TopBottomPanel::top("game_information").show(ctx, |ui| {
            render_header_content(ui, self);
        });
        //user can select what mode he wants to play
        if self.playing_mode == PlayingMode::NotSet {
            CentralPanel::default().show(ctx, |ui| {
                render_mode_selection(ui, self, frame);
            });

        //render page for user vs user mode
        } else if self.playing_mode == PlayingMode::UserVsUser && !self.choices_made {
            CentralPanel::default().show(ctx, |ui| {
                user_vs_user(ui, self, frame);
            });

        //render page for user vs computer mode
        } else if self.playing_mode == PlayingMode::UserVsComputer && !self.choices_made {
            CentralPanel::default().show(ctx, |ui| {
                user_vs_computer(ui, self, frame);
            });

        //show the result of the current game
        } else if self.choices_made {
            //declare how is the winner
            who_is_the_winner(self);

            CentralPanel::default().show(ctx, |ui| {
                render_result(ui, self, frame);

                ui.add_space(10.);

                //ask for another game
                if self.winner != Winner::Default {
                    display_new_game_popup(ui, frame, self);
                }
            });
        }
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}
