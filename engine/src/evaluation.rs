use std::cmp;
use types::position::Position;
use types::bitboard::BitBoard;
use types::square::Square;
use crate::game;

const PIECE_SQUARE_TABLES_MIDGAME: [[[i32; 8]; 4]; 5] = [
    // ROOKS
    [
        [-31, -21, -25, -13, -27, -22, -2, -17],
        [-20, -13, -11, -5, -15, -2, 12, -19],
        [-14, -8, -1, -4, -4, -6, 16, -1],
        [-5, 6, 3, -6, 3, 12, 18, 9]
    ],
    // KNIGHTS
    [
        [-175, -77, -61, -35, -34, -9, -67, -201],
        [-92, -41, -17, -8, -13, -22, -27, -83],
        [-74, -27, 6, 40, 44, 58, 4, -56],
        [-73, -15, 12, 49, 51, 53, 37, -26]
    ],
    // BISHOPS
    [
        [-53, -15, -7, -5, -12, -16, -17, -48],
        [-5, 8, 21, 11, 29, 6, -14, 1],
        [-8, 19, -5, 25, 22, 1, 5, -14],
        [-23, 4, 17, 39, 31, 11, 0, -23]
    ],
    // QUEENS
    [
        [3, -3, -3, 4, 0, -4, -5, -2],
        [-5, 5, 6, 5, 14, 10, 6, -2],
        [-5, 8, 13, 9, 12, 6, 10, 1],
        [4, 12, 7, 8, 5, 8, 8, -2]
    ],
    // KINGS
    [
        [271, 278, 195, 164, 154, 123, 88, 59],
        [327, 303, 258, 190, 179, 145, 120, 89],
        [271, 234, 169, 138, 105, 81, 65, 45],
        [198, 179, 120, 98, 70, 31, 33, -1],
    ],
];

const PAWN_SQUARE_TABLE_MIDGAME: [[i32; 8]; 8] = [
    [0, 3, -9, -4, 13, 5, -7, 0],
    [0, 3, -15, -23, 0, -12, 7, 0],
    [0, 10, 11, 6, -13, -7, -4, 0],
    [0, 19, 15, 20, 1, 22, -13, 0],
    [0, 16, 32, 40, 11, -8, 5, 0],
    [0, 19, 22, 17, -2, -5, -16, 0],
    [0, 7, 5, 4, -13, -8, 10, 0],
    [0, -5, -22, -8, 5, -8, -8, 0]
];

// Phase values for calculation of phase value for tapered evaluation
// For now, these values are taken from Stockfish
const MIDGAME_LIMIT: u32 = 15258;
const ENDGAME_LIMIT: u32 = 3915;

// Material values weighed in centipawns
const MATERIAL_VALUES_MIDGAME: [u32; 6] = [1276, 781, 825, 2538, 0, 124];
const MATERIAL_VALUES_ENDGAME: [u32; 6] = [1380, 854, 915, 2682, 0, 206];

pub fn main_evaluation(pos: &mut Position) -> i32 {
    if game::is_in_checkmate(pos) {
        if pos.state.active_player == types::Color::White {
            return i32::MIN + 1;
        } else {
            return i32::MAX - 1;
        }
    }
    let player_to_move = match pos.state.active_player {
        types::Color::White => -1,
        types::Color::Black => 1
    };

    let midgame_evaluation = get_midgame_evaluation(pos);
    let mut endgame_evaluation = get_endgame_evaluation(pos);
    let phase = get_phase_value(pos) as i32;
    endgame_evaluation = endgame_evaluation * scale_factor(pos, endgame_evaluation) as i32 / 64;
    let evaluation = (midgame_evaluation * phase + ((endgame_evaluation * (128 - phase)))) / 128;
    // evaluation += tempo(pos);
    evaluation * player_to_move
}

