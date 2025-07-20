use macroquad::prelude::*;

mod dice;
mod game;
use game::{Game, take_action, roll_all_dice};
mod render_game;
use render_game::{render_game, handle_restart_click, RenderState};

#[macroquad::main("Rusty Dice")]
async fn main() {
    // Create a game with 4 players and roll dice automatically
    let mut game = Game::new();
    roll_all_dice(&mut game);

    // Create render state for UI controls
    let mut render_state = RenderState::new();

    loop {
        // Render the game and get any player action
        if let Some(action) = render_game(&game, &mut render_state) {
            // Handle the player action
            match take_action(&game, action) {
                Ok(new_game) => game = new_game,
                Err(e) => println!("Action error: {}", e),
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