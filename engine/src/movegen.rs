use crate::{
    magics::*,
    game
};
use precompute::magics::MagicTableEntry;
use types::{
    bitboard::BitBoard,
    position::Position,
    Castling,
    Color,
    types_utils::*,
};

/* Use our ray-scanning algorithm from the precompute module to get potential blockers for a piece,
/ then AND the result with the all_pieces BitBoard to get the actual blockers. */
pub fn get_all_actual_blockers(
    piece: u8,
    square: u8,
    position: &Position,
) -> BitBoard {
    let mut blockers = match piece {
        0 => BitBoard::from_u64(ROOK_MAGICS[square as usize].mask),
        2 => BitBoard::from_u64(BISHOP_MAGICS[square as usize].mask),
        3 => BitBoard::from_u64(ROOK_MAGICS[square as usize].mask)
            | BitBoard::from_u64(BISHOP_MAGICS[square as usize].mask),
        _ => panic!("Piece different than slider passed to movegen::get_all_actual_blockers()"),
    };
    blockers &= position.all_pieces();
    blockers
}

pub fn pseudolegal_slider_moves(piece: u8, square: u8, pos: &Position) -> BitBoard {
    let blockers = get_all_actual_blockers(piece, square, pos);
    return match piece {
        0 => BitBoard::from_u64(ROOK_MOVES[magic_index(&ROOK_MAGICS[square as usize], blockers)]),
        2 => BitBoard::from_u64(BISHOP_MOVES[magic_index(&BISHOP_MAGICS[square as usize], blockers)]),
        3 => BitBoard::from_u64(ROOK_MOVES[magic_index(&ROOK_MAGICS[square as usize], blockers)]
            | BISHOP_MOVES[magic_index(&BISHOP_MAGICS[square as usize], blockers)]),
        _ => panic!("Piece different than slider passed to movegen::slider_moves()"),
    };
}

fn slider_moves_from_blockers(piece: u8, square: u8, blockers: BitBoard) -> BitBoard {
    let moves = match piece {
        0 => ROOK_MOVES[magic_index(&ROOK_MAGICS[square as usize], blockers)],
        2 => BISHOP_MOVES[magic_index(&BISHOP_MAGICS[square as usize], blockers)],
        3 => ROOK_MOVES[magic_index(&ROOK_MAGICS[square as usize], blockers)]
            | BISHOP_MOVES[magic_index(&BISHOP_MAGICS[square as usize], blockers)],
        _ => panic!("Piece different than slider passed to movegen::slider_moves()"),
    };
    BitBoard::from_u64(moves)
}

pub fn slider_moves(piece: u8, square: u8, pos: &Position) -> BitBoard {
    let blockers = get_all_actual_blockers(piece, square, pos);
    let color = pos.piece_color(square);
    slider_moves_from_blockers(piece, square, blockers) & !pos.color_bitboards[color as usize]
}

pub fn get_pseudolegal_knight_moves(square: u8) -> BitBoard {
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
        if let Some(offset_by_delta) = try_square_offset(square, dx, dy) {
            moves |= BitBoard::from_square(offset_by_delta);
        }
    }
    moves
}

pub fn pawn_attacks(position: &mut Position, square: u8) -> BitBoard {
    let mut moves = BitBoard::empty();
    let color = position.piece_at(square).unwrap().1;
    for &dx in &[-1, 1] {
        if let Some(offset_by_delta) = try_square_offset(square, dx, match color {
            Color::White => 1,
            Color::Black => -1,
        }){
            moves |= BitBoard::from_square(offset_by_delta);
        }
    }
    moves
}

#[inline]
fn magic_index(entry: &MagicTableEntry, blockers: BitBoard) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

pub fn get_rook_moves(square: u8, position: &Position) -> BitBoard {
    let blockers = get_all_actual_blockers(0, square, position);
    return slider_moves_from_blockers(0, square, blockers);
}

#[inline]
pub fn get_bishop_moves(square: u8, pos: &Position) -> BitBoard {
    // Handle potential errors when trying to unwrap a piece from an empty square
    slider_moves(2, square, pos)
}

#[inline]
pub fn get_queen_moves(square: u8, pos: &Position) -> BitBoard {
    slider_moves(3, square, pos)
}

pub fn get_knight_moves(square: u8, position: &Position) -> BitBoard {
    let mut moves = get_pseudolegal_knight_moves(square);
    let color = position.piece_at(square).unwrap().1;
    moves &= !position.color_bitboards[color as usize];
    moves
}

