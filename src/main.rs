use macroquad::prelude::*;

mod dice;
mod game;
use game::{Game, take_action, roll_all_dice};
mod render_game;
use render_game::{render_game, handle_restart_click, RenderState};
mod ai;
use ai::ai_decide_action;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Dice".to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Create a game with 4 players and roll dice automatically
    let mut game = Game::new();
    roll_all_dice(&mut game);

    // Create render state for UI controls
    let mut render_state = RenderState::new();

    loop {
        // Always render the game
        let action = render_game(&game, &mut render_state);

        // Only allow human (player 0) to act when it's their turn and the game is not over
        if game.current_player == 0 && game.winner.is_none() {
            if let Some(action) = action {
                match take_action(&game, &action) {
                    Ok(new_game) => {
                        match action.action {
                            game::Action::Bet => {
                                if let Some((_, dice_count, face_value)) = new_game.bets.last() {
                                    render_state.selected_dice_count = *dice_count;
                                    render_state.selected_face_value = *face_value;
                                }
                            }
                            game::Action::Call => {
                                render_state.selected_dice_count = 1;
                                render_state.selected_face_value = 1;
                            }
                        }
                        game = new_game;
                    }
                    Err(e) => println!("Action error: {}", e),
                }
            }
        } else if game.winner.is_none() {
            // AI takes actions for players 1-3
            while game.current_player != 0 && game.winner.is_none() {
                let ai_action = ai_decide_action(&game);
                match take_action(&game, &ai_action) {
                    Ok(new_game) => {
                        match ai_action.action {
                            game::Action::Bet => {
                                if let Some((_, dice_count, face_value)) = new_game.bets.last() {
                                    render_state.selected_dice_count = *dice_count;
                                    render_state.selected_face_value = *face_value;
                                }
                            }
                            game::Action::Call => {
                                render_state.selected_dice_count = 1;
                                render_state.selected_face_value = 1;
                            }
                        }
                        game = new_game;
                    }
                    Err(e) => {
                        println!("AI error: {}", e);
                        break;
                    }
                }
            }
        }

        // Check for restart button click when there's a winner
        if game.winner.is_some() && handle_restart_click() {
            // Reset the game
            game = Game::new();
            roll_all_dice(&mut game);
            render_state = RenderState::new();
        }

        next_frame().await;
    }
} 