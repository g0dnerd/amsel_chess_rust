use types::position::Position;
use types::Color;
use types::bitboard::BitBoard;
use types::square::*;
use crate::movegen;
use rand::seq::SliceRandom;

// Passes updated attack bitboards to the position struct from the given position and move
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

// Updates the blockers for all sliders in the given bitboard
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

// Updates the attacked pieces for all sliders in the given bitboard
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
        let is_slider = (pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3]).contains(from);
        let blocked_sliders = new_pos.is_blocking_slider(from);
        new_pos.make_move(&from, &to);
        new_pos.attack_bitboards[from as usize] = BitBoard::empty();
        let color = pos.piece_at(from).unwrap().1;// If the moved piece was attacking at least 1 square, update attackers

        attacks_from_square(&mut new_pos, from, to);
        
        let affected_sliders = new_pos.is_blocking_slider(to);
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
                let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[0]).squares_from_bb()[0];
                if new_pos.attacked_by_black.contains(king_square) {
                    // Update the corresponding entry in the hash map by removing target_square from the entry
                    return Err("This move would put your king in check!".to_string());
                }
            },
            Color::Black => {
                let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[1]).squares_from_bb()[0];
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

pub fn make_player_move(pos: &mut Position, from: Square, to: Square) -> Result<(), &'static str> {
    // Check if the targetted piece contains a piece of the active player's color
    if let Some(piece) = pos.piece_at(from) {
        if piece.1 != pos.state.active_player {
            return Err("Piece does not belong to active player.");
        }
    } else {
        return Err("Illegal move: no piece on origin square.");
    }

    let legal_moves = 
        movegen::get_all_legal_moves_for_color(pos.state.active_player, &pos);
    
    if legal_moves.is_empty() {
        match pos.state.active_player {
            types::Color::White => {
                let king_square = (pos.piece_bitboards[4] & pos.color_bitboards[0]).squares_from_bb()[0];
                if pos.attacked_by_black.contains(king_square) {
                    println!("Black wins by checkmate.");
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::BLACK_VICTORY);
                } else {
                    println!("Stalemate.");
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::STALEMATE);
                }
            },
            types::Color::Black => {
                let king_square = (pos.piece_bitboards[4] & pos.color_bitboards[1]).squares_from_bb()[0];
                if pos.attacked_by_white.contains(king_square) {
                    println!("White wins by checkmate.");
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::WHITE_VICTORY);
                } else {
                    println!("Stalemate.");
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::STALEMATE);
                }
            }
        }
    }

    // Check if the move is legal by checking if it is in the list of legal moves
    if !legal_moves.contains_key(&from) {
        return Err("Illegal move: no movable piece on origin square.");
    }
    let to_squares = legal_moves.get(&from).unwrap().squares_from_bb();
    if !to_squares.contains(&to) {
        return Err("Illegal move: piece cannot move to target square.");
    }

    // Bitboard of sliders that no longer have the path blocked by the moved piece
    let freed_sliders = pos.is_blocking_slider(from);
    let is_slider = (pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3]).contains(from);
    let is_pawn = pos.piece_bitboards[5].contains(from);
    pos.make_move(&from, &to);
    
    // Bitboard of sliders that now have their path blocked by the moved piece
    let blocked_sliders = pos.is_blocking_slider(to);

    // Update the attack map for the moved piece
    attacks_from_square(pos, from, to);

    // If there are any sliders that are no longer blocked, update their attack and blocker maps
    if !freed_sliders.is_empty() {
        pos.slider_blockers[from as usize] = BitBoard::empty();
        update_sliders(pos, freed_sliders);
    }
    // If there are newly blocked sliders, update their attack and blocker maps
    if !blocked_sliders.is_empty() {
        pos.slider_blockers[from as usize] = BitBoard::empty();
        update_sliders(pos, blocked_sliders);
    }
    // If the moved piece is a slider, update its attack and blocker maps
    if is_slider {
        pos.slider_blockers[from as usize] = BitBoard::empty();
        update_sliders(pos, BitBoard::from_square(to));
    }

    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        let rank = to.rank();
        match !pos.state.active_player {
            types::Color::White => {
                if rank == Rank::Eighth {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                    update_sliders(pos, BitBoard::from_square(to));
                    println!("Blockers for new slider have been updated: {:?}", pos.slider_blockers);
                    println!("Attackers have been updated: {:?}", pos.attack_bitboards);
                }
            },
            types::Color::Black => {
                if rank == Rank::First {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                    update_sliders(pos, BitBoard::from_square(to));
                    println!("Blockers for new slider have been updated: {:?}", pos.slider_blockers);
                    println!("Attackers have been updated: {:?}", pos.attack_bitboards);
                }
            }
        }
    }

    Ok(())
}

