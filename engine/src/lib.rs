pub mod movegen;
pub mod magics;
pub mod game;
pub mod evaluation;
pub mod negamax;

pub mod parse_input {
    pub fn user_input_to_square_index(input: &str) -> Result<[u8; 2], String> {
        if input == "" {
            return Ok([97, 97]);
        }
        else if input == "legal" {
            return Ok([98, 98]);
        }
        if input.len() != 5 {
            return Err("Invalid input length".to_string());
        }
        let start_file = input.chars().nth(0).unwrap();
        let start_rank = input.chars().nth(1).unwrap();
        let start_file_index = match start_file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err("Invalid file in first square".to_string()),
        };
        let rank_index = match start_rank {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err("Invalid rank in first square".to_string()),
        };

        let target_file = input.chars().nth(3).unwrap();
        let target_rank = input.chars().nth(4).unwrap();
        let target_file_index = match target_file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err("Invalid file in second square".to_string()),
        };
        let target_rank_index = match target_rank {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err("Invalid rank in second square".to_string()),
        };

        Ok([start_file_index + rank_index * 8, target_file_index + target_rank_index * 8])
    }
}