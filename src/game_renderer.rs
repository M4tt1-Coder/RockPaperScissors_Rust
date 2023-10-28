///The function container of the application.
/// 
/// Provides the methods for the library to creare the application.

//general using statements

//picture loading modul import
use crate::pictures::{show_paper_picture, show_rock_picture, show_scissors_picture};

//eframe import
use eframe::{
    egui::{
        popup, vec2, AboveOrBelow, Button, Color32, Direction, Label, Layout, RichText, Spinner, Ui,
    },
    Frame,
};

//to select a random integer
use rand::Rng;

//enum and structs of the crate
use crate::{Choice, Game, PlayingMode, Winner};

/// A function to display the game information in a header element
/// 
/// Takes in the UI struct to design the Ui of the program and the local game struct for it's 
/// information.
/// 
/// Must be called before the main body part -> important for egui to render the different containers 
pub fn render_header_content(ui: &mut Ui, game: &mut Game) {
    //displays the elements not vertically but horizontally
    ui.horizontal(|ui| {
        //Show the round the players are currently in
        let round_text = format!("Round: {}", game.round);
        ui.label(RichText::new(round_text).color(Color32::WHITE));
        ui.add_space(15.);

        //winner status
        //checks if there is a winner, meant by '... != Winner::Default'
        if game.winner != Winner::Default && game.winner != Winner::Draw {
            //displays messages according to the winner
            if game.winner == Winner::Player1 {
                let winner_text = format!("Winner: Player 1");
                ui.label(RichText::new(winner_text).color(Color32::WHITE));
            } else if game.winner == Winner::Player2 {
                let winner_text = format!("Winner: Player 2");
                ui.label(RichText::new(winner_text).color(Color32::WHITE));
            }
        } else {
            //gives the information that nobody won or still hasn't won
            ui.label(RichText::new("Winner: Unknown").color(Color32::WHITE));
        }
        ui.add_space(15.);

        //render the spinner which indicates that the game isn't over
        if !game.finished {
            ui.label(
                RichText::new("Playing ... ")
                    .italics()
                    .color(Color32::LIGHT_YELLOW),
            );
            //ui.add(Label::new("Playing ... "));
            ui.add(Spinner::new().color(Color32::RED));
        }
    });
}

/// A function which asks the user by clicking on a button for the game mode
/// 
/// Depending on the game mode choices of the user the game mode enum will be set in the game struct.
/// 
/// Needs to be set or the game won't continue!
pub fn render_mode_selection(ui: &mut Ui, game: &mut Game, frame: &mut Frame) {
    //Depending on the space that is needed for the content of the function -> set the screen size of the app
    //have to set for every function individually
    frame.set_window_size(vec2(285., 100.));

    //horizontally aligned elements to get the game mode 
    ui.horizontal(|ui| {
        //one button for user VS user - mode
        if ui
            .add_sized(
                (50., 50.),
                Button::new(RichText::new("User VS User").color(Color32::WHITE)),
            )
            .clicked()
        {
            //sets game mode to user vs user
            game.playing_mode = PlayingMode::UserVsUser;
        }
        ui.add_space(10.);

        //between a "OR" label
        ui.add_sized(
            (40., 40.),
            Label::new(RichText::new("OR").color(Color32::WHITE)),
        );
        ui.add_space(10.);

        //one buttom for user VS computer - mode
        if ui
            .add_sized(
                (50., 50.),
                Button::new(RichText::new("User VS Computer").color(Color32::WHITE)),
            )
            .clicked()
        {
            //sets game mode to user vs computer
            game.playing_mode = PlayingMode::UserVsComputer;
        }
    });
}

//ask for the users's game choice

