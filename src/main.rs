use macroquad::prelude::*;

mod dice;
use dice::{roll_dice, draw_dice_dots};
mod game;
use game::{Game, GameState, Player};

#[macroquad::main("Rusty Dice - Hello World")]
async fn main() {
    let mut current_dice_value = 6;
    let mut last_roll_time = 0.0;

    // Example: create a game with 2 players
    let mut game = Game::new(2);

    loop {
        clear_background(WHITE);

        // Handle input
        if is_key_pressed(KeyCode::Space) {
            current_dice_value = roll_dice();
            last_roll_time = get_time();
        }

        // Draw title
        draw_text(
            "Rusty Dice",
            screen_width() / 2.0 - 80.0,
            screen_height() / 2.0 - 80.0,
            40.0,
            BLACK,
        );

        // Draw instructions
        draw_text(
            "Press SPACE to roll!",
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 - 40.0,
            20.0,
            DARKGRAY,
        );

        // Draw current dice value
        draw_text(
            &format!("Value: {}", current_dice_value),
            screen_width() / 2.0 - 40.0,
            screen_height() / 2.0 + 180.0,
            24.0,
            BLACK,
        );

        // Draw dice background
        draw_rectangle(
            screen_width() / 2.0 - 50.0,
            screen_height() / 2.0 + 60.0,
            100.0,
            100.0,
            GRAY,
        );

        // Draw dice dots based on current value
        let dice_x = screen_width() / 2.0;
        let dice_y = screen_height() / 2.0 + 110.0;
        draw_dice_dots(dice_x, dice_y, current_dice_value);

        // Add a small animation effect when rolling
        if get_time() - last_roll_time < 0.1 {
            draw_text(
                "Rolling...",
                screen_width() / 2.0 - 40.0,
                screen_height() / 2.0 + 220.0,
                16.0,
                RED,
            );
        }

        next_frame().await;
    }
} 