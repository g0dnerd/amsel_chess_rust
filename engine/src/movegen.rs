// use std::collections::HashMap;
use crate::magics::*;
use crate::game;
use precompute::magics::MagicTableEntry;
use precompute::precompute_magics::{BISHOP, ROOK};
use types::bitboard::BitBoard;
use types::position::Position;
use types::Castling;
use types::square::Square;
use types::Color;

/* Use our ray-scanning algorithm from the precompute module to get potential blockers for a piece,
/ then AND the result with the all_pieces BitBoard to get the actual blockers. */
pub fn get_all_actual_blockers(
    directions: &[(i8, i8)],
    square: Square,
    position: &Position,
) -> BitBoard {
    let mut blockers = BitBoard::empty();
    // XOR the square and the blocker BitBoard together to remove the piece we are analyzing from the list of potential blockers.
    let all_pieces = position.all_pieces() ^ BitBoard::from_square(square);
    for &(dx, dy) in directions {
        let mut ray = square;
        while let Some(offset_by_delta) = ray.try_offset(dx, dy) {
            blockers |= BitBoard::from_square(ray);
            ray = offset_by_delta;
        }
    }
    blockers &= !BitBoard::from_square(square);
    blockers & all_pieces
}

pub fn get_first_actual_blockers(
    directions: &[(i8, i8)],
    square: Square,
    position: &Position,
) -> BitBoard {
    let mut blockers = BitBoard::empty();
    let all_pieces = position.all_pieces() ^ BitBoard::from_square(square);
    for &(dx, dy) in directions {
        let mut ray = square;
        while let Some(offset_by_delta) = ray.try_offset(dx, dy) {
            ray = offset_by_delta;
            if position.piece_at(ray).is_some() {
                blockers |= BitBoard::from_square(ray);
                break;
            }
        }
    }
    blockers & all_pieces
}

pub fn slider_moves(square: Square, blockers: BitBoard, directions: &[(i8, i8)]) -> BitBoard {
    let mut moves = BitBoard::empty();
    for &(dx, dy) in directions {
        let mut ray = square;
        
        /* Find possible moves with the following procedure:
        /  1. Start at the piece's square.
        /  2. Try to offset the square by one of the four delta directions specified below.
        /  3. Loop terminates if that new square is in the list of blockers.
        /  4. If not, square gets added to legal moves. */
        while !blockers.contains(ray) {
            if let Some(offset_by_delta) = ray.try_offset(dx, dy) {
                ray = offset_by_delta;
                moves |= BitBoard::from_square(ray);
            } else {
                break;
            }
        }
    }
    moves
}

pub fn pawn_attacks(position: &mut Position, square: Square) -> BitBoard {
    let mut moves = BitBoard::empty();
    let color = position.piece_at(square).unwrap().1;
    for &dx in &[-1, 1] {
        if let Some(offset_by_delta) = square.try_offset(dx, match color {
            Color::White => 1,
            Color::Black => -1,
        }){
            moves |= BitBoard::from_square(offset_by_delta);
        }
    }
    moves
}

fn magic_index(entry: &MagicTableEntry, blockers: BitBoard) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

pub fn get_rook_moves(square: Square, position: &Position) -> BitBoard {
    // Handle potential errors when trying to unwrap a piece from an empty square
    let piece = position.piece_at(square);
    match piece {
        None => panic!("get_rook_moves called on empty square"),
        _ => (),
    }

    let blockers = get_all_actual_blockers(&ROOK.directions, square, position);
    let magic_entry = &ROOK_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    let mut moves = BitBoard::from_u64(ROOK_MOVES[index]);

    // Remove all moves that would capture a piece of the same color
    let color = position.piece_at(square).unwrap().1;
    moves = moves & !position.color_bitboards[color as usize];
    moves
}

pub fn get_rook_moves_from_blockers(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_entry = &ROOK_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    BitBoard(ROOK_MOVES[index])
}

