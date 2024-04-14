use types::position::Position;
use types::Color;
use types::bitboard::BitBoard;
use types::square::*;
use types::state::GameResult;
use crate::negamax;
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
        pos.update_attacks_from_square(from, to, attacks);
    }
}

// Updates the blockers for all sliders in the given bitboard
pub fn update_slider_blockers(pos: &mut Position, affected: BitBoard) {
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
pub fn update_slider_attacks(pos: &mut Position, affected: BitBoard) {
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
        movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    
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
    if !legal_moves.contains(&(from, to)) {
        return Err("Not a legal move.");
    }

    // Bitboard of sliders that no longer have the path blocked by the moved piece
    let freed_sliders = pos.is_blocking_slider(from);
    let is_slider = (pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3]).contains(from);
    let is_pawn = pos.piece_bitboards[5].contains(from);
    pos.make_move(&from, &to);

    // If the move is a castling move, move the rook as well
    if from == Square::E1 && to == Square::G1 {
        pos.make_move(&Square::H1, &Square::F1);
        update_slider_attacks(pos, BitBoard::from_square(Square::F1));
        update_slider_blockers(pos, BitBoard::from_square(Square::F1));
    } else if from == Square::E1 && to == Square::C1 {
        pos.make_move(&Square::A1, &Square::D1);
        update_slider_attacks(pos, BitBoard::from_square(Square::D1));
        update_slider_blockers(pos, BitBoard::from_square(Square::D1));
    } else if from == Square::E8 && to == Square::G8 {
        pos.make_move(&Square::H8, &Square::F8);
        update_slider_attacks(pos, BitBoard::from_square(Square::F8));
        update_slider_blockers(pos, BitBoard::from_square(Square::F8));
    } else if from == Square::E8 && to == Square::C8 {
        pos.make_move(&Square::A8, &Square::D8);
        update_slider_attacks(pos, BitBoard::from_square(Square::D8));
        update_slider_blockers(pos, BitBoard::from_square(Square::D8));
    }
    
    // Bitboard of sliders that now have their path blocked by the moved piece
    let blocked_sliders = pos.is_blocking_slider(to);

    // If there are any sliders that are no longer blocked, update their attack and blocker maps
    if !freed_sliders.is_empty() {
        pos.slider_blockers[from as usize] = BitBoard::empty();
        update_slider_attacks(pos, freed_sliders);
        update_slider_blockers(pos, freed_sliders);
    }
    // If there are newly blocked sliders, update their attack and blocker maps
    if !blocked_sliders.is_empty() {
        update_slider_blockers(pos, blocked_sliders);
        update_slider_attacks(pos, blocked_sliders);
    }
    // If the moved piece is a slider, update its attack and blocker maps
    if is_slider {
        pos.slider_blockers[from as usize] = BitBoard::empty();
        update_slider_blockers(pos, BitBoard::from_square(to));
        update_slider_attacks(pos, BitBoard::from_square(to));
    } else {
        // Update the attack map for the moved piece
        attacks_from_square(pos, from, to);
    }

    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        let rank = to.rank();
        match !pos.state.active_player {
            types::Color::White => {
                if rank == Rank::Eighth {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                    update_slider_blockers(pos, BitBoard::from_square(to));
                    update_slider_attacks(pos, BitBoard::from_square(to));
                    println!("Blockers for new slider have been updated: {:?}", pos.slider_blockers);
                    println!("Attackers have been updated: {:?}", pos.attack_bitboards);
                }
            },
            types::Color::Black => {
                if rank == Rank::First {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                    update_slider_blockers(pos, BitBoard::from_square(to));
                    update_slider_attacks(pos, BitBoard::from_square(to));
                    println!("Blockers for new slider have been updated: {:?}", pos.slider_blockers);
                    println!("Attackers have been updated: {:?}", pos.attack_bitboards);
                }
            }
        }
    }

    let king_square = match pos.state.active_player {
        Color::White => (pos.piece_bitboards[4] & pos.color_bitboards[0]).squares_from_bb()[0],
        Color::Black => (pos.piece_bitboards[4] & pos.color_bitboards[1]).squares_from_bb()[0],
    };
    match pos.state.active_player {
        Color::White => pos.check = pos.attacked_by_black.contains(king_square),
        Color::Black => pos.check = pos.attacked_by_white.contains(king_square),
    }

    Ok(())
}