pub fn get_king_moves(square: u8, position: &Position) -> BitBoard {
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
        if let Some(offset_by_delta) = try_square_offset(square, dx, dy) {
            moves |= BitBoard::from_square(offset_by_delta);
        }
    }
    
    let color = position.piece_at(square).unwrap().1;

    if position.state.castling_rights.0 != Castling::NO_CASTLING && !position.check {
        match color {
            Color::White => {
                if position.state.castling_rights.0 & Castling::WHITE_KING_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(5).is_none()
                        && position.piece_at(6).is_none()
                    {
                        moves |= BitBoard::from_square(6);
                    }
                }
                if position.state.castling_rights.0 & Castling::WHITE_QUEEN_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(3).is_none()
                        && position.piece_at(2).is_none()
                        && position.piece_at(1).is_none()
                    {
                        moves |= BitBoard::from_square(2);
                    }
                }
            }
            Color::Black => {
                if position.state.castling_rights.0 & Castling::BLACK_KING_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(61).is_none()
                        && position.piece_at(62).is_none()
                    {
                        moves |= BitBoard::from_square(62);
                    }
                }
                if position.state.castling_rights.0 & Castling::BLACK_QUEEN_SIDE != Castling::NO_CASTLING {
                    if position.piece_at(59).is_none()
                        && position.piece_at(58).is_none()
                        && position.piece_at(57).is_none()
                    {
                        moves |= BitBoard::from_square(58);
                    }
                }
            
            }
        }
    }

    moves = moves & !position.color_bitboards[color as usize];
    moves
}

pub fn get_pawn_moves(square: u8, position: &Position) -> BitBoard {
    let mut moves = BitBoard::empty();
    let color = position.piece_at(square).unwrap().1;
    let direction = match color {
        Color::White => 1,
        Color::Black => -1,
    };
    let offset_by_delta = try_square_offset(square, 0, direction);
    if let Some(offset_by_delta) = offset_by_delta {
        if position.piece_at(offset_by_delta).is_none() {
            moves |= BitBoard::from_square(offset_by_delta);
            if (color == Color::White && square / 8 == 1)
                || (color == Color::Black && square / 8 == 6)
            {
                if let Some(offset_by_delta) = try_square_offset(offset_by_delta, 0, direction) {
                    if position.piece_at(offset_by_delta).is_none() {
                        moves |= BitBoard::from_square(offset_by_delta);
                    }
                }
            }
        }
    }
    for &dx in &[-1, 1] {
        if let Some(offset_by_delta) = try_square_offset(square, dx, direction) {
            if let Some((_piece, x_color)) = position.piece_at(offset_by_delta) {
                if color != x_color {
                    moves |= BitBoard::from_square(offset_by_delta);
                }
            } else {
                if position.en_passant_square == Some(offset_by_delta) {
                    moves |= BitBoard::from_square(offset_by_delta);
                }
            }
        }
    }
    moves
}

pub fn get_all_legal_moves_for_color(color: Color, pos: &mut Position) -> Vec<(u8, u8)> {
    if pos.check {
        return get_legal_moves_from_check(color, pos);
    }
    let mut moves: Vec<(u8, u8)> = Vec::new();

    // Iterate over all squares with a piece of the given color
    let mut squares = pos.color_bitboards[color as usize];
    while !squares.is_empty() {
        let square = squares.trailing_zeros() as u8;
        let piece = pos.piece_at(square).unwrap().0;
        let piece_moves = match piece {
            0 | 2 | 3 => slider_moves(piece, square, pos),
            1 => get_knight_moves(square, pos),
            4 => get_king_moves(square, pos),
            5 => get_pawn_moves(square, pos),
            _ => BitBoard::empty(),
        };

        let mut piece_moves_iterator = piece_moves;
        while !piece_moves_iterator.is_empty() {
            let piece_move = piece_moves_iterator.trailing_zeros() as u8;
            moves.push((square, piece_move));
            piece_moves_iterator.clear_lsb();
        }
        squares.clear_lsb();
    }

    // Iterate over all moves and remove those that would put or leave the king in check
    let mut moves_to_remove: Vec<(u8, u8)> = Vec::new();
    for (from, to) in moves.iter() {
        let mut new_pos = pos.clone();
        let is_pawn = new_pos.piece_at(*from).unwrap().0 == 5;
        let is_king = new_pos.piece_at(*from).unwrap().0 == 4;

        // If the move would put a king next to another king, remove it
        if is_king {
            let opposite_king = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[!color as usize]).trailing_zeros() as i8;
            if (opposite_king - *to as i8).abs() < 2 {
                moves_to_remove.push((*from, *to));
                continue;
            } else if (opposite_king - *to as i8).abs() > 6 && (opposite_king - *to as i8).abs() < 10 {
                moves_to_remove.push((*from, *to));
                continue;
            }
        }

        let mut attackers_to_update = BitBoard::empty();

        // Remove the move if it would castle through check
        if is_king && (*to as i8 - *from as i8).abs() == 2 {
            match to {
                2 | 58 => {
                    if pos.is_square_attacked_by_color(*to, !color) ||
                            pos.is_square_attacked_by_color(*to - 1, !color) ||
                            pos.is_square_attacked_by_color(*to - 2, !color) {
                        moves_to_remove.push((*from, *to));
                        continue;
                    }
                },
                6 | 62 => {
                    if pos.is_square_attacked_by_color(*to, !color) ||
                            pos.is_square_attacked_by_color(*to + 1, !color) {
                        moves_to_remove.push((*from, *to));
                        continue;
                    }
                },
                _ => (),
            }
        }

        // List of sliders that after the move no longer have their path blocker by the moved piece
        let freed_sliders = game::get_attacking_sliders(&mut new_pos, *from);
        attackers_to_update |= freed_sliders;

        new_pos.make_move(&from, &to);

        // If the move is a castling move, move the rook as well
        if is_king && ((*from as i8 % 8) - (*to as i8 % 8)).abs() > 1 {
            if from > to {
                new_pos.make_castling_move(&(to - 2), &(from - 1));
                attackers_to_update |= BitBoard::from_square(from - 1);
                attackers_to_update ^= BitBoard::from_square(to - 1);
            } else {
                new_pos.make_castling_move(&(to + 1), &(from + 1));
                attackers_to_update |= BitBoard::from_square(from + 1);
                attackers_to_update ^= BitBoard::from_square(to + 1);
            
            }
        }

        // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
        if is_pawn && (color as usize == 0 && to / 8 == 7 || color as usize == 1 && to / 8 == 0) {
            new_pos.promote_pawn(*to, types::Piece::QUEEN);
        }

        attackers_to_update |= BitBoard::from_square(*to);

        // List of sliders that now have their path blocked by the moved piece
        attackers_to_update |= game::get_attacking_sliders(&mut new_pos, *to);

        if !attackers_to_update.is_empty() {
            game::update_attackers(&mut new_pos, attackers_to_update);
        }

        // If after these updates, the king is in the list of attacked squares, the move is illegal
        let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[color as usize]).trailing_zeros() as u8;
        if new_pos.is_square_attacked_by_color(king_square, !color) {
            moves_to_remove.push((*from, *to));
            continue;
        }
    }
    // Remove all illegal moves
    moves.retain(|&x| !moves_to_remove.contains(&x));
    moves
}

