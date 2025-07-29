use crate::game::{Game, PlayerAction, Action};
use macroquad::rand::gen_range;

pub fn ai_decide_action(game: &Game) -> PlayerAction {
    // If there is a bet, decide whether to call or bet
    if !game.bets.is_empty() {
        // Calculate total dice remaining in the game
        let total_dice_remaining: u8 = game.current_player_dice_count.iter().sum();
        
        // Get the last bet
        if let Some((_, bet_dice_count, _)) = game.bets.last() {
            // Only call if the bet exceeds 1/6 of total dice remaining
            if *bet_dice_count as f32 > (total_dice_remaining as f32 / 6.0) {
                PlayerAction {
                    action: Action::Call,
                    bet: None,
                }
            } else {
                // Make a higher bet instead of calling
                let current_bet = game.bets.last().unwrap();
                let (_, current_dice_count, current_face_value) = current_bet;
                
                // Try to increase the dice count or face value
                let new_dice_count = if gen_range(0, 2) == 0 {
                    *current_dice_count + 1
                } else {
                    *current_dice_count
                };
                
                let new_face_value = if new_dice_count == *current_dice_count {
                    (*current_face_value + 1).min(6)
                } else {
                    gen_range(1, 7)
                };
                
                PlayerAction {
                    action: Action::Bet,
                    bet: Some((new_dice_count, new_face_value)),
                }
            }
        } else {
            // Fallback: call if no bet info available
            PlayerAction {
                action: Action::Call,
                bet: None,
            }
        }
    } else {
        // Otherwise, make a random valid bet
        let dice_count = gen_range(1, game.current_player_dice_count[game.current_player as usize] + 1);
        let face_value = gen_range(1, 7);
        PlayerAction {
            action: Action::Bet,
            bet: Some((dice_count, face_value)),
        }
    }
} 