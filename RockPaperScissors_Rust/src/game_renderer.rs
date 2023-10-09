use eframe::{egui::{Ui, Label, Spinner, Button, vec2, Image},egui, epaint::Color32, Frame};
use rand::Rng;


use crate::{Game, Winner, PlayingMode, Choice};

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
    frame.set_window_size(vec2(285., 100.));
    
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

//ask for the users's game choice 

//case [USER] vs [USER]
pub fn user_vs_user(ui: &mut Ui, game: &mut Game, frame: &mut Frame){
    frame.set_window_size(vec2(285., 160.));

    //label for the asking of the user's input
    ui.horizontal(|ui|{
        if game.player_1_choice == Choice::Default {
            ui.add(Label::new("Make a choice player one ... "));
        }else if game.player_2_choice == Choice::Default {
            ui.add(Label::new("Make a choice player two ... "));
        }
    });
    ui.add_space(30.);    

    //three buttons with rock or paper or scissors
    ui.horizontal(|ui|{
        //rock button
        if ui.add_sized((60., 60.), Button::new("Rock")).clicked(){
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Rock;
            }else if game.player_2_choice == Choice::Default {
                game.player_2_choice = Choice::Rock;
            }
        }

        //paper button
        if ui.add_sized((60., 60.), Button::new("Paper")).clicked(){
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Paper;
            }else if game.player_2_choice == Choice::Default {
                game.player_2_choice = Choice::Paper;
            }
        }

        //scissors button
        if ui.add_sized((60., 60.), Button::new("Scissors")).clicked(){
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Scissors;
            }else if game.player_2_choice == Choice::Default {
                game.player_2_choice = Choice::Scissors;
            }
        }
    });

    //when all choices were made the game should continue to result page
    if game.player_1_choice != Choice::Default && game.player_2_choice != Choice::Default{
        game.choices_made = true;
    }
}

//case [USER] vs [COMPUTER]
pub fn user_vs_computer(ui: &mut Ui, game: &mut Game, frame: &mut Frame){
    frame.set_window_size(vec2(285., 160.));

    //label for the asking of the user's input
    ui.horizontal(|ui|{
        if game.player_1_choice == Choice::Default {
            ui.add(Label::new("Make a choice player ... "));
        }
    });
    ui.add_space(30.);    

    //three buttons with rock or paper or scissors
    ui.horizontal(|ui|{
        //rock button
        if ui.add_sized((60., 60.), Button::new("Rock")).clicked(){
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Rock;
            }
        }

        //paper button
        if ui.add_sized((60., 60.), Button::new("Paper")).clicked(){
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Paper;
            }
        }

        //scissors button
        if ui.add_sized((60., 60.), Button::new("Scissors")).clicked(){
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Scissors;
            }
        }
    });

    //set computer answer as 'player two'
    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0..2);
    if rand_index == 0{
        game.player_2_choice = Choice::Rock;
    }else if rand_index == 1{
        game.player_2_choice = Choice::Paper;
    }else if rand_index == 2{
        game.player_2_choice = Choice::Scissors;
    }

    //when all choices were made the game should continue to result page
    if game.player_1_choice != Choice::Default && game.player_2_choice != Choice::Default{
        game.choices_made = true;
    }
}

//TODO - add some pictures to the result screen
//-> https://github.com/emilk/egui/blob/c69fe941afdea5ef6f3f84ed063554500b6262e8/eframe/examples/image.rs

//result screen for all cases
pub fn render_result(ui: &mut Ui, game: &mut Game, frame: &mut Frame){
    frame.set_window_size(vec2(500., 200.));
    
    //show a message with who won the game
    ui.horizontal(|ui|{
        if game.winner == Winner::Draw{
            ui.add(Label::new("The game ends in a draw!"));
        }else if game.winner == Winner::Player1{
            ui.add(Label::new("Player 1 wins!"));        
        }else if game.winner == Winner::Player2{
            ui.add(Label::new("Player 2 wins!"));
        }
    });
    ui.add_space(30.);

    //just show the choices of every again 
    show_player_choices(&game.player_1_choice, &game.player_2_choice, ui);
}

//_________________
//HELPER functions

//render the players choice to the result screen
fn show_player_choices(user_one_choice: &Choice, user_two_choice: &Choice, ui: &mut Ui){
    ui.horizontal(|ui|{
        //render user 1 choice
        if user_one_choice == &Choice::Rock {
            ui.add(Image::new(egui::include_image!("../pictures/rock.jpg")));
            ui.add_sized((50.,50.), Label::new("Rock"));
        }else if user_one_choice == &Choice::Paper {
            ui.add_sized((50.,50.), Label::new("Paper"));
        }else if user_one_choice == &Choice::Scissors{
            ui.add_sized((50.,50.), Label::new("Scissors"));
        }
        ui.add_space(20.);

        //[vs] text
        ui.add(Label::new("VS"));

        //render user 2 choice
        if user_two_choice == &Choice::Rock {
            ui.add_sized((50.,50.), Label::new("Rock"));
        }else if user_two_choice == &Choice::Paper {
            ui.add_sized((50.,50.), Label::new("Paper"));
        }else if user_two_choice == &Choice::Scissors{
            ui.add_sized((50.,50.), Label::new("Scissors"));
        }
    });
}