/* Movegen method with reduced scope since when in check, the only possible pieces with available moves are then king,
/ pieces that can capture the piece giving check or pieces that can block the check */
fn get_legal_moves_from_check(color: Color, pos: &mut Position) -> Vec<(u8, u8)> {
    let mut moves: Vec<(u8, u8)> = Vec::new();

    // Iterate over all squares with a piece of the given color
    let mut squares = pos.color_bitboards[color as usize];
    while squares.0 != 0 {
        let square = squares.trailing_zeros() as u8;
        let piece = pos.piece_at(square).unwrap().0;
        let mut piece_moves = match piece {
            0 | 2 | 3 => slider_moves(piece, square, pos),
            1 => get_knight_moves(square, pos),
            4 => get_king_moves(square, pos),
            5 => get_pawn_moves(square, pos),
            _ => BitBoard::empty(),
        };
        squares.clear_lsb();
        while piece_moves.0 != 0 {
            let piece_move = piece_moves.trailing_zeros() as u8;
            moves.push((square, piece_move));
            piece_moves.clear_lsb();
        }
    }

    // Iterate over all moves and remove those that don't end the check
    let mut moves_to_remove: Vec<(u8, u8)> = Vec::new();
    for (from, to) in moves.iter() {
        
        let mut new_pos = pos.clone();
        let is_king = new_pos.piece_at(*from).unwrap().0 == 4;
        
        // If the move would put a king next to another king, remove it
        if is_king {
            let opposite_king = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[!color as usize]).trailing_zeros() as i8;
            if (opposite_king - *to as i8).abs() < 2 {
                moves_to_remove.push((*from, *to));
                continue;
            } else if (opposite_king - *to as i8).abs() > 6 && (opposite_king - *to as i8).abs() < 10 {
                moves_to_remove.push((*from, *to));
                continue;
            }
        }

        let mut attackers_to_update = BitBoard::empty();

        attackers_to_update |= game::get_attacking_sliders(&mut new_pos, *from);

        new_pos.make_move(&from, &to);

        attackers_to_update |= BitBoard::from_square(*to);

        // List of sliders that now have their path blocked by the moved piece
        attackers_to_update |= game::get_attacking_sliders(&mut new_pos, *to);

        if !attackers_to_update.is_empty() {
            game::update_attackers(&mut new_pos, attackers_to_update);
        }

        // If after these updates, the king is in the list of attacked squares, the move is illegal
        let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[color as usize]).trailing_zeros() as u8;
        if new_pos.is_square_attacked_by_color(king_square, !color) {
            moves_to_remove.push((*from, *to));
            continue;
        }
    }
    // Remove all tuples from moves that are in moves_to_remove
    moves.retain(|&x| !moves_to_remove.contains(&x));
    moves
}

pub fn get_all_captures_for_color(color: Color, pos: &mut Position) -> Vec<(u8, u8)> {
    let mut moves = get_all_legal_moves_for_color(color, pos);
    moves.retain(|&(_from, to)| pos.is_capture(&to));
    moves
}