use types::position::Position;
use types::Color;
use types::bitboard::BitBoard;
// use types::state::State;
use types::square::Square;
use crate::movegen;

/* This combines a position and a state into a game structure,
/ while still aiming to keep overall complexity low. */

/* This double serves as a check detection - if this returns None, the king of color: Color is not in check in the given position.
/ Uses movegen to see if any piece of the opposite color could move to the king's square. */
pub fn get_attackers_on_king(color: Color, pos: Position) -> Option<BitBoard> {
    let king_bitboard = pos.color_bitboards[color as usize] & pos.piece_boards[4];
    let king_square: Square = king_bitboard.squares_from_bb()[0];
    let mut attackers = BitBoard::empty();

    // Retrieve all slider pieces of the opposing color
    let slider_pieces = pos.piece_boards[0] | pos.piece_boards[2] | pos.piece_boards[3] & pos.color_bitboards[!color as usize];
    if !slider_pieces.is_empty() {    
        let slider_squares = slider_pieces.squares_from_bb();
        for square in slider_squares {
            let piece = pos.piece_at(square).unwrap().0;
            let piece_moves = match piece {
                0 => movegen::get_rook_moves_from_blockers(square, pos.color_bitboards[0] | pos.color_bitboards[1]),
                2 => movegen::get_bishop_moves_from_blockers(square, pos.color_bitboards[0] | pos.color_bitboards[1]),
                3 => movegen::get_queen_moves_from_blockers(square, pos.color_bitboards[0] | pos.color_bitboards[1]),
                _ => BitBoard::empty(),
            };
            if piece_moves.contains(king_square) {
                attackers |= BitBoard::from_square(square);
            }
        }
    }

    // Retrieve all knight pieces of the opposing color
    let knight_pieces = pos.piece_boards[1] & pos.color_bitboards[!color as usize];
    if !knight_pieces.is_empty() {
        let knight_squares = knight_pieces.squares_from_bb();
        for square in knight_squares {
            let piece = pos.piece_at(square).unwrap().0;
            let piece_moves = match piece {
                1 => movegen::get_knight_moves(square, &pos),
                _ => BitBoard::empty(),
            };
            if piece_moves.contains(king_square) {
                attackers |= BitBoard::from_square(square);
            }
        }
    }

    // Retrieve all pawn pieces of the opposing color
    // TODO: this feels very performance expensive, there has to be a smarter way to do this
    let pawn_pieces = pos.piece_boards[5] & pos.color_bitboards[!color as usize];
    if !pawn_pieces.is_empty() {
        let pawn_squares = pawn_pieces.squares_from_bb();
        for square in pawn_squares {
            let piece = pos.piece_at(square).unwrap().0;
            let piece_moves = match piece {
                5 => movegen::get_pawn_moves(square, &pos),
                _ => BitBoard::empty(),
            };
            if piece_moves.contains(king_square) {    
                let target_squares = piece_moves.squares_from_bb();
                for target_square in target_squares {
                    if target_square == king_square {
                        // Only consider diagonal moves as attacks
                        if !target_square.file_index() == king_square.file_index() {                    
                            attackers |= BitBoard::from_square(square);
                        }
                    }
                }
            }
        }
    }

    if attackers.is_empty() {
        None
    } else {
        Some(attackers)
    }
}

// Checks move legality by simulating the move and checking if the king would be in check afterwards
pub fn is_legal_move(from: Square, to: Square, pos: &Position) -> bool {
    let color = pos.piece_at(from).unwrap().1;
    let new_pos = pos.simulate_move(from, to);
    if let Some(_attackers) = get_attackers_on_king(color, new_pos) {
        false
    } else {
        true
    }
}

// Checks if a user provided move is legal, returns an error message if not
pub fn is_legal_player_move(from: Square, to: Square, pos: &Position) -> Result<(), String> {
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
        let new_pos = pos.simulate_move(from, to);
        let color = pos.piece_at(from).unwrap().1;
        // Check if the move would put the king in check
        if let Some(_attackers) = get_attackers_on_king(color, new_pos) {
            Err("This move would put your king in check!".to_string())
        } else {
            Ok(())
        }
    } else {
        Err("This move is not possible for the selected piece!".to_string())
    }
}