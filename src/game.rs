// Game state and player management for Rusty Dice

use crate::dice::roll_dice;

const MAX_PLAYERS: usize = 8;
const DICE_PER_PLAYER: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Bet,
    Call,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerAction {
    pub action: Action,
    pub bet: Option<(u8, u8)> // (dice_count, face_value)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub player_count: u8,
    pub player_names: [String; MAX_PLAYERS],
    pub current_player_dice_count: [u8; MAX_PLAYERS],
    pub player_dice: [[u8; DICE_PER_PLAYER]; MAX_PLAYERS],
    pub bets: Vec<(u8, u8, u8)>, // (player_index, dice_count, face_value)
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
            bets: Vec::new(), // Initialize bets array
        }
    }
}

pub fn take_action(game: &Game, player_index: usize, action: PlayerAction) -> Game {
    let mut new_game = game.clone();
    
    match action.action {
        Action::Call => {
            if let Some(_last_bet) = new_game.bets.last() {
                // TODO: Implement call logic - check if the bet was valid
                // For now, just remove the last bet
                new_game.bets.pop();
            }
        }
        Action::Bet => {
            if let Some((dice_count, face_value)) = action.bet {
                // Add the bet to the betting history
                new_game.bets.push((player_index as u8, dice_count, face_value));
            }
        }
    }
    
    new_game
}