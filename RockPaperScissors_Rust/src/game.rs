//using statements
use eframe::{App, egui::{self, TopBottomPanel, Label, Spinner, Color32}};

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
pub enum Winner{
    Player1,
    Player2,
    Draw,
    Default 
}

pub struct Game{
    round: i8,
    winner: Winner,// competitor 1 = 0 AND competitor 2 = 1 AND draw = 3
    player_1_choice: Choice,
    player_2_choice: Choice,
    finished: bool,
}

impl Game{
    pub fn new() -> Game{
        Game{
            round: 1,
            winner: Winner::Default,
            player_1_choice: Choice::Default,
            player_2_choice: Choice::Default,
            finished: false,
        }
    }
}

impl App for Game{
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        
    }

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("game_information").show(ctx, |ui|{
            ui.horizontal(|ui| {
                
                //Show the round the players are currently in
                let round_text = format!("Round: {}", self.round);
                ui.add(Label::new(round_text));
                ui.add_space(15.);
                
                //winner status
                if self.winner != Winner::Default && self.winner != Winner::Draw{
                    if self.winner == Winner::Player1{
                        
                        let winner_text = format!("Winner: Player 1");
                        ui.add(Label::new(winner_text));
                    
                    }else if self.winner == Winner::Player2{
                        
                        let winner_text = format!("Winner: Player 2");
                        ui.add(Label::new(winner_text));
                    
                    }
                }else{
                    ui.add(Label::new("Winner: Unknown"));
                }
                ui.add_space(15.); 

                //render the spinner which indicates that the game isn't over
                if !self.finished{
                    ui.add(Label::new("Playing ... "));
                    ui.add(Spinner::new().color(Color32::RED));
                }
                
            });
        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}