pub fn make_random_engine_move(pos: &mut Position) -> Option<GameResult> {

    let legal_moves = 
        movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);

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
                    let result = types::state::GameResult(types::Results::BLACK_VICTORY);
                    return Some(result);
                } else {
                    println!("Stalemate.");
                    pos.print_position();
                    let result = types::state::GameResult(types::Results::STALEMATE);
                    return Some(result);
                }
            },
            types::Color::Black => {
                let king_square = (pos.piece_bitboards[4] & pos.color_bitboards[1]).squares_from_bb()[0];
                if pos.attacked_by_white.contains(king_square) {
                    println!("White wins by checkmate.");
                    println!("Complete attacker map: {:?}", pos.attack_bitboards);
                    println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_white.squares_from_bb());
                    pos.print_position();
                    let result = types::state::GameResult(types::Results::WHITE_VICTORY);
                    return Some(result);
                } else {
                    println!("Stalemate.");
                    pos.print_position();
                    let result = types::state::GameResult(types::Results::STALEMATE);
                    return Some(result);
                }
            }
        }
    }

    // Randomly choose a move for the engine to make
    let mut rng = rand::thread_rng();
    let random_move = legal_moves.choose(&mut rng);
    let (from, target_square) = random_move.unwrap();
    println!("AI move: {:?} {:?}", from, target_square);

    // Bitboard of sliders that no longer have their path blocked by the moved piece
    let blocked_sliders = pos.is_blocking_slider(*from);
    let is_slider = (pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3]).contains(*from);
    let is_pawn = pos.piece_bitboards[5].contains(*from);
    pos.make_move(from, target_square);
    
    // Bitboard of sliders that now have their path blocked by the moved piece
    let affected_sliders = pos.is_blocking_slider(*target_square);

    // If there are any sliders that are no longer blocked, update their attack and blocker maps
    if !blocked_sliders.is_empty() {
        update_slider_attacks(pos, blocked_sliders);
        update_slider_blockers(pos, blocked_sliders);
    }
    // If there are newly blocked sliders, update their attack and blocker maps
    if !affected_sliders.is_empty() {
        update_slider_blockers(pos, affected_sliders);
        update_slider_attacks(pos, affected_sliders);
    }
    // If the moved piece is a slider, update its attack and blocker maps
    if is_slider {
        pos.slider_blockers[*from as usize] = BitBoard::empty();
        update_slider_blockers(pos, BitBoard::from_square(*target_square));
        update_slider_attacks(pos, BitBoard::from_square(*target_square));
    } else {        
        // Update the attack map for the moved piece
        attacks_from_square(pos, *from, *target_square);
    }
        
    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        let rank = target_square.rank();
        match !pos.state.active_player {
            types::Color::White => {
                if rank == Rank::Eighth {
                    pos.promote_pawn(*target_square, types::Piece::QUEEN);
                    update_slider_blockers(pos, BitBoard::from_square(*target_square));
                    update_slider_attacks(pos, BitBoard::from_square(*target_square));
                }
            },
            types::Color::Black => {
                if rank == Rank::First {
                    pos.promote_pawn(*target_square, types::Piece::QUEEN);
                    update_slider_blockers(pos, BitBoard::from_square(*target_square));
                    update_slider_attacks(pos, BitBoard::from_square(*target_square));
                }
            }
        }
    }
    None
}

pub fn make_engine_move(pos: &mut Position) -> Option<GameResult> {  
    let best_move = negamax::find_best_move(pos);
    let (from, target_square) = best_move;

    println!("AI move: {:?} {:?}", from, target_square);

    return make_specific_engine_move(pos, from, target_square);
}

pub fn would_give_check(pos: &mut Position, start: &Square, end: &Square) -> bool {
    let mut new_pos = pos.clone();
    let color = new_pos.piece_at(*start).unwrap().1;

    // List of sliders that after the move no longer have their path blocker by the moved piece
    let blocked_sliders = new_pos.is_blocking_slider(*start);
    let is_slider = (pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3]).contains(*start);
    new_pos.make_move(start, end);

    // If the moved piece was blocking a slider, update those sliders
    if !blocked_sliders.is_empty() {
        update_slider_attacks(&mut new_pos, blocked_sliders);
        update_slider_blockers(&mut new_pos, blocked_sliders);
    }

    if is_slider {
        new_pos.slider_blockers[*start as usize] = BitBoard::empty();
        update_slider_blockers(&mut new_pos, BitBoard::from_square(*end));
        update_slider_attacks(&mut new_pos, BitBoard::from_square(*end)); 
    } else {
        // update attackers for the moved piece
        attacks_from_square(&mut new_pos, *start, *end);
    }

    // If after these updates, the enemy king is in the list of attacked squares, the move gives check
    match !color {
        Color::White => {
            let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[0]).squares_from_bb()[0];
            if new_pos.attacked_by_black.contains(king_square) {
                return true;
            }
        },
        Color::Black => {
            let king_square = (new_pos.piece_bitboards[4] & new_pos.color_bitboards[1]).squares_from_bb()[0];
            if new_pos.attacked_by_white.contains(king_square) {
                return true;
            }
        }
    }
    false
}

