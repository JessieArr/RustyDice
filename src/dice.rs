use macroquad::rand::gen_range;
use macroquad::prelude::*;

/// Rolls a dice and returns a random value from 1 to 6
pub fn roll_dice() -> u8 {
    gen_range(1, 7)
}

/// Draws dots on a dice based on the given value (1-6)
pub fn draw_dice_dots(x: f32, y: f32, value: u8) {
    match value {
        1 => {
            // Center dot
            draw_circle(x, y, 8.0, BLACK);
        }
        2 => {
            // Top-left and bottom-right
            draw_circle(x - 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x + 25.0, y + 25.0, 8.0, BLACK);
        }
        3 => {
            // Top-left, center, and bottom-right
            draw_circle(x - 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x, y, 8.0, BLACK);
            draw_circle(x + 25.0, y + 25.0, 8.0, BLACK);
        }
        4 => {
            // All four corners
            draw_circle(x - 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x + 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x - 25.0, y + 25.0, 8.0, BLACK);
            draw_circle(x + 25.0, y + 25.0, 8.0, BLACK);
        }
        5 => {
            // All four corners plus center
            draw_circle(x - 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x + 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x, y, 8.0, BLACK);
            draw_circle(x - 25.0, y + 25.0, 8.0, BLACK);
            draw_circle(x + 25.0, y + 25.0, 8.0, BLACK);
        }
        6 => {
            // Two columns of three dots
            draw_circle(x - 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x + 25.0, y - 25.0, 8.0, BLACK);
            draw_circle(x - 25.0, y, 8.0, BLACK);
            draw_circle(x + 25.0, y, 8.0, BLACK);
            draw_circle(x - 25.0, y + 25.0, 8.0, BLACK);
            draw_circle(x + 25.0, y + 25.0, 8.0, BLACK);
        }
        _ => {
            // Invalid value, draw nothing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_dice_range() {
        for _ in 0..1000 {
            let result = roll_dice();
            assert!(result >= 1 && result <= 6, "Dice roll {} is not in range 1-6", result);
        }
    }

    #[test]
    fn test_roll_dice_distribution() {
        let mut counts = [0; 6];
        let num_rolls = 10000;
        
        for _ in 0..num_rolls {
            let result = roll_dice();
            counts[(result - 1) as usize] += 1;
        }
        
        // Check that each number appears at least once
        for (i, &count) in counts.iter().enumerate() {
            assert!(count > 0, "Number {} never appeared in {} rolls", i + 1, num_rolls);
        }
        
        // Check that distribution is roughly uniform (within 20% of expected)
        let expected = num_rolls / 6;
        for (i, &count) in counts.iter().enumerate() {
            let deviation = (count as f32 - expected as f32).abs() / expected as f32;
            assert!(
                deviation < 0.2,
                "Number {} appeared {} times (expected ~{}), deviation: {:.2}",
                i + 1,
                count,
                expected,
                deviation
            );
        }
    }
} 