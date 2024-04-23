use std::cmp;
use types::{
    position::Position,
    bitboard::BitBoard,
    types_utils::*,
};
use crate::{
    movegen,
    game,
};

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

const PIECE_SQUARE_TABLES_ENDGAME: [[[i32; 8]; 4]; 5] = [
    // ROOKS
    [
        [-9, -12, -6, -6, -5, -6, 4, 18],
        [-13, -9, -8, 1, 8, 1, 5, 0],
        [-10, -1, -2, -9, 7, -7, 20, 19],
        [-9, -2, -6, 7, -6, 10, -5, 13]
    ],
    // KNIGHTS
    [
        [-96, -67, 40, -35, -45, -51, -69, -100],
        [-64, -54, -27, -2, -16, -44, -50, -88],
        [-49, -18, -8, 13, 9, -16, -51, -56],
        [-21, 8, 29, 28, 39, 17, 12, -17]
    ],
    // BISHOPS
    [
        [-57, -37, -16, -20, -17, -30, -31, -46],
        [-30, -13, -1, -6, -1, 6, -20, -42],
        [-37, -17, -2, 0, -14, 4, -1, -37],
        [-12, 1, 10, 17, 15, 6, 1, -24]
    ],
    // QUEENS
    [
        [-69, -55, -39, -23, -29, 38, -50, -75],
        [-57, -31, -18, -3, -6, -18, -27, -52],
        [-47, -22, -9, 13, -9, -12, -24, -43],
        [-26, -4, 3, 24, 21, 1, -8, -36]
    ],
    // KINGS
    [
        [1, 53, 88, 103, 96, 92, 47, 11],
        [45, 100, 130, 156, 166, 172, 121, 59],
        [85, 133, 169, 172, 199, 184, 116, 73],
        [76, 135, 175, 172, 199, 191, 131, 78]
    ],
];

const PAWN_SQUARE_TABLE_ENDGAME: [[i32; 8]; 8] = [
    [0, -10, -10, 6, 10, 28, 0, 0],
    [0, -6, -10, -2, 5, -20, -11, 0],
    [0, 10, -10, -8, 4, 21, 12, 0],
    [0, 0, 4, -4, -5, 28, 21, 0],
    [0, 14, 4, -13, -5, 30, 25, 0],
    [0, 7, 3, -12, -5, 7, 19, 0],
    [0, -5, -6, -10, 14, 6, 4, 0],
    [0, -19, -4, -9, 9, 13, 7, 0]
];

trait PieceSquareTable {
    fn get_value(&self, piece: usize, rank: usize, file: usize) -> i32;
}

impl PieceSquareTable for [[i32; 8]; 8] {
    fn get_value(&self, _piece: usize, rank: usize, file: usize) -> i32 {
        self[rank][file]
    }
}

impl PieceSquareTable for [[[i32; 8]; 4]; 5] {
    fn get_value(&self, piece: usize, rank: usize, file: usize) -> i32 {
        self[piece][rank][file]
    }
}

// Mobility bonus depending on how many squares a rook can reach (min. 0, max. 14)
const ROOK_MOBILITY_BONUS_TABLE_MIDGAME: [i32; 15] = [
    -60, -20, 2, 3, 3, 11, 22, 31, 40, 40, 41, 48, 57, 57, 62
];
// Mobility bonus depending on how many squares a knight can reach (min. 0, max. 8)
const KNIGHT_MOBILITY_BONUS_TABLE_MIDGAME: [i32; 9] = [
    -62, -53, -12, -4, 3, 13, 22, 28, 33
];
// Mobility bonus depending on how many squares a bishop can reach (min. 0, max. 13)
const BISHOP_MOBILITY_BONUS_TABLE_MIDGAME: [i32; 14] = [
    -48, -20, 16, 26, 38, 51, 55, 63, 63, 68, 81, 81, 91, 98
];
// Mobility bonus depending on how many squares a queen can reach (min. 0, max. 27)
const QUEEN_MOBILITY_BONUS_TABLE_MIDGAME: [i32; 28] = [
    -30, -12, -8, -9, 20, 23, 23, 35, 38, 53, 64, 65, 65, 66, 67, 67, 72, 72, 77, 79, 93, 108, 108, 108, 110, 114, 114, 116
];

const ROOK_MOBILITY_BONUS_TABLE_ENDGAME: [i32; 15] = [
    -78, -17, 23, 39, 70, 99, 103, 121, 134, 139, 158, 164, 168, 169, 172
];

const KNIGHT_MOBILITY_BONUS_TABLE_ENDGAME: [i32; 9] = [
    -81, -56, -31, -16, 5, 11, 17, 20, 25
];