// The scale factor scales down the weight of the endgame evaluation value in the main evaluation
fn scale_factor(pos: &mut Position, endgame_evaluation: i32) -> u32 {
    let pos_flipped = pos.colorflip();
    let (pos_white, pos_black) = if endgame_evaluation > 0 {
        (pos.clone(), pos_flipped.clone())
    } else {
        (pos_flipped.clone(), pos.clone())
    };

    let mut scale_factor: u32 = 64;
    let pawn_count_white = (pos_white.piece_bitboards[5] & pos_white.color_bitboards[0]).count_ones();
    // let pawn_count_black = (pos_black.piece_bitboards[5] & pos_black_flipped.color_bitboards[0]).count_ones();
    let queen_count_white = (pos_white.piece_bitboards[3] & pos_white.color_bitboards[0]).count_ones();
    let queen_count_black = (pos_black.piece_bitboards[3] & pos_black.color_bitboards[0]).count_ones();
    let bishop_count_white = (pos_white.piece_bitboards[2] & pos_white.color_bitboards[0]).count_ones();
    let bishop_count_black = (pos_black.piece_bitboards[2] & pos_black.color_bitboards[0]).count_ones();
    let knight_count_white = (pos_white.piece_bitboards[1] & pos_white.color_bitboards[0]).count_ones();
    let knight_count_black = (pos_black.piece_bitboards[1] & pos_black.color_bitboards[0]).count_ones();
    let non_pawn_material_white = get_npm(&pos_white);
    let non_pawn_material_black = get_npm(&pos_black);

    // If white has no more pawns and the material difference is less than the midgame value of a bishop, scale down the endgame evaluation
    if pawn_count_white == 0 && (non_pawn_material_white - non_pawn_material_black) <= MATERIAL_VALUES_MIDGAME[2] {
        // If the material diffence is also less than the midgame value of a rook, scale down to 0
        if non_pawn_material_white < MATERIAL_VALUES_MIDGAME[0] {
            scale_factor = 0;
        } else {
            // If the material difference is equal to or more than the midgame value of a rook and black's non pawn material is
            // worth less than or as much as a  bishop, scale down to 4
            if non_pawn_material_black <= MATERIAL_VALUES_MIDGAME[2] {
                scale_factor = 4;
            } else {
                // If black's npm is worth more than a bishop, scale down to 14 instead.
                scale_factor = 14;
            }
        }
    }

    // If there is only one queen on the board
    if queen_count_white + queen_count_black == 1 {
        // And it belongs to white
        if queen_count_white == 1 {
            // Set the scale factor to 37 + 3 * amount of black's minor pieces
            scale_factor = 37 + 3 * (bishop_count_black + knight_count_black);
        } else {
            // If the queen belongs to black, do the same for white
            scale_factor = 37 + 3 * (bishop_count_white + knight_count_white);
        }
    } else {
        // If the total queen amount is different from 2, choose the smaller value between the current scale factor
        // and 36 + 7 * white's pawn amount
        scale_factor = cmp::min(scale_factor, 36 + 7 * pawn_count_white);
    }

    // TODO: Opposite Bishops, Passed pawns, Pawn flanks, etc.

    scale_factor    
}    

// The phase value is a value that indicates how much the game is still in midgame vs endgame
fn get_phase_value(pos: &mut Position) -> u32 {
    let pos_flipped = pos.colorflip();
    let mut non_pawn_material = get_npm(pos) + get_npm(&pos_flipped);
    non_pawn_material = cmp::max(ENDGAME_LIMIT, cmp::min(non_pawn_material, MIDGAME_LIMIT));
    ((non_pawn_material - ENDGAME_LIMIT) * 128) / (MIDGAME_LIMIT - ENDGAME_LIMIT)
}

// Gets value of non-pawn material
fn get_npm(pos: &Position) -> u32 {
    let mut npm = 0;
    for piece in 0..6 {
        if piece == 5 {
            continue;
        }
        npm += (pos.piece_bitboards[piece] & pos.color_bitboards[0]).count_ones();
    }
    npm
}

fn get_midgame_evaluation(pos: &mut Position) -> i32 {
    let mut evaluation_score = 0;
    let pos_flipped = pos.colorflip();
    evaluation_score += get_piece_value_midgame(pos) as i32 - get_piece_value_midgame(&pos_flipped) as i32;
    evaluation_score += get_piece_square_table_value_midgame(pos) - get_piece_square_table_value_midgame(&pos_flipped);
    evaluation_score
}

fn get_endgame_evaluation(pos: &mut Position) -> i32 {
    let mut evaluation_score: i32 = 0;
    let pos_flipped = pos.colorflip();
    evaluation_score += get_piece_value_endgame(pos) as i32 - get_piece_value_endgame(&pos_flipped) as i32;
    evaluation_score
}

fn get_piece_square_table_value_midgame(pos: &Position) -> i32 {
    let mut psqt_score = 0;
    for piece in 0..6 {
        let mut piece_value = 0;
        let piece_bitboard = pos.piece_bitboards[piece] & pos.color_bitboards[0];
        for square in 0..64 {
            if !(piece_bitboard & BitBoard::from_square(Square::index(square))).is_empty() {
                let rank = cmp::min(7- square/8, square / 8);
                let file = square % 8;
                match piece {
                    5 => piece_value += PAWN_SQUARE_TABLE_MIDGAME[rank as usize][file as usize],
                    _ => piece_value += PIECE_SQUARE_TABLES_MIDGAME[piece][rank as usize][file as usize]
                }
            }
        }
        psqt_score += piece_value;
    }
    psqt_score
}

fn get_piece_value_midgame(pos: &Position) -> u32 {
    get_material_value(pos, true)
}

fn get_piece_value_endgame(pos: &Position) -> u32 {
    get_material_value(pos, false)
}

// Get the percentage phase value that indicates how much the game is still in midgame vs endgame
fn get_material_value(pos: &Position, midgame: bool) -> u32 {
    let mut total_piece_value: u32 = 0;
    match midgame {
        true => {
            for piece in 0..6 {
                total_piece_value += (pos.piece_bitboards[piece] & pos.color_bitboards[0]).count_ones() * MATERIAL_VALUES_MIDGAME[piece];
            }
        },
        false => {
            for piece in 0..6 {
                total_piece_value += (pos.piece_bitboards[piece] & pos.color_bitboards[0]).count_ones() * MATERIAL_VALUES_ENDGAME[piece];
            }
        }
    }
    total_piece_value
}