pub fn get_bishop_moves(square: Square, position: &Position) -> BitBoard {
    // Handle potential errors when trying to unwrap a piece from an empty square
    let piece = position.piece_at(square);
    match piece {
        None => panic!("get_bishop_moves called on empty square"),
        _ => (),
    }

    let blockers = get_all_actual_blockers(&BISHOP.directions, square, position);
    let magic_entry = &BISHOP_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    let mut moves = BitBoard::from_u64(BISHOP_MOVES[index]);

    // Remove all moves that would capture a piece of the same color
    let color = position.piece_at(square).unwrap().1;
    moves = moves & !position.color_bitboards[color as usize];
    moves
}

pub fn get_bishop_moves_from_blockers(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_entry = &BISHOP_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    BitBoard(BISHOP_MOVES[index])
}

pub fn get_queen_moves(square: Square, position: &Position) -> BitBoard {
    get_rook_moves(square, position)
        | get_bishop_moves(square, position)
}

pub fn get_queen_moves_from_blockers(square: Square, blockers: BitBoard) -> BitBoard {
    get_rook_moves_from_blockers(square, blockers)
        | get_bishop_moves_from_blockers(square, blockers)
}

pub fn get_knight_moves(square: Square, position: &Position) -> BitBoard {
    // Handle potential errors when trying to unwrap a piece from an empty square
    let piece = position.piece_at(square);
    match piece {
        None => panic!("get_knight_moves called on empty square"),
        _ => (),
    }
    let mut moves = BitBoard::empty();
    for &(dx, dy) in &[
        (1, 2),
        (2, 1),
        (1, -2),
        (2, -1),
        (-1, 2),
        (-2, 1),
        (-1, -2),
        (-2, -1),
    ] {
        if let Some(offset_by_delta) = square.try_offset(dx, dy) {
            moves |= BitBoard::from_square(offset_by_delta);
        }
    }
    let color = position.piece_at(square).unwrap().1;
    moves = moves & !position.color_bitboards[color as usize];
    moves
}

pub fn get_king_moves(square: Square, position: &Position) -> BitBoard {
    // Handle potential errors when trying to unwrap a piece from an empty square
    let piece = position.piece_at(square);
    match piece {
        None => panic!("get_king_moves called on empty square"),
        _ => (),
    }

    let mut moves = BitBoard::empty();
    for &(dx, dy) in &[
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ] {
        if let Some(offset_by_delta) = square.try_offset(dx, dy) {
            moves |= BitBoard::from_square(offset_by_delta);
        }
    }
    
    let color = position.piece_at(square).unwrap().1;

    if position.state.castling_rights.0 != Castling::NO_CASTLING {
        match color {
            Color::White => {
                if position.state.castling_rights.0 & Castling::WHITE_KING_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(Square::F1).is_none()
                        && position.piece_at(Square::G1).is_none()
                    {
                        moves |= BitBoard::from_square(Square::G1);
                    }
                }
                if position.state.castling_rights.0 & Castling::WHITE_QUEEN_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(Square::D1).is_none()
                        && position.piece_at(Square::C1).is_none()
                        && position.piece_at(Square::B1).is_none()
                    {
                        moves |= BitBoard::from_square(Square::B1);
                    }
                }
            }
            Color::Black => {
                if position.state.castling_rights.0 & Castling::BLACK_KING_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(Square::F8).is_none()
                        && position.piece_at(Square::G8).is_none()
                    {
                        moves |= BitBoard::from_square(Square::G8);
                    }
                }
                if position.state.castling_rights.0 & Castling::BLACK_QUEEN_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(Square::D8).is_none()
                        && position.piece_at(Square::C8).is_none()
                        && position.piece_at(Square::B8).is_none()
                    {
                        moves |= BitBoard::from_square(Square::B8);
                    }
                }
            
            }
        }
    }

    moves = moves & !position.color_bitboards[color as usize];
    moves
}

