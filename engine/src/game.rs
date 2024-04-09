use types::position::Position;
use types::Color;
use types::bitboard::BitBoard;
use types::square::Square;
use crate::movegen;

pub fn update_attacked_squares(pos: &mut Position) {
    let color = !pos.state.active_player;
    let slider_moves = movegen::get_all_slider_moves(color, &pos);
    let pawn_attacks = movegen::attacked_by_pawns(color, &pos);
    let knight_attacks = movegen::attacked_by_knights(color, &pos);
    let attacked_squares = slider_moves | pawn_attacks | knight_attacks;
    pos.update_attacked_squares(attacked_squares, color);
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
            0 => moves = movegen::get_rook_moves_from_position(from, pos),
            1 => moves = movegen::get_knight_moves(from, pos),
            2 => moves = movegen::get_bishop_moves_from_position(from, pos),
            3 => moves = movegen::get_queen_moves_from_position(from, pos),
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
        new_pos.make_move(&from, &to);
        let color = pos.piece_at(from).unwrap().1;
        // Check if the move would put the king in check
        match color {
            Color::White => {
                let king_square = pos.piece_boards[4].squares_from_bb()[0];
                if pos.attacked_by_black.contains(king_square) {
                    return Err("This move would put your king in check!".to_string());
                }
            },
            Color::Black => {
                let king_square = pos.piece_boards[4].squares_from_bb()[0];
                if pos.attacked_by_white.contains(king_square) {
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