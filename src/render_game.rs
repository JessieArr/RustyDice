use macroquad::prelude::*;
use crate::game::{Game, Action, PlayerAction};
use crate::dice::draw_dice_dots;

pub struct RenderState {
    pub selected_dice_count: u8,
    pub selected_face_value: u8,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            selected_dice_count: 1,
            selected_face_value: 1,
        }
    }
}

pub fn render_game(game: &Game, render_state: &mut RenderState, dice_revealed: bool) -> Option<PlayerAction> {
    clear_background(WHITE);

    // Check if there's a winner
    if let Some(winner_id) = game.winner {
        render_winner_screen(game, winner_id);
        return None;
    }

    // Handle mouse input for UI controls
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);
        
        // Call button (only enabled when bets exist)
        let call_button_rect = Rect::new(50.0, screen_height() - 100.0, 120.0, 40.0);
        if call_button_rect.contains(mouse_pos) && !game.bets.is_empty() {
            return Some(PlayerAction {
                action: Action::Call,
                bet: None,
            });
        }
        
        // Bet button
        let bet_button_rect = Rect::new(200.0, screen_height() - 100.0, 120.0, 40.0);
        if bet_button_rect.contains(mouse_pos) {
            return Some(PlayerAction {
                action: Action::Bet,
                bet: Some((render_state.selected_dice_count, render_state.selected_face_value)),
            });
        }
        
        // Dice count dropdown
        let dice_dropdown_rect = Rect::new(350.0, screen_height() - 100.0, 80.0, 40.0);
        if dice_dropdown_rect.contains(mouse_pos) {
            render_state.selected_dice_count = (render_state.selected_dice_count % 20) + 1; // Cycle through 1-20
        }
        
        // Face value dropdown
        let face_dropdown_rect = Rect::new(450.0, screen_height() - 100.0, 80.0, 40.0);
        if face_dropdown_rect.contains(mouse_pos) {
            render_state.selected_face_value = (render_state.selected_face_value % 6) + 1; // Cycle through 1-6
        }
    }

    render_game_ui(game, render_state, dice_revealed);
    None
}

fn render_winner_screen(game: &Game, winner_id: u8) {
    // Display winner screen
    draw_text(
        "Game Over!",
        screen_width() / 2.0 - 80.0,
        screen_height() / 2.0 - 100.0,
        50.0,
        BLACK,
    );
    
    draw_text(
        &format!("Winner: {}", game.player_names[winner_id as usize]),
        screen_width() / 2.0 - 120.0,
        screen_height() / 2.0 - 40.0,
        30.0,
        GREEN,
    );
    
    // Restart button
    let restart_button_rect = Rect::new(screen_width() / 2.0 - 80.0, screen_height() / 2.0 + 20.0, 160.0, 50.0);
    draw_rectangle(restart_button_rect.x, restart_button_rect.y, restart_button_rect.w, restart_button_rect.h, BLUE);
    draw_text(
        "Restart Game",
        screen_width() / 2.0 - 60.0,
        screen_height() / 2.0 + 35.0,
        24.0,
        WHITE,
    );
}

fn render_game_ui(game: &Game, render_state: &RenderState, dice_revealed: bool) {
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
    
    // Draw dice visibility status
    let visibility_text = if dice_revealed {
        "Dice Revealed - All players can see all dice"
    } else {
        "Dice Hidden - Only you can see your own dice"
    };
    let visibility_color = if dice_revealed { GREEN } else { ORANGE };
    draw_text(
        visibility_text,
        screen_width() / 2.0 - 150.0,
        95.0,
        16.0,
        visibility_color,
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
            
            // Only show dice dots if:
            // 1. It's the current player (player 0) - they can always see their own dice
            // 2. Dice have been revealed after a call action
            if player == 0 || dice_revealed {
                draw_dice_dots(dice_x, dice_y, game.player_dice[player][die]);
            } else {
                // Draw question marks for hidden dice
                draw_text(
                    "?",
                    dice_x - 8.0,
                    dice_y + 8.0,
                    24.0,
                    BLACK,
                );
            }
        }
    }

    // Draw betting history
    if !game.bets.is_empty() {
        let bet_list_offset = screen_height() - 600.0;
        draw_text(
            "Betting History:",
            50.0,
            bet_list_offset,
            20.0,
            BLACK,
        );
        
        for (i, bet) in game.bets.iter().enumerate() {
            let (player, dice_count, face_value) = bet;
            draw_text(
                &format!("Player {}: {} dice showing {}", player + 1, dice_count, face_value),
                50.0,
                bet_list_offset + ((1.0 + i as f32) * 20.0),
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
        &format!("{}", render_state.selected_dice_count),
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
        &format!("{}", render_state.selected_face_value),
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
}

pub fn handle_restart_click() -> bool {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);
        
        let restart_button_rect = Rect::new(screen_width() / 2.0 - 80.0, screen_height() / 2.0 + 20.0, 160.0, 50.0);
        if restart_button_rect.contains(mouse_pos) {
            return true;
        }
    }
    false
} 