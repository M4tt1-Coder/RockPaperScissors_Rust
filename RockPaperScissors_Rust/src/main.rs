//using statements
use eframe::{Error, egui};
use RockPaperScissors_Rust::Game;

fn main() -> Result<(), Error> {
    //give an app instance
    let game = Game::new();

    //set the name of the application
    let app_name = "Rock Paper Scissors";

    //declare the app settings
    let options = eframe::NativeOptions{
        min_window_size: Some(egui::vec2(250., 250.)),
        max_window_size: Some(egui::vec2(400., 400.)),
        ..eframe::NativeOptions::default()
    };

    //run the game
    eframe::run_native(app_name,
        options,
        Box::new(|_cc| Box::new(game)))
}