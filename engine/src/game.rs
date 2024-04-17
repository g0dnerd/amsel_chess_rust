use std::panic;

use types::position::Position;
use types::Color;
use types::bitboard::BitBoard;
use types::square::*;
use crate::negamax;
use crate::movegen;

/* Find all sliders that are attacking the given square by using a fictitious queen that can move in all directions,
getting all possible moves for that piece and then filtering out the sliders from the resulting bitboard. */
pub fn get_attacking_sliders(pos: &mut Position, from: Square) -> BitBoard {
    let super_piece_directions: [(i8, i8); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let blocker_positions = movegen::get_all_actual_blockers(&super_piece_directions, from, pos);
    // TODO: This returns too many sliders - assumes every slider can move in all directions
    let mut attacking_sliders = movegen::get_queen_moves_from_blockers(from, blocker_positions);
    attacking_sliders &= pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3];
    attacking_sliders
}

pub fn update_attackers(pos: &mut Position, attackers: BitBoard) {
    /* println!("Update attackers looking at move history {:?}", pos.move_history);
    println!("Updating attackers on {:?}", attackers.squares_from_bb()); */
    let mut attacker_board = attackers;
    while attacker_board != BitBoard::empty() {
        let index = attacker_board.trailing_zeros() as usize;
        let attacker_square = Square::index(index);
        if let Some((piece, color)) = pos.piece_at(attacker_square) {
            match piece {
                0 => {
                    let attacks = movegen::get_rook_moves(attacker_square, pos);
                    pos.attack_bitboards[attacker_square as usize] = attacks;
                    match color {
                        Color::White => pos.update_attack_maps(attacker_square, attacks),
                        Color::Black => pos.update_attack_maps(attacker_square, attacks),
                    }
                },
                1 => {
                    let attacks = movegen::get_pseudolegal_knight_moves(attacker_square);
                    pos.attack_bitboards[attacker_square as usize] = attacks;
                    match color {
                        Color::White => pos.update_attack_maps(attacker_square, attacks),
                        Color::Black => pos.update_attack_maps(attacker_square, attacks),
                    }
                },
                2 => {
                    let attacks = movegen::get_bishop_moves(attacker_square, pos);
                    pos.attack_bitboards[attacker_square as usize] = attacks;
                    match color {
                        Color::White => pos.update_attack_maps(attacker_square, attacks),
                        Color::Black => pos.update_attack_maps(attacker_square, attacks),
                    }
                },
                3 => {
                    let attacks = movegen::get_queen_moves(attacker_square, pos);
                    pos.attack_bitboards[attacker_square as usize] = attacks;
                    match color {
                        Color::White => pos.update_attack_maps(attacker_square, attacks),
                        Color::Black => pos.update_attack_maps(attacker_square, attacks),
                    }
                },
                4 => {
                    let attacks = movegen::get_king_moves(attacker_square, pos);
                    pos.attack_bitboards[attacker_square as usize] = attacks;
                    match color {
                        Color::White => pos.update_attack_maps(attacker_square, attacks),
                        Color::Black => pos.update_attack_maps(attacker_square, attacks),
                    }
                },
                5 => {
                    let attacks = movegen::pawn_attacks(pos, attacker_square);
                    pos.attack_bitboards[attacker_square as usize] = attacks;
                    match color {
                        Color::White => pos.update_attack_maps(attacker_square, attacks),
                        Color::Black => pos.update_attack_maps(attacker_square, attacks),
                    }
                },
                _ => (),
            }
        } else {
            panic!("Trying to update attackers on empty square, move history is {:?} and attackers are {:?}",
                pos.move_history, attackers);
        }
        attacker_board.clear_lsb();
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

    let legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    if legal_moves.is_empty() {
        match pos.state.active_player {
            types::Color::White => {
                let king_square = (pos.piece_bitboards[4] & pos.color_bitboards[0]).squares_from_bb()[0];
                if pos.is_attacked_by_black(king_square) {
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
                if pos.is_attacked_by_white(king_square) {
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

    let mut attackers_to_update = BitBoard::empty();

    // Add sliders that are no longer blocked by the moved piece to the list of pieces to update
    attackers_to_update |= get_attacking_sliders(pos, from);

    let is_pawn = pos.piece_bitboards[5].contains(from);
    let is_king = pos.piece_bitboards[4].contains(from);
    pos.make_move(&from, &to);

    // Add sliders that now have their path blocked by the moved piece
    attackers_to_update |= get_attacking_sliders(pos, to);

    // If the move is a castling move, move the rook as well
    if is_king {
        if from == Square::E1 && to == Square::G1 {
            pos.make_castling_move(&Square::H1, &Square::F1);
            pos.state.switch_active_player();
            attackers_to_update |= BitBoard::from_square(Square::F1);
        } else if from == Square::E1 && to == Square::C1 {
            pos.make_castling_move(&Square::A1, &Square::D1);
            pos.state.switch_active_player();
            attackers_to_update |= BitBoard::from_square(Square::D1);
        } else if from == Square::E8 && to == Square::G8 {
            pos.make_castling_move(&Square::H8, &Square::F8);
            pos.state.switch_active_player();
            attackers_to_update |= BitBoard::from_square(Square::F8);
        } else if from == Square::E8 && to == Square::C8 {
            pos.make_castling_move(&Square::A8, &Square::D8);
            pos.state.switch_active_player();
            attackers_to_update |= BitBoard::from_square(Square::D8);
        }
    }

    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        let rank = to.rank();
        match !pos.state.active_player {
            types::Color::White => {
                if rank == Rank::Eighth {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                }
            },
            types::Color::Black => {
                if rank == Rank::First {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                }
            }
        }
    }
    
    attackers_to_update |= BitBoard::from_square(to);
    update_attackers(pos, attackers_to_update);

    let king_square = match pos.state.active_player {
        Color::White => (pos.piece_bitboards[4] & pos.color_bitboards[0]).squares_from_bb()[0],
        Color::Black => (pos.piece_bitboards[4] & pos.color_bitboards[1]).squares_from_bb()[0],
    };
    match pos.state.active_player {
        Color::White => pos.check = pos.is_attacked_by_black(king_square),
        Color::Black => pos.check = pos.is_attacked_by_white(king_square),
    }

    Ok(())
}

pub fn make_engine_move(pos: &mut Position, depth: u8) {  
    let best_move = negamax::find_best_move(pos, depth);
    let (from, target_square) = best_move;

    println!("AI move: {:?} {:?}", from, target_square);

    make_specific_engine_move(pos, from, target_square);
}

pub fn would_give_check(pos: &mut Position, from: &Square, to: &Square) -> bool {
    let mut new_pos = pos.clone();
    let mut attackers_to_update = BitBoard::empty();
    let is_pawn = new_pos.piece_bitboards[5].contains(*from);
    let is_king = new_pos.piece_bitboards[4].contains(*from);
    let color = new_pos.piece_at(*from).unwrap().1;

    // List of sliders that after the move no longer have their path blocker by the moved piece
    attackers_to_update |= get_attacking_sliders(&mut new_pos, *from);
    new_pos.make_move(from, to);

    // List of sliders that after the move have their path blocked by the moved piece
    attackers_to_update |= get_attacking_sliders(&mut new_pos, *to);

    // If the move is a castling move, move the rook as well
    if is_king {
        if *from == Square::E1 && *to == Square::G1 {
            new_pos.make_move(&Square::H1, &Square::F1);
            attackers_to_update |= BitBoard::from_square(Square::F1);
            attackers_to_update ^= BitBoard::from_square(Square::H1);
        } else if *from == Square::E1 && *to == Square::C1 {
            new_pos.make_move(&Square::A1, &Square::D1);
            attackers_to_update |= BitBoard::from_square(Square::D1);
            attackers_to_update ^= BitBoard::from_square(Square::A1);
        } else if *from == Square::E8 && *to == Square::G8 {
            new_pos.make_move(&Square::H8, &Square::F8);
            attackers_to_update |= BitBoard::from_square(Square::F8);
            attackers_to_update ^= BitBoard::from_square(Square::H8);
        } else if *from == Square::E8 && *to == Square::C8 {
            new_pos.make_move(&Square::A8, &Square::D8);
            attackers_to_update |= BitBoard::from_square(Square::D8);
            attackers_to_update ^= BitBoard::from_square(Square::A8);
        }
    }

    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        let rank = to.rank();
        match !new_pos.state.active_player {
            types::Color::White => {
                if rank == Rank::Eighth {
                    new_pos.promote_pawn(*to, types::Piece::QUEEN);
                }
            },
            types::Color::Black => {
                if rank == Rank::First {
                    new_pos.promote_pawn(*to, types::Piece::QUEEN);
                }
            }
        }
    }

    attackers_to_update |= BitBoard::from_square(*to);
    update_attackers(&mut new_pos, attackers_to_update);

    // If after these updates, the enemy king is in the list of attacked squares, the move gives check
    match !color {
        Color::White => {
            let king_square = 
                panic::catch_unwind(|| (new_pos.piece_bitboards[4] & new_pos.color_bitboards[0]).squares_from_bb()[0]);
            
            match king_square {
                Ok(king_square) => {
                    if new_pos.is_attacked_by_black(king_square) {
                        return true;
                    }
                },
                Err(_) => {
                    panic!("King square not found in would_give_check with move history {:?} and move {:?} {:?}",
                        new_pos.move_history, from, to);
                }
            }
        },
        Color::Black => {
            let king_square = 
                panic::catch_unwind(|| (new_pos.piece_bitboards[4] & new_pos.color_bitboards[1]).squares_from_bb()[0]);
            match king_square {
                Ok(king_square) => {
                    if new_pos.is_attacked_by_white(king_square) {
                        return true;
                    }
                },
                Err(_) => {
                    panic!("King square not found in would_give_check with move history {:?} and move {:?} {:?}",
                        new_pos.move_history, from, to);
                }
            }
        }
    }
    false
}

pub fn is_in_checkmate(pos: &mut Position) -> bool {
    if pos.check {
        let legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
        if legal_moves.is_empty() {
            return true;
        }
    }
    return false;
}

pub fn make_specific_engine_move(pos: &mut Position, from: Square, to: Square) {
    let mut attackers_to_update = BitBoard::empty();

    // Add sliders that are no longer blocked by the moved piece to the list of pieces to update
    attackers_to_update |= get_attacking_sliders(pos, from);

    let is_pawn = pos.piece_bitboards[5].contains(from);
    let is_king = pos.piece_bitboards[4].contains(from);
    pos.make_move(&from, &to);

    // Add sliders that now have their path blocked by the moved piece
    attackers_to_update |= get_attacking_sliders(pos, to);

    // If the move is a castling move, move the rook as well
    if is_king {
        if from == Square::E1 && to == Square::G1 {
            pos.make_move(&Square::H1, &Square::F1);
            attackers_to_update |= BitBoard::from_square(Square::F1);
            attackers_to_update ^= BitBoard::from_square(Square::H1);
        } else if from == Square::E1 && to == Square::C1 {
            pos.make_move(&Square::A1, &Square::D1);
            attackers_to_update |= BitBoard::from_square(Square::D1);
            attackers_to_update ^= BitBoard::from_square(Square::A1);
        } else if from == Square::E8 && to == Square::G8 {
            pos.make_move(&Square::H8, &Square::F8);
            attackers_to_update |= BitBoard::from_square(Square::F8);
            attackers_to_update ^= BitBoard::from_square(Square::H8);
        } else if from == Square::E8 && to == Square::C8 {
            pos.make_move(&Square::A8, &Square::D8);
            attackers_to_update |= BitBoard::from_square(Square::D8);
            attackers_to_update ^= BitBoard::from_square(Square::A8);
        }
    }

    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        let rank = to.rank();
        match !pos.state.active_player {
            types::Color::White => {
                if rank == Rank::Eighth {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                }
            },
            types::Color::Black => {
                if rank == Rank::First {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                }
            }
        }
    }
    
    attackers_to_update |= BitBoard::from_square(to);
    update_attackers(pos, attackers_to_update);

    let king_square = match pos.state.active_player {
        Color::White => (pos.piece_bitboards[4] & pos.color_bitboards[0]).squares_from_bb()[0],
        Color::Black => (pos.piece_bitboards[4] & pos.color_bitboards[1]).squares_from_bb()[0],
    };
    match pos.state.active_player {
        Color::White => pos.check = pos.is_attacked_by_black(king_square),
        Color::Black => pos.check = pos.is_attacked_by_white(king_square),
    }

}