pub fn get_pawn_moves(square: Square, position: &Position) -> BitBoard {
    let mut moves = BitBoard::empty();
    let color = position.piece_at(square).unwrap().1;
    let direction = match color {
        Color::White => 1,
        Color::Black => -1,
    };
    let offset_by_delta = square.try_offset(0, direction);
    if let Some(offset_by_delta) = offset_by_delta {
        if position.piece_at(offset_by_delta).is_none() {
            moves |= BitBoard::from_square(offset_by_delta);
            if (color == Color::White && square.rank_index() == 1)
                || (color == Color::Black && square.rank_index() == 6)
            {
                if let Some(offset_by_delta) = offset_by_delta.try_offset(0, direction) {
                    if position.piece_at(offset_by_delta).is_none() {
                        moves |= BitBoard::from_square(offset_by_delta);
                    }
                }
            }
        }
    }
    for &dx in &[-1, 1] {
        if let Some(offset_by_delta) = square.try_offset(dx, direction) {
            if let Some((_piece, x_color)) = position.piece_at(offset_by_delta) {
                if color != x_color {
                    moves |= BitBoard::from_square(offset_by_delta);
                }
            }
        }
    }
    moves
}

pub fn get_moves_by_square(square: Square, pos: &Position) -> BitBoard {
    let piece = pos.piece_at(square).unwrap().0;
    match piece {
        0 => get_rook_moves(square, pos),
        1 => get_knight_moves(square, pos),
        2 => get_bishop_moves(square, pos),
        3 => get_queen_moves(square, pos),
        4 => get_king_moves(square, pos),
        5 => get_pawn_moves(square, pos),
        _ => BitBoard::empty(),
    }
}

pub fn get_all_legal_moves_for_color(color: Color, pos: &mut Position) -> Vec<(Square, Square)> {
    let mut moves: Vec<(Square, Square)> = Vec::new();

    // Iterate over all squares with a piece of the given color
    let squares = pos.color_bitboards[color as usize].squares_from_bb();
    for square in squares {
        let piece = pos.piece_at(square).unwrap().0;
        let piece_moves = match piece {
            0 => get_rook_moves(square, pos),
            1 => get_knight_moves(square, pos),
            2 => get_bishop_moves(square, pos),
            3 => get_queen_moves(square, pos),
            4 => get_king_moves(square, pos),
            5 => get_pawn_moves(square, pos),
            _ => BitBoard::empty(),
        };

        for piece_move in piece_moves.squares_from_bb() {
            moves.push((square, piece_move));
        }
    }

    // Iterate over all moves and remove those that would put or leave the king in check
    let mut moves_to_remove: Vec<(Square, Square)> = Vec::new();
    for (square, target_square) in moves.iter() {
        let mut new_pos = pos.clone();
        let is_king = new_pos.piece_bitboards[4].contains(*square);
        let is_slider = (new_pos.piece_bitboards[0] | new_pos.piece_bitboards[2] | new_pos.piece_bitboards[3]).contains(*square);

        // List of sliders that after the move no longer have their path blocker by the moved piece
        let blocked_sliders = game::is_blocking_slider(&mut new_pos, *square);

        // Remove the move if it would castle through check
        if pos.piece_at(*square).unwrap().0 == 4 {
            if *square == Square::E1 && *target_square == Square::G1 {
                if pos.attacked_by_black.contains(Square::F1) || pos.attacked_by_black.contains(Square::G1) {
                    moves_to_remove.push((*square, *target_square));
                    continue;
                }
            } else if *square == Square::E1 && *target_square == Square::B1 {
                if pos.attacked_by_black.contains(Square::D1) ||
                        pos.attacked_by_black.contains(Square::C1) ||
                        pos.attacked_by_black.contains(Square::B1) {
                    moves_to_remove.push((*square, *target_square));
                    continue;
                }
            } else if *square == Square::E8 && *target_square == Square::G8 {
                if pos.attacked_by_white.contains(Square::F8) || pos.attacked_by_white.contains(Square::G8) {
                    moves_to_remove.push((*square, *target_square));
                    continue;
                }
            } else if *square == Square::E8 && *target_square == Square::B8 {
                if pos.attacked_by_white.contains(Square::D8) ||
                        pos.attacked_by_white.contains(Square::C8) ||
                        pos.attacked_by_white.contains(Square::B8) {
                    moves_to_remove.push((*square, *target_square));
                    continue;
                }
            }
        }

        new_pos.make_move(&square, &target_square);

        // Bitboard of squares of sliders that now have their path blocked by the new pieces
        let affected_sliders = game::is_blocking_slider(&mut new_pos, *target_square);
        
        // If the moved piece was blocking a slider, update those sliders
        if !blocked_sliders.is_empty() {
            game::update_slider_attacks(&mut new_pos, blocked_sliders);
            // Update blockers for each slider that is no longer blocked
            for slider in blocked_sliders.squares_from_bb() {
                game::update_slider_blockers(pos, slider);
            }             
        }

        // If the moved piece now blocks a slider, update those sliders
        if !affected_sliders.is_empty() {
            // If the moved piece is a king and is blocking an opposing slider, the move is illegal
            if is_king && (affected_sliders & new_pos.color_bitboards[!color as usize] != BitBoard::empty()) {
                // Flag only this move for removal
                moves_to_remove.push((*square, *target_square));
                continue;
            }
            for slider in affected_sliders.squares_from_bb() {
                game::update_slider_blockers(pos, slider);
            }
            game::update_slider_attacks(&mut new_pos, affected_sliders);
        }

        if is_slider {
            new_pos.slider_blockers[*square as usize] = BitBoard::empty();
            game::update_slider_blockers(&mut new_pos, *target_square);
            game::update_slider_attacks(&mut new_pos, BitBoard::from_square(*square));

        } else {
            // update attackers for the moved piece
            game::attacks_after_move(&mut new_pos, *square, *target_square);
        }

        // If after these updates, the king is in the list of attacked squares, the move is illegal
        match color {
            Color::White => {
                let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[0]).squares_from_bb()[0];
                if new_pos.attacked_by_black.contains(king_square) {
                    moves_to_remove.push((*square, *target_square));
                    continue;
                }
            },
            Color::Black => {
                let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[1]).squares_from_bb()[0];
                if new_pos.attacked_by_white.contains(king_square) {
                    moves_to_remove.push((*square, *target_square));
                    continue;
                }
            }
        }
    }
    // Remove all tuples from moves that are in moves_to_remove
    moves.retain(|&x| !moves_to_remove.contains(&x));
    moves
}

