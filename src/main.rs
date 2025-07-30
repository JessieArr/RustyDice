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
    
    // Track timing for AI actions
    let mut last_ai_action_time = std::time::Instant::now();
    
    // Track whether dice should be revealed (after a call action)
    let mut dice_revealed = false;
    let mut dice_revealed_time: Option<std::time::Instant> = None;

    loop {
        // Check if dice should be hidden again after 3 seconds
        if let Some(revealed_time) = dice_revealed_time {
            let current_time = std::time::Instant::now();
            if current_time.duration_since(revealed_time) >= std::time::Duration::from_secs(3) {
                dice_revealed = false;
                dice_revealed_time = None;
            }
        }
        
        // Always render the game
        let action = render_game(&game, &mut render_state, dice_revealed);

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
                                dice_revealed = true; // Reveal dice after a call
                                dice_revealed_time = Some(std::time::Instant::now()); // Start timing
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
                // Check if enough time has passed since last AI action
                let current_time = std::time::Instant::now();
                let time_since_last_action = current_time.duration_since(last_ai_action_time);
                
                if time_since_last_action >= std::time::Duration::from_millis(1000) {
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
                                    dice_revealed = true; // Reveal dice after a call
                                    dice_revealed_time = Some(std::time::Instant::now()); // Start timing
                                }
                            }
                            game = new_game;
                            last_ai_action_time = current_time; // Update the last action time
                        }
                        Err(e) => {
                            println!("AI error: {}", e);
                            break;
                        }
                    }
                } else {
                    // Not enough time has passed, break out of the AI loop to allow rendering
                    break;
                }
            }
        }

        // Check for restart button click when there's a winner
        if game.winner.is_some() && handle_restart_click() {
            // Reset the game
            game = Game::new();
            roll_all_dice(&mut game);
            render_state = RenderState::new();
            dice_revealed = false; // Reset dice visibility
            dice_revealed_time = None; // Reset timing
        }

        next_frame().await;
    }
} 