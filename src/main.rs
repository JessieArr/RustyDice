use macroquad::prelude::*;

mod dice;
use dice::{draw_dice_dots};
mod game;
use game::{Game, roll_all_dice, take_action, Action, PlayerAction};

#[macroquad::main("Rusty Dice")]
async fn main() {
    let mut selected_dice_count = 1;
    let mut selected_face_value = 1;

    // Create a game with 4 players and roll dice automatically
    let mut game = Game::new();
    roll_all_dice(&mut game);

    loop {
        clear_background(WHITE);

        // Handle mouse input for UI controls
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_pos = Vec2::new(mouse_x, mouse_y);
            
            // Call button (only enabled when bets exist)
            let call_button_rect = Rect::new(50.0, screen_height() - 100.0, 120.0, 40.0);
            if call_button_rect.contains(mouse_pos) && !game.bets.is_empty() {
                let call_action = PlayerAction {
                    action: Action::Call,
                    bet: None,
                };
                match take_action(&game, call_action) {
                    Ok(new_game) => game = new_game,
                    Err(e) => println!("Call error: {}", e),
                }
            }
            
            // Bet button
            let bet_button_rect = Rect::new(200.0, screen_height() - 100.0, 120.0, 40.0);
            if bet_button_rect.contains(mouse_pos) {
                let bet_action = PlayerAction {
                    action: Action::Bet,
                    bet: Some((selected_dice_count, selected_face_value)),
                };
                match take_action(&game, bet_action) {
                    Ok(new_game) => game = new_game,
                    Err(e) => println!("Bet error: {}", e),
                }
            }
            
            // Dice count dropdown
            let dice_dropdown_rect = Rect::new(350.0, screen_height() - 100.0, 80.0, 40.0);
            if dice_dropdown_rect.contains(mouse_pos) {
                selected_dice_count = (selected_dice_count % 20) + 1; // Cycle through 1-20
            }
            
            // Face value dropdown
            let face_dropdown_rect = Rect::new(450.0, screen_height() - 100.0, 80.0, 40.0);
            if face_dropdown_rect.contains(mouse_pos) {
                selected_face_value = (selected_face_value % 6) + 1; // Cycle through 1-6
            }
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
            "Click to bet or call!",
            screen_width() / 2.0 - 100.0,
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

        // Draw betting history
        if !game.bets.is_empty() {
            draw_text(
                "Betting History:",
                50.0,
                screen_height() - 200.0,
                20.0,
                BLACK,
            );
            
            for (i, bet) in game.bets.iter().enumerate() {
                let (player, dice_count, face_value) = bet;
                draw_text(
                    &format!("Player {}: {} dice showing {}", player + 1, dice_count, face_value),
                    50.0,
                    screen_height() - 180.0 + (i as f32 * 20.0),
                    16.0,
                    DARKGRAY,
                );
            }
        }

        // Draw UI controls
        let ui_y = screen_height() - 100.0;
        
        // Call button
        let call_button_color = if game.bets.is_empty() { DARKGRAY } else { BLUE };
        draw_rectangle(50.0, ui_y, 120.0, 40.0, call_button_color);
        draw_text(
            "Call",
            100.0,
            ui_y + 15.0,
            20.0,
            WHITE,
        );
        
        // Bet button
        draw_rectangle(200.0, ui_y, 120.0, 40.0, GREEN);
        draw_text(
            "Bet",
            250.0,
            ui_y + 15.0,
            20.0,
            WHITE,
        );
        
        // Dice count dropdown
        draw_rectangle(350.0, ui_y, 80.0, 40.0, LIGHTGRAY);
        draw_text(
            &format!("{}", selected_dice_count),
            380.0,
            ui_y + 15.0,
            20.0,
            BLACK,
        );
        draw_text(
            "Dice",
            350.0,
            ui_y - 20.0,
            14.0,
            BLACK,
        );
        
        // Face value dropdown
        draw_rectangle(450.0, ui_y, 80.0, 40.0, LIGHTGRAY);
        draw_text(
            &format!("{}", selected_face_value),
            480.0,
            ui_y + 15.0,
            20.0,
            BLACK,
        );
        draw_text(
            "Face",
            450.0,
            ui_y - 20.0,
            14.0,
            BLACK,
        );

        next_frame().await;
    }
} 