/* Movegen method with reduced scope since when in check, the only possible pieces with available moves are then king,
/ pieces that can capture the piece giving check or pieces that can block the check */
pub fn get_legal_moves_from_check(color: Color, pos: &mut Position) -> Vec<(Square, Square)> {
    let mut legal_moves: Vec<(Square, Square)> = get_all_legal_moves_for_color(color, pos);
    let mut moves_to_remove: Vec<(Square, Square)> = Vec::new();
    for legal_move in legal_moves.iter() {
        let mut new_pos = pos.clone();
        let is_slider = (new_pos.piece_bitboards[0] | new_pos.piece_bitboards[2] | new_pos.piece_bitboards[3]).contains(legal_move.0);
        new_pos.make_move(&legal_move.0, &legal_move.1);
        if is_slider {
            game::update_slider_attacks(&mut new_pos, BitBoard::from_square(legal_move.1));
            game::update_slider_blockers(&mut new_pos, legal_move.1);
        }
        let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[color as usize]).squares_from_bb()[0];
        let blocked_sliders = game::is_blocking_slider(&mut new_pos, legal_move.1);
        let is_capture = new_pos.piece_at(legal_move.1).is_some();

        if !is_capture && blocked_sliders.is_empty() && new_pos.piece_at(legal_move.1).unwrap().0 != 4 {
            moves_to_remove.push(*legal_move);
            continue;
        }
        if !blocked_sliders.is_empty() {
            game::update_slider_attacks(&mut new_pos, blocked_sliders);
            for slider in blocked_sliders.squares_from_bb() {
                game::update_slider_blockers(&mut new_pos, slider);
            }
        }
        
        if new_pos.piece_at(legal_move.1) == Some((4, color)) {
            match new_pos.piece_at(legal_move.1).unwrap().1 {
                Color::White => {
                    if new_pos.attacked_by_black.contains(king_square) {
                        moves_to_remove.push(*legal_move);
                        continue;
                    }
                },
                Color::Black => {
                    if new_pos.attacked_by_white.contains(king_square) {
                        moves_to_remove.push(*legal_move);
                        continue;
                    }
                }
            
            }
        }
    }
    legal_moves.retain(|&x| !moves_to_remove.contains(&x));
    legal_moves
}