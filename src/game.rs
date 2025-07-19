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
    pub current_player: u8,
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
            current_player: 0, // Start with player 0
            player_names,
            current_player_dice_count,
            player_dice: [[0; DICE_PER_PLAYER]; MAX_PLAYERS],
            bets: Vec::new(), // Initialize bets array
        }
    }
}

pub fn take_action(game: &Game, action: PlayerAction) -> Result<Game, String> {
    let mut new_game = game.clone();
    
    match action.action {
        Action::Call => {
            if let Some(_last_bet) = new_game.bets.last() {
                // TODO: Implement call logic - check if the bet was valid
                // For now, just remove the last bet
                new_game.bets.pop();
            } else {
                return Err("Cannot call when no bets have been made".to_string());
            }
        }
        Action::Bet => {
            if let Some((dice_count, face_value)) = action.bet {
                // Validate bet parameters
                if face_value < 1 || face_value > 6 {
                    return Err("Face value must be between 1 and 6".to_string());
                }
                if dice_count == 0 {
                    return Err("Cannot bet on 0 dice".to_string());
                }
                
                // Check if this bet is higher than the previous bet
                if let Some(last_bet) = new_game.bets.last() {
                    let (_, last_dice_count, last_face_value) = last_bet;
                    if dice_count < *last_dice_count || 
                       (dice_count == *last_dice_count && face_value <= *last_face_value) {
                        return Err("New bet must be higher than the previous bet".to_string());
                    }
                }
                
                // Add the bet to the betting history
                new_game.bets.push((game.current_player as u8, dice_count, face_value));
                // Advance to the next player
                new_game.current_player = (new_game.current_player + 1) % new_game.player_count;
            } else {
                return Err("Bet action requires dice count and face value".to_string());
            }
        }
    }
    
    Ok(new_game)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game() -> Game {
        Game::new()
    }

    #[test]
    fn test_call_without_bets_returns_error() {
        let game = create_test_game();
        let call_action = PlayerAction {
            action: Action::Call,
            bet: None,
        };
        
        let result = take_action(&game, call_action);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot call when no bets have been made");
    }

    #[test]
    fn test_bet_with_invalid_face_value_returns_error() {
        let game = create_test_game();
        let invalid_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 7)), // Face value > 6
        };
        
        let result = take_action(&game, invalid_bet);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Face value must be between 1 and 6");
    }

    #[test]
    fn test_bet_with_zero_face_value_returns_error() {
        let game = create_test_game();
        let invalid_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 0)), // Face value = 0
        };
        
        let result = take_action(&game, invalid_bet);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Face value must be between 1 and 6");
    }

    #[test]
    fn test_bet_with_zero_dice_returns_error() {
        let game = create_test_game();
        let invalid_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((0, 5)), // 0 dice
        };
        
        let result = take_action(&game, invalid_bet);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot bet on 0 dice");
    }

    #[test]
    fn test_bet_without_data_returns_error() {
        let game = create_test_game();
        let invalid_bet = PlayerAction {
            action: Action::Bet,
            bet: None, // No bet data
        };
        
        let result = take_action(&game, invalid_bet);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Bet action requires dice count and face value");
    }

    #[test]
    fn test_bet_lower_than_previous_returns_error() {
        let mut game = create_test_game();
        
        // Make first bet
        let first_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 5)),
        };
        game = take_action(&game, first_bet).unwrap();
        
        // Try to make lower bet
        let lower_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((2, 5)), // Fewer dice
        };
        
        let result = take_action(&game, lower_bet);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "New bet must be higher than the previous bet");
    }

    #[test]
    fn test_bet_same_dice_lower_face_returns_error() {
        let mut game = create_test_game();
        
        // Make first bet
        let first_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 5)),
        };
        game = take_action(&game, first_bet).unwrap();
        
        // Try to make bet with same dice but lower face
        let lower_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 4)), // Same dice, lower face
        };
        
        let result = take_action(&game, lower_bet);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "New bet must be higher than the previous bet");
    }

    #[test]
    fn test_valid_bet_succeeds() {
        let game = create_test_game();
        let valid_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 5)),
        };
        
        let result = take_action(&game, valid_bet);
        assert!(result.is_ok());
        
        let new_game = result.unwrap();
        assert_eq!(new_game.bets.len(), 1);
        assert_eq!(new_game.bets[0], (0, 3, 5));
        assert_eq!(new_game.current_player, 1); // Current player should increment
    }

    #[test]
    fn test_valid_bet_progression_succeeds() {
        let mut game = create_test_game();
        
        // First bet
        let first_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 5)),
        };
        game = take_action(&game, first_bet).unwrap();
        
        // Higher bet (more dice)
        let higher_bet = PlayerAction {
            action: Action::Bet,
            bet: Some((4, 5)), // More dice, same face
        };
        let result = take_action(&game, higher_bet);
        assert!(result.is_ok());
        
        let new_game = result.unwrap();
        assert_eq!(new_game.bets.len(), 2);
        assert_eq!(new_game.bets[1], (1, 4, 5));
    }

    #[test]
    fn test_valid_call_succeeds() {
        let mut game = create_test_game();
        
        // Make a bet first
        let bet = PlayerAction {
            action: Action::Bet,
            bet: Some((3, 5)),
        };
        game = take_action(&game, bet).unwrap();
        
        // Call the bet
        let call = PlayerAction {
            action: Action::Call,
            bet: None,
        };
        let result = take_action(&game, call);
        assert!(result.is_ok());
        
        let new_game = result.unwrap();
        assert_eq!(new_game.bets.len(), 0); // Bet was removed
    }
}