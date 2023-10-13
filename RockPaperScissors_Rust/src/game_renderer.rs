use eframe::{egui::{Ui, Label, Spinner, Button, vec2, AboveOrBelow, popup}, epaint::Color32, Frame};
use rand::Rng;
use crate::pictures::{show_paper_picture, show_rock_picture,show_scissors_picture};

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
    frame.set_window_size(vec2(310., 200.));
    
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

//show a popup to ask the player to start a new game
pub fn display_new_game_popup(ui: &mut Ui, frame: &mut Frame, game: &mut Game){
    frame.set_window_size(vec2(285., 252.));

    //prepare variables for showing popup
    //button instance
    let trigger_button = ui.add_sized((70., 40.), Button::new("Play Again"));
    //create id for popup
    let popup_id = ui.make_persistent_id("again_popup");
    //check if button was clicked
    if trigger_button.clicked(){
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }

    //set which cover mode the popup should have
    let above = AboveOrBelow::Above;

    //the popup to show
    popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &trigger_button,
        above,
        |ui| {
            //normal question header
            ui.horizontal(|ui| {
                ui.add(
                    Label::new("Do you want to play again?")
                );
            });

            //two buttons to answer the question
            ui.horizontal(
                |ui|{
                    //button for answer yes
                    if ui.add_sized((40., 30.), Button::new("Yes")).clicked(){
                        reset_game(game);
                    }
                    ui.add_space(50.);

                    //button for answer no
                    if ui.add_sized((40., 30.), Button::new("No")).clicked(){
                        return
                    }
                }
            );
        }
    );
}

//_________________
//HELPER functions

//reset the game properties
fn reset_game(game: &mut Game){
    //increase rounds counter
    game.round += 1;
    //reset winner prop
    game.winner = Winner::Default;
    //set default for choices
    game.player_1_choice = Choice::Default;
    game.player_2_choice = Choice::Default;
    //restart logic booleans
    game.choices_made = false;
    game.finished = false;
    //no playing mode
    game.playing_mode = PlayingMode::NotSet;
}

//render the players choice to the result screen
fn show_player_choices(user_one_choice: &Choice, user_two_choice: &Choice, ui: &mut Ui){
    ui.horizontal(|ui|{
        //render user 1 choice
        if user_one_choice == &Choice::Rock {
            show_rock_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Rock"));
        }else if user_one_choice == &Choice::Paper {
            show_paper_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Paper"));
        }else if user_one_choice == &Choice::Scissors{
            show_scissors_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Scissors"));
        }
        ui.add_space(10.);

        //[vs] text
        ui.add(Label::new("VS"));
        ui.add_space(10.);
        
        //render user 2 choice
        if user_two_choice == &Choice::Rock {
            show_rock_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Rock"));
        }else if user_two_choice == &Choice::Paper {
            show_paper_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Paper"));
        }else if user_two_choice == &Choice::Scissors{
            show_scissors_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Scissors"));
        }
    });
}