pub fn make_engine_move(pos: &mut Position) {

    let legal_moves = 
        movegen::get_all_legal_moves_for_color(pos.state.active_player, &pos);

    // If the AI has no legal moves, change the game result to stalemate, draw or checkmate
    if legal_moves.is_empty() {
        match pos.state.active_player {
            types::Color::White => {
                let king_square = (pos.piece_bitboards[4] & pos.color_bitboards[0]).squares_from_bb()[0];
                if pos.attacked_by_black.contains(king_square) {
                    println!("Black wins by checkmate.");
                    println!("Complete attacker map: {:?}", pos.attack_bitboards);
                    println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_black.squares_from_bb());
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::BLACK_VICTORY);
                } else {
                    println!("Stalemate.");
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::STALEMATE);
                }
            },
            types::Color::Black => {
                let king_square = (pos.piece_bitboards[4] & pos.color_bitboards[1]).squares_from_bb()[0];
                if pos.attacked_by_white.contains(king_square) {
                    println!("White wins by checkmate.");
                    println!("Complete attacker map: {:?}", pos.attack_bitboards);
                    println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_white.squares_from_bb());
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::WHITE_VICTORY);
                } else {
                    println!("Stalemate.");
                    pos.print_position();
                    pos.state.game_result = types::state::GameResult(types::Results::STALEMATE);
                }
            }
        }
    }

    let squares: Vec<Square> = legal_moves
        .iter()
        .filter_map(|(square, moves)|
            if !moves.is_empty() { Some(*square) } else { None})
        .collect();

    // Randomly choose a move for the engine to make
    let mut rng = rand::thread_rng();
    if let Some(from) = squares.choose(&mut rng) {
        let target_square = legal_moves.get(from).unwrap().squares_from_bb();
        if let Some(target_square) = target_square.choose(&mut rng) {
            println!("AI move: {:?} {:?}", from, target_square);

            // Bitboard of sliders that no longer have their path blocked by the moved piece
            let blocked_sliders = pos.is_blocking_slider(*from);
            let is_slider = (pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3]).contains(*from);
            let is_pawn = pos.piece_bitboards[5].contains(*from);
            pos.make_move(from, target_square);
            
            // Bitboard of sliders that now have their path blocked by the moved piece
            let affected_sliders = pos.is_blocking_slider(*target_square);
            
            // Update the attack map for the moved piece
            attacks_from_square(pos, *from, *target_square);

            // If there are any sliders that are no longer blocked, update their attack and blocker maps
            if !blocked_sliders.is_empty() {
                update_sliders(pos, blocked_sliders);
            }
            // If there are newly blocked sliders, update their attack and blocker maps
            if !affected_sliders.is_empty() {
                update_sliders(pos, affected_sliders);
            }
            // If the moved piece is a slider, update its attack and blocker maps
            if is_slider {
                pos.slider_blockers[*from as usize] = BitBoard::empty();
                update_sliders(pos, BitBoard::from_square(*target_square));
            }
                
            // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
            if is_pawn {
                let rank = target_square.rank();
                match !pos.state.active_player {
                    types::Color::White => {
                        if rank == Rank::Eighth {
                            pos.promote_pawn(*target_square, types::Piece::QUEEN);
                            update_sliders(pos, BitBoard::from_square(*target_square));
                            println!("Blockers for new slider have been updated: {:?}", pos.slider_blockers);
                            println!("Attackers have been updated: {:?}", pos.attack_bitboards);
                        }
                    },
                    types::Color::Black => {
                        if rank == Rank::First {
                            pos.promote_pawn(*target_square, types::Piece::QUEEN);
                            update_sliders(pos, BitBoard::from_square(*target_square));
                            println!("Blockers for new slider have been updated: {:?}", pos.slider_blockers);
                            println!("Attackers have been updated: {:?}", pos.attack_bitboards);
                        }
                    }
                }
            }
        }
    }



}