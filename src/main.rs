//using statements
use eframe::{egui, Error};
use RockPaperScissors_Rust::Game;

///Set the default game window settings
/// 
/// App name = "RockPaperScissors"
/// 
/// Uses egui, a GUI for RUST: https://docs.rs/egui/latest/egui/

fn main() -> Result<(), Error> {
    //give an app instance
    let game = Game::new();

    //set the name of the application
    let app_name = "Rock Paper Scissors";

    //declare the app settings
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(250., 250.)),
        max_window_size: Some(egui::vec2(400., 400.)),
        ..eframe::NativeOptions::default()
    };

    //run the game
    eframe::run_native(
        app_name,
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(game)
        }),
    )
}

//TODO - refactor comments on general
//TODO - write readme file
//TODO - think of workflow on github