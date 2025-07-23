use crate::game::{Game, PlayerAction, Action};
use macroquad::rand::gen_range;

pub fn ai_decide_action(game: &Game) -> PlayerAction {
    // If there is a bet, call
    if !game.bets.is_empty() {
        PlayerAction {
            action: Action::Call,
            bet: None,
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