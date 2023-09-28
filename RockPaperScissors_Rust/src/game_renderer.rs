use eframe::{egui::{Context, Ui, Label, Spinner, Button}, epaint::Color32, Frame};

use crate::{Game, Winner, PlayingMode};

pub fn render_header_content(ui: &mut Ui, game: &mut Game){
    ui.horizontal(|ui| {
        
        //Show the round the players are currently in
        let round_text = format!("Round: {}",game.round);
        ui.add(Label::new(round_text));
        ui.add_space(15.);
        
        //winner status
        if game.winner != Winner::Default && game.winner != Winner::Draw{
            if game.winner == Winner::Player1{
                
                let winner_text = format!("Winner: Player 1");
                ui.add(Label::new(winner_text));
            
            }else if game.winner == Winner::Player2{
                
                let winner_text = format!("Winner: Player 2");
                ui.add(Label::new(winner_text));
            
            }
        }else{
            ui.add(Label::new("Winner: Unknown"));
        }
        ui.add_space(15.); 

        //render the spinner which indicates that the game isn't over
        if !game.finished{
            ui.add(Label::new("Playing ... "));
            ui.add(Spinner::new().color(Color32::RED));
        }
            
    });
}

//set the window size custumized to the page
pub fn render_mode_selection(ui: &mut Ui, game: &mut Game, frame: &mut Frame) {
    frame.set_window_size(eframe::egui::vec2(285., 100.));
    
    ui.horizontal(|ui| {
        //one button for user VS user - mode 
        if ui.add_sized((50., 50.), Button::new("User VS User")).clicked(){
            game.playing_mode = PlayingMode::UserVsUser;
        }
        ui.add_space(10.);

        //between a "OR" label
        ui.add_sized((40.,40.), Label::new("OR"));
        ui.add_space(10.);

        //one buttom for user VS computer - mode
        if ui.add_sized((50.,  50.), Button::new("User VS Computer")).clicked(){
            game.playing_mode = PlayingMode::UserVsComputer;            
        }
    });
}
