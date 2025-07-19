use macroquad::prelude::*;

mod dice;
use dice::{draw_dice_dots};
mod game;
use game::{Game, roll_all_dice};

#[macroquad::main("Rusty Dice - Hello World")]
async fn main() {
    let mut last_roll_time = 0.0;

    // Create a game with 4 players
    let mut game = Game::new();

    loop {
        clear_background(WHITE);

        // Handle input
        if is_key_pressed(KeyCode::Space) {
            roll_all_dice(&mut game);
            last_roll_time = get_time();
        }

        // Draw title
        draw_text(
            "Rusty Dice - 4 Players",
            screen_width() / 2.0 - 120.0,
            30.0,
            40.0,
            BLACK,
        );

        // Draw instructions
        draw_text(
            "Press SPACE to roll all dice!",
            screen_width() / 2.0 - 120.0,
            70.0,
            20.0,
            DARKGRAY,
        );

        // Draw all players and their dice
        let start_y = 120.0;
        let dice_size = 60.0;
        let spacing = 80.0;
        
        for player in 0..game.player_count as usize {
            let player_y = start_y + (player as f32 * 130.0);
            
            // Draw player name
            draw_text(
                &game.player_names[player],
                screen_width() / 2.0 - 50.0,
                player_y,
                24.0,
                BLACK,
            );
            
            // Draw dice count
            draw_text(
                &format!("Dice: {}", game.current_player_dice_count[player]),
                screen_width() / 2.0 - 30.0,
                player_y + 25.0,
                16.0,
                DARKGRAY,
            );
            
            // Draw dice for this player
            let dice_count = game.current_player_dice_count[player] as usize;
            for die in 0..dice_count {
                let dice_x = screen_width() / 2.0 - (dice_count as f32 * spacing) / 2.0 + (die as f32 * spacing);
                let dice_y = player_y + 60.0;
                
                // Draw dice background
                draw_rectangle(
                    dice_x - dice_size / 2.0,
                    dice_y - dice_size / 2.0,
                    dice_size,
                    dice_size,
                    GRAY,
                );
                
                // Draw dice dots
                draw_dice_dots(dice_x, dice_y, game.player_dice[player][die]);
            }
        }

        // Add a small animation effect when rolling
        if get_time() - last_roll_time < 0.1 {
            draw_text(
                "Rolling...",
                screen_width() / 2.0 - 40.0,
                screen_height() - 50.0,
                16.0,
                RED,
            );
        }

        next_frame().await;
    }
} 