/// A function that asks for the two players choices.
/// 
/// First shows which player's turn it is to decide ...
/// then gives the opportunity to click a button for the choice-making.
/// 
/// Takes in as parameter: current game instance to change the player's default choices;
/// Ui struct for displaying the content; frame for screen sizing
///case [USER] vs [USER]
pub fn user_vs_user(ui: &mut Ui, game: &mut Game, frame: &mut Frame) {
    //setting screen size for this function
    frame.set_window_size(vec2(285., 160.));

    //label for the asking of the user's input
    ui.horizontal(|ui| {
        if game.player_1_choice == Choice::Default {
            ui.label(RichText::new("Make a choice player one ... ").color(Color32::WHITE));
        } else if game.player_2_choice == Choice::Default {
            ui.label(RichText::new("Make a choice player two ... ").color(Color32::WHITE));
        }
    });
    ui.add_space(30.);

    //three buttons with rock or paper or scissors
    ui.horizontal(|ui| {
        //rock button
        if ui
            .add_sized(
                (60., 60.),
                Button::new(RichText::new("Rock").color(Color32::WHITE)),
            )
            .clicked()
        {
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Rock;
            } else if game.player_2_choice == Choice::Default {
                game.player_2_choice = Choice::Rock;
            }
        }

        //paper button
        if ui
            .add_sized(
                (60., 60.),
                Button::new(RichText::new("Paper").color(Color32::WHITE)),
            )
            .clicked()
        {
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Paper;
            } else if game.player_2_choice == Choice::Default {
                game.player_2_choice = Choice::Paper;
            }
        }

        //scissors button
        if ui
            .add_sized(
                (60., 60.),
                Button::new(RichText::new("Scissors").color(Color32::WHITE)),
            )
            .clicked()
        {
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Scissors;
            } else if game.player_2_choice == Choice::Default {
                game.player_2_choice = Choice::Scissors;
            }
        }
    });

    //when all choices were made the game should continue to result page
    if game.player_1_choice != Choice::Default && game.player_2_choice != Choice::Default {
        game.choices_made = true;
    }
}

/// This function asks for the simple choice of one player.
/// It sets the player 2 choice to what ever the engine generated for an integer.
/// 
/// The elements are all again horizontal aligned.
/// 
/// The generated integer borders are 0 to 3.
///case [USER] vs [COMPUTER]
pub fn user_vs_computer(ui: &mut Ui, game: &mut Game, frame: &mut Frame) {
    //sceen-sizing
    frame.set_window_size(vec2(285., 160.));

    //label for the asking of the user's input
    ui.horizontal(|ui| {
        if game.player_1_choice == Choice::Default {
            ui.label(RichText::new("Make a choice player ... ").color(Color32::LIGHT_RED));
        }
    });
    ui.add_space(30.);

    //three buttons with rock or paper or scissors
    ui.horizontal(|ui| {
        //rock button
        if ui
            .add_sized(
                (60., 60.),
                Button::new(RichText::new("Rock").color(Color32::LIGHT_GRAY)),
            )
            .clicked()
        {
            //when player 1 choice is default state then set to rock, etc.
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Rock;
            }
        }

        //paper button
        if ui
            .add_sized(
                (60., 60.),
                Button::new(RichText::new("Paper").color(Color32::WHITE)),
            )
            .clicked()
        {
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Paper;
            }
        }

        //scissors button
        if ui
            .add_sized(
                (60., 60.),
                Button::new(RichText::new("Scissors").color(Color32::WHITE)),
            )
            .clicked()
        {
            if game.player_1_choice == Choice::Default {
                game.player_1_choice = Choice::Scissors;
            }
        }
    });

    //set computer answer as 'player two'
    //generator instance
    let mut rng = rand::thread_rng();
    //generated value
    let rand_index = rng.gen_range(0..3);
    //see in statements for index logic -> what resembles what
    if rand_index == 0 {
        game.player_2_choice = Choice::Rock;
    } else if rand_index == 1 {
        game.player_2_choice = Choice::Paper;
    } else if rand_index == 2 {
        game.player_2_choice = Choice::Scissors;
    }

    //when all choices were made the game should continue to result page
    if game.player_1_choice != Choice::Default && game.player_2_choice != Choice::Default {
        game.choices_made = true;
    }
}

//source for adding pictures with egui or in egui: -> https://github.com/emilk/egui/blob/c69fe941afdea5ef6f3f84ed063554500b6262e8/eframe/examples/image.rs

