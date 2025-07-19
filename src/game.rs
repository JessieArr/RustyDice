// Game state and player management for Rusty Dice

use crate::dice::roll_dice;

const MAX_PLAYERS: usize = 8;
const DICE_PER_PLAYER: usize = 5;

pub struct Game {
    pub player_count: u8,
    pub current_player_dice_count: u8,
    pub player_dice: [[u8; DICE_PER_PLAYER]; MAX_PLAYERS],
}

pub fn roll_all_dice(game: &mut Game) {
    for player in 0..game.player_count as usize {
        for die in 0..game.current_player_dice_count as usize {
            game.player_dice[player][die] = roll_dice();
        }
    }
}