const BISHOP_MOBILITY_BONUS_TABLE_ENDGAME: [i32; 14] = [
    -59, -23, -3, 13, 24, 42, 54, 57, 65, 73, 78, 86, 88, 97
];

const QUEEN_MOBILITY_BONUS_TABLE_ENDGAME: [i32; 28] = [
    -48, -30, -7, 19, 40, 55, 59, 75, 78, 96, 96, 100, 121, 127, 131, 133, 136, 141, 147, 150, 151, 168, 168, 171, 182, 182, 192, 219
];

// Phase values for calculation of phase value for tapered evaluation
// For now, these values are taken from Stockfish
const MIDGAME_LIMIT: u32 = 15258;
const ENDGAME_LIMIT: u32 = 3915;

// Material values weighed in centipawns
const MATERIAL_VALUES_MIDGAME: [u32; 6] = [1276, 781, 825, 2538, 0, 124];
const MATERIAL_VALUES_ENDGAME: [u32; 6] = [1380, 854, 915, 2682, 0, 206];

pub fn main_evaluation(pos: &mut Position) -> i32 {
    // Instantly return the lower bound of the evaluation if the position is in checkmate
    // (seen from the side to move - if you are to move and in checkmate, eval is -infinity)
    if game::is_in_checkmate(pos) {
        return i32::MIN + 1;
    }
    let player_to_move = match pos.state.active_player {
        types::Color::White => 1,
        types::Color::Black => -1
    };

    let midgame_evaluation = get_midgame_evaluation(pos);
    let mut endgame_evaluation = get_endgame_evaluation(pos);
    let phase = get_phase_value(pos) as i32;
    let scale_factor = scale_factor(pos, endgame_evaluation);
    endgame_evaluation = endgame_evaluation * scale_factor as i32 / 64;
    /* println!("Phase: {}", phase);
    println!("Scale factor: {}", scale_factor);
    println!("Midgame evaluation: {}, Endgame evaluation: {}", midgame_evaluation, endgame_evaluation); */
    let mut evaluation = (midgame_evaluation * phase + ((endgame_evaluation * (128 - phase)))) / 128;
    evaluation += tempo(pos);
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
    let pawn_count_black = (pos_black.piece_bitboards[5] & pos_black.color_bitboards[0]).count_ones();
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

    // If the scale factor has not been changed yet during this evaluation
    if scale_factor == 64 {
        let opposite_bishops = opposite_bishops(pos);
        if opposite_bishops && non_pawn_material_white == MATERIAL_VALUES_MIDGAME[2] && non_pawn_material_black == MATERIAL_VALUES_MIDGAME[2] {
            scale_factor = 22 /* + 4 * candidate_passed(pos_white) */
        } else if opposite_bishops {
            scale_factor = 22 + 3 * piece_count(&pos_white);
        } else {
            // If both white's and black's non pawn material are worth as much as a bishop and if both players control the same amount of pawns
            if non_pawn_material_white == MATERIAL_VALUES_MIDGAME[0] && non_pawn_material_black == MATERIAL_VALUES_MIDGAME[0] && pawn_count_white - pawn_count_black <= 1 {
                let mut pawn_king_black = false;
                let mut pcw_flank = [0, 0];
                let mut pos_iterator = pos_white.color_bitboards[0] | pos_black.color_bitboards[0];
                // Iterate over all occupied squares in the position
                while pos_iterator != BitBoard::empty() {
                    let index = pos_iterator.trailing_zeros() as usize;
                    // If the square is occupied by a white pawn
                    if pos_white.color_bitboards[0] & pos_white.piece_bitboards[5] & BitBoard::from_index(index) != BitBoard::empty() {
                        let array_index = if index / 8 < 4 {1} else {0};
                        // Note the pawn flank
                        pcw_flank[array_index] = 1;
                    }
                    // If the square is occupied by a black king
                    if pos_black.color_bitboards[0] & pos_black.piece_bitboards[4] & BitBoard::from_index(index) != BitBoard::empty() {
                        // For each direction
                        for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                            let new_index = index as i32 + delta.0 * 8 + delta.1;
                            if new_index < 0 || new_index > 63 {
                                continue;
                            }
                            // If the square is occupied by a black pawn
                            if pos_black.color_bitboards[0] & pos_black.piece_bitboards[5] & BitBoard::from_index(new_index as usize) != BitBoard::empty() {
                                // Note that a black pawn is next to the black king
                                pawn_king_black = true;
                                break;
                            }
                        }
                    }
                    pos_iterator.clear_lsb();
                }
                // If the pawn flank is different and there is an opposite color pawn next to the opposite color king, scale down to 36
                if pcw_flank[0] != pcw_flank[1] && pawn_king_black {
                    return 36;
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
        }
    }

    scale_factor    
}    

// The phase value is a value that indicates how much the game is still in midgame vs endgame
fn get_phase_value(pos: &mut Position) -> u32 {
    let pos_flipped = pos.colorflip();
    let mut non_pawn_material = get_npm(pos) + get_npm(&pos_flipped);
    // println!("npm before ceiling: {}", non_pawn_material);
    non_pawn_material = cmp::max(ENDGAME_LIMIT, cmp::min(non_pawn_material, MIDGAME_LIMIT));
    // println!("Non pawn material: {}", non_pawn_material);
    ((non_pawn_material - ENDGAME_LIMIT) * 128) / (MIDGAME_LIMIT - ENDGAME_LIMIT)
}

// Gets value of non-pawn material
fn get_npm(pos: &Position) -> u32 {
    let mut npm = 0;
    for piece in 0..6 {
        if piece == 5 {
            continue;
        }
        npm += (pos.piece_bitboards[piece] & pos.color_bitboards[0]).count_ones() * MATERIAL_VALUES_MIDGAME[piece];
    }
    npm
}

fn get_midgame_evaluation(pos: &mut Position) -> i32 {
    let mut evaluation_score = 0;
    let pos_flipped = pos.colorflip();
    evaluation_score += get_piece_value_midgame(pos) as i32 - get_piece_value_midgame(&pos_flipped) as i32;
    evaluation_score += get_piece_square_table_value(pos, true) - get_piece_square_table_value(&pos_flipped, true);
    evaluation_score += get_mobility_score(pos, true) as i32 - get_mobility_score(&pos_flipped, true) as i32;
    // TODO: pawn structure: isolated, backward, doubled, connected, chained, etc.
    // TODO: piece safety
    // TODO: passed pawns
    // TODO: space
    // TODO: king safety score
    evaluation_score
}

fn get_endgame_evaluation(pos: &mut Position) -> i32 {
    let mut evaluation_score: i32 = 0;
    let pos_flipped = pos.colorflip();
    evaluation_score += get_piece_value_endgame(pos) as i32 - get_piece_value_endgame(&pos_flipped) as i32;
    evaluation_score
}

fn piece_count(pos: &Position) -> u32 {
    pos.color_bitboards[0].count_ones()
}

fn get_piece_square_table_value(pos: &Position, midgame: bool) -> i32 {
    let mut psqt_score = 0;
    for piece in 0..6 {
        let piece_bitboard = pos.piece_bitboards[piece] & pos.color_bitboards[0];
        // Define trait object for piece-square table
        let piece_square_table: &dyn PieceSquareTable = match piece {
            5 => {
                match midgame {
                    true => &PAWN_SQUARE_TABLE_MIDGAME,
                    false => &PAWN_SQUARE_TABLE_ENDGAME,
                }
            },
            _ => {
                match midgame {
                    true => &PIECE_SQUARE_TABLES_MIDGAME,
                    false => &PIECE_SQUARE_TABLES_ENDGAME,
                }
            },
        };
        // Iterate over set bits of the piece bitboard
        let mut bb = piece_bitboard;
        while bb.0 != 0 {
            let square = bb.trailing_zeros() as usize;
            let rank = cmp::min(7 - square / 8, square / 8);
            let file = square % 8;
            psqt_score += piece_square_table.get_value(piece, rank as usize, file as usize);
            bb.clear_lsb(); // Clear the least significant set bit
        }
    }
    psqt_score
}

fn get_mobility_score(pos: &Position, midgame: bool) -> i32 {
    let mobility_range = get_mobility_range(pos);
    let mut mobility_score = 0;
    let mut iterator = pos.color_bitboards[0];
    while !iterator.is_empty() {
        let index = iterator.trailing_zeros() as u8;
        let mobility = get_mobility(pos, index, mobility_range);
        let piece = pos.piece_at(index).unwrap().0;
        match piece {
            0 => {
                if midgame {
                    mobility_score += ROOK_MOBILITY_BONUS_TABLE_MIDGAME[mobility as usize];
                } else {
                    mobility_score += ROOK_MOBILITY_BONUS_TABLE_ENDGAME[mobility as usize];
                }
            },
            1 => {
                if midgame {
                    mobility_score += KNIGHT_MOBILITY_BONUS_TABLE_MIDGAME[mobility as usize];
                } else {
                    mobility_score += KNIGHT_MOBILITY_BONUS_TABLE_ENDGAME[mobility as usize];
                }
            },
            2 => {
                if midgame {
                    mobility_score += BISHOP_MOBILITY_BONUS_TABLE_MIDGAME[mobility as usize];
                } else {
                    mobility_score += BISHOP_MOBILITY_BONUS_TABLE_ENDGAME[mobility as usize];
                }
            },
            3 => {
                if midgame {
                    mobility_score += QUEEN_MOBILITY_BONUS_TABLE_MIDGAME[mobility as usize];
                } else {
                    mobility_score += QUEEN_MOBILITY_BONUS_TABLE_ENDGAME[mobility as usize];
                }
            },
            _ => ()
        }
        iterator.clear_lsb();
    }
    
    mobility_score
} 

fn get_mobility(pos: &Position, square: u8, mobility_range: BitBoard) -> u32 {
    if let Some((piece, _color)) = pos.piece_at(square) {
        match piece {
            0 | 2 | 3 => {
                let mut moves = movegen::slider_moves(piece, square, pos);
                // Remove all squares that are not within our mobility range
                moves &= mobility_range;
                return moves.count_ones();
            },                    
            1 => {
                let mut moves = movegen::get_pseudolegal_knight_moves(square);
                // Remove all squares that are not within our mobility range
                moves &= mobility_range;
                return moves.count_ones();
            },
            _ => ()
        }
    }
    0
}

fn is_in_mobility_area(pos: &Position, square: u8) -> bool {
    // If the target square is occupied by our own king or queen, return false
    if pos.color_bitboards[0] & pos.piece_bitboards[4] & BitBoard::from_square(square) != BitBoard::empty() {
        return false;
    }
    if pos.color_bitboards[0] & pos.piece_bitboards[3] & BitBoard::from_square(square) != BitBoard::empty() {
        return false;
    }
    // If the square is protected by an enemy pawn, return false
    if let Some(offset_square) = try_square_offset(square, -1, 1) {
        if pos.color_bitboards[1] & pos.piece_bitboards[5] & BitBoard::from_square(offset_square) != BitBoard::empty() {
            return false;
        }
    }
    if let Some(offset_square) = try_square_offset(square, 1, 1) {
        if pos.color_bitboards[1] & pos.piece_bitboards[5] & BitBoard::from_square(offset_square) != BitBoard::empty() {
            return false;
        }
    }
    // If the square is on the 2nd or 3rd rank and is occupied by our own pawn, return false
    if pos.color_bitboards[0] & pos.piece_bitboards[5] & BitBoard::from_square(square) != BitBoard::empty() &&
        square / 8 < 3 {
            return false;
    }
    // TODO: exclude blockers for king from the mobility area
    true
}

fn get_mobility_range(pos: &Position) -> BitBoard {
    let mut mobility_range = BitBoard::from_u64(0xffffffffffffffff);
    let queen = pos.piece_bitboards[3] & pos.color_bitboards[0];
    let king = pos.piece_bitboards[4] & pos.color_bitboards[0];
    mobility_range ^= queen ^ king;
    let mut mobility_range_iterator = mobility_range;
    while !mobility_range_iterator.is_empty() {
        let index = mobility_range_iterator.trailing_zeros() as u8;
        if !is_in_mobility_area(pos, index) {
            mobility_range &= !BitBoard::from_square(index);
        }
        mobility_range_iterator.clear_lsb();
    }
    mobility_range
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

/* Material imbalance describes the concept of pieces not only having a static value assignment (like 1-3-3-5-9), but
considering what other pieces are on the board as well */
/* fn material_imbalance(pos: &Position) {
    let mut imbalance = 0;
    v += 

} */

// Returns true if the position contains a bishop pair from the view of the white player
/* fn bishop_pair(pos: &Position) -> u32 {
    if (pos.piece_bitboards[2] & pos.color_bitboards[0]).count_ones() < 2 {
        return 0;
    }
    1438
} */

// Returns true if the position contains opposite bishops (white vs dark squares)
fn opposite_bishops(pos: &Position) -> bool {
    if pos.piece_bitboards[2].count_ones() != 2 {
        return false;
    }
    let white_bishops = pos.piece_bitboards[2] & pos.color_bitboards[0];
    let black_bishops = pos.piece_bitboards[2] & pos.color_bitboards[1];
    let white_bishop_square = white_bishops.trailing_zeros() % 8;
    let black_bishop_square = black_bishops.trailing_zeros() % 8;
    white_bishop_square % 2 != black_bishop_square % 2
}

// Being the side whose turn it is confers a small bonus
fn tempo(pos: &Position) -> i32 {
    28 * if pos.state.active_player == types::Color::White {1} else {-1}
}