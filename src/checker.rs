//using statements
use crate::{Game, Choice, Winner};
pub fn who_is_the_winner(game: &mut Game){
    //check out of player one's sight
    
    //user 1 has chosen rock
    if game.player_1_choice == Choice::Rock{
        //... vs rock (from player 2)
        if game.player_2_choice == Choice::Rock{
            game.winner = Winner::Draw;
        }

        // ... vs paper
        if game.player_2_choice == Choice::Paper{
            game.winner = Winner::Player2;
        }

        // ... vs scissors
        if game.player_2_choice == Choice::Scissors{
            game.winner = Winner::Player1;
        }
    }

    //user 1 has chosen paper
    if game.player_1_choice == Choice::Paper{
        // ... vs rock
        if game.player_2_choice == Choice::Rock{
            game.winner = Winner::Player1;
        }

        // ... vs paper
        if game.player_2_choice == Choice::Paper{
            game.winner = Winner::Draw;
        }

        // ... vs scissors
        if game.player_2_choice == Choice::Scissors{
            game.winner = Winner::Player2;
        }
    }

    //user 1 has chosen scissors
    if game.player_1_choice == Choice::Scissors{
        // ... vs rock
        if game.player_2_choice == Choice::Rock{
            game.winner = Winner::Player2;
        }

        // ... vs paper
        if game.player_2_choice == Choice::Paper{
            game.winner = Winner::Player1;
        }

        // ... vs scissors
        if game.player_2_choice == Choice::Scissors{
            game.winner = Winner::Draw;
        }
    }

    //set the finished state
    if game.winner != Winner::Default {
        game.finished = true;
    }
}