//result screen for all cases
/// That method depending on the game results the checker made displays the gaming end state in the current round.
/// 
/// Gives information about who won the game and how he won meaning again showing which choices were made by the players or the computer.
/// It shows the symbols via picture.
/// 
/// Calls a helper function to optimize readablity.
/// 
/// Takes in: ...
/// - game => the current game object
/// - ui => ui struct to display content
/// - frame => frame to display sceensize
pub fn render_result(ui: &mut Ui, game: &mut Game, frame: &mut Frame) {
    //sceen-sizing
    frame.set_window_size(vec2(310., 200.));

    //show a message with who won the game
    ui.horizontal(|ui| {
        //attemp to show the message in the middle of the window
        ui.with_layout(
            Layout::centered_and_justified(Direction::RightToLeft),
            |ui| {
                if game.winner == Winner::Draw {
                    ui.label(RichText::new("The game ends in a draw!").color(Color32::WHITE));
                } else if game.winner == Winner::Player1 {
                    ui.label(RichText::new("Player 1 wins!").color(Color32::WHITE));
                } else if game.winner == Winner::Player2 {
                    ui.label(RichText::new("Player 2 wins!").color(Color32::WHITE));
                }
            },
        );
    });
    ui.add_space(30.);

    //just show the choices of every again
    show_player_choices(&game.player_1_choice, &game.player_2_choice, ui);
}

//show a popup to ask the player to start a new game
/// After the game ended the game asks for another round.
/// 
/// Shows the gamer a toggle button the he start a new game.
/// 
/// User just clicks on "Yes" or "No" to start a new game or to do nothing.
/// 
/// When "yes" of course calls a reset function -> just sets all important properties to default.
pub fn display_new_game_popup(ui: &mut Ui, frame: &mut Frame, game: &mut Game) {
    //screen-sizing
    frame.set_window_size(vec2(285., 252.));

    //prepare variables for showing popup
    //button instance
    ui.horizontal(|ui| {
        ui.with_layout(
            Layout::centered_and_justified(Direction::RightToLeft),
            |ui| {
                let trigger_button = ui.add_sized(
                    (70., 40.),
                    Button::new(RichText::new("Play Again!").color(Color32::WHITE)),
                );

                //create id for popup
                let popup_id = ui.make_persistent_id("again_popup");
                //check if button was clicked
                if trigger_button.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                }

                //set which cover mode the popup should have
                let above = AboveOrBelow::Above;

                //the popup to show
                popup::popup_above_or_below_widget(ui, popup_id, &trigger_button, above, |ui| {
                    //normal question header
                    ui.horizontal(|ui| {
                        //ui.add(Label::new("Do you want to play again?"));
                        ui.label(
                            RichText::new("Do you want to play again?")
                                .size(15.)
                                .color(Color32::WHITE),
                        )
                    });

                    //two buttons to answer the question
                    ui.horizontal(|ui| {
                        //button for answer yes
                        if ui
                            .add_sized(
                                (40., 30.),
                                Button::new(RichText::new("Yes").color(Color32::WHITE)),
                            )
                            .clicked()
                        {
                            reset_game(game);
                        }
                        ui.add_space(50.);

                        //button for answer no
                        if ui
                            .add_sized(
                                (40., 30.),
                                Button::new(RichText::new("No").color(Color32::WHITE)),
                            )
                            .clicked()
                        {
                            return;
                        }
                    });
                });
            },
        );
    });
}

//_________________
//HELPER functions

//reset the game properties
/// The function is not public.
/// It's purpose is to reset the game, not more.
/// 
/// Takes in the game instance.
fn reset_game(game: &mut Game) {
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
/// The function is not public.
/// It should show all pictures according to the players choices.
/// 
/// The elements are horizontally aligned. 
/// 
/// Takes in:
/// - user_one_choice = player 1 choice
/// - user_two_choice = player 2 choice
/// - ui = egui ui struct -> display widgets
fn show_player_choices(user_one_choice: &Choice, user_two_choice: &Choice, ui: &mut Ui) {
    ui.horizontal(|ui| {
        //render user 1 choice
        if user_one_choice == &Choice::Rock {
            show_rock_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Rock"));
        } else if user_one_choice == &Choice::Paper {
            show_paper_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Paper"));
        } else if user_one_choice == &Choice::Scissors {
            show_scissors_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Scissors"));
        }
        ui.add_space(10.);

        //[vs] text
        ui.label(RichText::new("VS").color(Color32::YELLOW));
        ui.add_space(10.);

        //render user 2 choice
        if user_two_choice == &Choice::Rock {
            show_rock_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Rock"));
        } else if user_two_choice == &Choice::Paper {
            show_paper_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Paper"));
        } else if user_two_choice == &Choice::Scissors {
            show_scissors_picture(ui);
            //ui.add_sized((50.,50.), Label::new("Scissors"));
        }
    });
}
