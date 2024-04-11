use types::position::Position;
use types::Color;
use types::bitboard::BitBoard;
use types::square::Square;
use crate::movegen;

pub fn attacks_from_square(pos: &mut Position, from: Square, to: Square) {
    let mut attacks = BitBoard::empty();
    if let Some(piece) = pos.piece_at(to) {
        match piece.0 {
            0 => attacks = movegen::get_rook_moves(to, pos),
            1 => attacks = movegen::get_knight_moves(to, pos),
            2 => attacks = movegen::get_bishop_moves(to, pos),
            3 => attacks = movegen::get_queen_moves(to, pos),
            4 => attacks = movegen::get_king_moves(to, pos),
            5 => attacks = movegen::pawn_attacks(pos, to),
            _ => (),
        }
    }
    pos.update_attacks_from_square(from, to, attacks);
}

pub fn update_sliders(pos: &mut Position, affected: BitBoard) {
    update_slider_blockers(pos, affected);
    update_slider_attacks(pos, affected);
}

fn update_slider_blockers(pos: &mut Position, affected: BitBoard) {
    let mut blockers = BitBoard::empty();
    for square in affected.squares_from_bb() {
        if let Some(piece) = pos.piece_at(square) {
            match piece.0 {
                0 => blockers |= movegen::get_actual_blockers(&[(0, 1), (0, -1), (1, 0), (-1, 0)], square, pos),
                2 => blockers |= movegen::get_actual_blockers(&[(1, 1), (1, -1), (-1, 1), (-1, -1)], square, pos),
                3 => blockers |= movegen::get_actual_blockers(&[(1, 1), (1, -1), (-1, 1), (-1, -1), (0, 1), (0, -1), (1, 0), (-1, 0)], square, pos),
                _ => (),
            }
            pos.update_slider_blockers(square, blockers);
        }
    }
}

fn update_slider_attacks(pos: &mut Position, affected: BitBoard) {
    let mut attacks = BitBoard::empty();
    for square in affected.squares_from_bb() {
        if let Some(piece) = pos.piece_at(square) {
            match piece.0 {
                0 => attacks = movegen::get_rook_moves(square, pos),
                2 => attacks = movegen::get_bishop_moves(square, pos),
                3 => attacks = movegen::get_queen_moves(square, pos),
                _ => (),
            }
            pos.update_attacks_from_square(square, square, attacks);
        }
    }
}


// Checks if a user provided move is legal, returns an error message if not
pub fn is_legal_move(from: Square, to: Square, pos: &Position) -> Result<(), String> {
    let mut moves = BitBoard::empty();
    let active_color = pos.state.active_player;

    // Check if there is a piece on the selected start square
    if let Some(piece) = pos.piece_at(from) {
        let color = pos.piece_at(from).unwrap().1;
        // Check if the piece belongs to the active player
        if active_color != color {
            return Err("It is not your turn!".to_string());
        }
        match piece.0 {
            0 => moves = movegen::get_rook_moves(from, pos),
            1 => moves = movegen::get_knight_moves(from, pos),
            2 => moves = movegen::get_bishop_moves(from, pos),
            3 => moves = movegen::get_queen_moves(from, pos),
            4 => moves = movegen::get_king_moves(from, pos),
            5 => moves = movegen::get_pawn_moves(from, pos),
            _ => (),
        }
    } else {
        return Err("There is no piece on this square!".to_string());
    }

    // Check if the move is possible for the selected piece
    if moves.contains(to) {
        let mut new_pos = pos.clone();
        let is_slider = (pos.piece_boards[0] | pos.piece_boards[2] | pos.piece_boards[3]).contains(from);
        let is_capture = pos.piece_at(to).is_some();
        let blocked_sliders = new_pos.is_blocking_slider(from);
        new_pos.make_move(&from, &to);
        new_pos.attack_bitboards[from as usize] = BitBoard::empty();
        let color = pos.piece_at(from).unwrap().1;// If the moved piece was attacking at least 1 square, update attackers
        if !new_pos.attack_bitboards[from as usize].is_empty() || is_capture {
            attacks_from_square(&mut new_pos, from, to);
        }
        let affected_sliders = new_pos.is_square_attacked_by_slider(to);
        // If the moved piece was blocking or is now blocking a slider, update attackers
        if !blocked_sliders.is_empty() {
            update_sliders(&mut new_pos, blocked_sliders);                
        }   
        if !affected_sliders.is_empty() {
            update_sliders(&mut new_pos, affected_sliders);
        }
        if is_slider {
            new_pos.slider_blockers[from as usize] = BitBoard::empty();
            update_sliders(&mut new_pos, BitBoard::from_square(to));
        }
        // Check if the move would put the king in check
        match color {
            Color::White => {
                let king_square = (new_pos.piece_boards[4] & new_pos.color_bitboards[0]).squares_from_bb()[0];
                if new_pos.attacked_by_black.contains(king_square) {
                    // Update the corresponding entry in the hash map by removing target_square from the entry
                    return Err("This move would put your king in check!".to_string());
                }
            },
            Color::Black => {
                let king_square = (new_pos.piece_boards[4] & new_pos.color_bitboards[1]).squares_from_bb()[0];
                if new_pos.attacked_by_white.contains(king_square) {
                    return Err("This move would put your king in check!".to_string());
                }
            }
        }
        // TODO: Check if the move is a castling move and if it would castle through check
    Ok(())
    } else {
        Err("This move is not possible for the selected piece!".to_string())
    }
}