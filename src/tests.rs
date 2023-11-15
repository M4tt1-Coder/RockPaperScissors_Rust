use crate::{Choice, Game, PlayingMode, Winner};

/// Tests the update / reset functionality of the private game_renderer method 
#[warn(dead_code)]
fn reset_game(game: &mut Game) -> &mut Game {
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

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_game() {
        let update_game:&mut Game = &mut Game{
            round: 1,
            winner: Winner::Player1,
            player_1_choice: Choice::Rock,
            player_2_choice: Choice::Paper,
            choices_made: true,
            finished: true,
            playing_mode: PlayingMode::UserVsComputer,
        };
        let compare_game = &mut Game{
            round: 2,
            winner: Winner::Default,
            player_1_choice: Choice::Default,
            player_2_choice: Choice::Default,
            choices_made: false,
            finished: false,
            playing_mode: PlayingMode::NotSet,
        };

        assert_eq!(compare_game, reset_game(update_game));
    }
}