pub fn is_in_checkmate(pos: &mut Position) -> bool {
    if pos.check {
        let legal_moves = movegen::get_legal_moves_from_check(pos.state.active_player, pos);
        if legal_moves.is_empty() {
            return true;
        }
    }
    return false;
}

pub fn make_specific_engine_move(pos: &mut Position, from: Square, target_square: Square) -> Option<GameResult> {
    // Bitboard of sliders that no longer have their path blocked by the moved piece
    let blocked_sliders = pos.is_blocking_slider(from);
    let is_slider = (pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3]).contains(from);
    let is_pawn = pos.piece_bitboards[5].contains(from);
        
    pos.make_move(&from, &target_square);

    // If the move is a castling move, move the rook as well
    if from == Square::E1 && target_square == Square::G1 {
        pos.make_move(&Square::H1, &Square::F1);
        update_slider_attacks(pos, BitBoard::from_square(Square::F1));
        update_slider_blockers(pos, BitBoard::from_square(Square::F1));
    } else if from == Square::E1 && target_square == Square::C1 {
        pos.make_move(&Square::A1, &Square::D1);
        update_slider_attacks(pos, BitBoard::from_square(Square::D1));
        update_slider_blockers(pos, BitBoard::from_square(Square::D1));
    } else if from == Square::E8 && target_square == Square::G8 {
        pos.make_move(&Square::H8, &Square::F8);
        update_slider_attacks(pos, BitBoard::from_square(Square::F8));
        update_slider_blockers(pos, BitBoard::from_square(Square::F8));
    } else if from == Square::E8 && target_square == Square::C8 {
        pos.make_move(&Square::A8, &Square::D8);
        update_slider_attacks(pos, BitBoard::from_square(Square::D8));
        update_slider_blockers(pos, BitBoard::from_square(Square::D8));
    }
    
    // Bitboard of sliders that now have their path blocked by the moved piece
    let affected_sliders = pos.is_blocking_slider(target_square);
    
    // If there are any sliders that are no longer blocked, update their attack and blocker maps
    if !blocked_sliders.is_empty() {
        update_slider_attacks(pos, blocked_sliders);
        update_slider_blockers(pos, blocked_sliders);
    }
    // If there are newly blocked sliders, update their attack and blocker maps
    if !affected_sliders.is_empty() {
        update_slider_blockers(pos, affected_sliders);
        update_slider_attacks(pos, affected_sliders);
    }
    // If the moved piece is a slider, update its attack and blocker maps
    if is_slider {
        pos.slider_blockers[from as usize] = BitBoard::empty();
        update_slider_blockers(pos, BitBoard::from_square(target_square));
        update_slider_attacks(pos, BitBoard::from_square(target_square));
    } else {
        // If not yet done, update the attack map for the moved piece
        attacks_from_square(pos, from, target_square);
    }
        
    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        let rank = target_square.rank();
        match !pos.state.active_player {
            types::Color::White => {
                if rank == Rank::Eighth {
                    pos.promote_pawn(target_square, types::Piece::QUEEN);
                    update_slider_blockers(pos, BitBoard::from_square(target_square));
                    update_slider_attacks(pos, BitBoard::from_square(target_square));
                }
            },
            types::Color::Black => {
                if rank == Rank::First {
                    pos.promote_pawn(target_square, types::Piece::QUEEN);
                    update_slider_blockers(pos, BitBoard::from_square(target_square));
                    update_slider_attacks(pos, BitBoard::from_square(target_square));
                }
            }
        }
    }

    let king_square = match pos.state.active_player {
        Color::White => (pos.piece_bitboards[4] & pos.color_bitboards[0]).squares_from_bb()[0],
        Color::Black => (pos.piece_bitboards[4] & pos.color_bitboards[1]).squares_from_bb()[0],
    };
    match pos.state.active_player {
        Color::White => pos.check = pos.attacked_by_black.contains(king_square),
        Color::Black => pos.check = pos.attacked_by_white.contains(king_square),
    }

    None
}