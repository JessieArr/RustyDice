// Game state and player management for Rusty Dice

use crate::dice::roll_dice;

const MAX_PLAYERS: usize = 8;
const DICE_PER_PLAYER: usize = 5;

pub struct Game {
    pub player_count: u8,
    pub player_names: [String; MAX_PLAYERS],
    pub current_player_dice_count: [u8; MAX_PLAYERS],
    pub player_dice: [[u8; DICE_PER_PLAYER]; MAX_PLAYERS],
}

pub fn roll_all_dice(game: &mut Game) {
    for player in 0..game.player_count as usize {
        let dice_count = game.current_player_dice_count[player] as usize;
        for die in 0..dice_count {
            game.player_dice[player][die] = roll_dice();
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let mut player_names = [(); MAX_PLAYERS].map(|_| String::new());
        for i in 0..4 {
            player_names[i] = format!("Player {}", i + 1);
        }
        
        let current_player_dice_count = [5; MAX_PLAYERS]; // Start with 5 dice each
        
        Self {
            player_count: 4,
            player_names,
            current_player_dice_count,
            player_dice: [[0; DICE_PER_PLAYER]; MAX_PLAYERS],
        }
    }
}
