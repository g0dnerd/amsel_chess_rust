use std::panic;

use types::{
    position::Position,
    bitboard::BitBoard,
    state::GameResult,
    types_utils::*,
    Color,
    Results,
};
use crate::{
    negamax,
    movegen,
    evaluation,
    parse_input,
};

pub fn main_game_loop(humans: u8, depth: u8, fen: Option<String>) -> Vec<(u8, u8)> {
    let mut pos = if fen.is_some() {
        let mut position = Position::from_fen(fen.unwrap());
        update_attackers(&mut position, !BitBoard::empty());
        position
    } else {
        Position::new()
    };
    match humans {
        0 => {
            println!("AI vs AI game.");
            while pos.state.game_result.is_ongoing() {
                pos.print_position();
                
                let eval = match pos.state.active_player {
                    Color::White => evaluation::main_evaluation(&mut pos),
                    Color::Black => -evaluation::main_evaluation(&mut pos),
                };
                if i32::MIN + 1 < eval && eval < i32::MAX {
                    println!("Current evaluation: {}", eval);   
                }

                if is_in_checkmate(&mut pos) {
                    pos.state.game_result = match pos.state.active_player {
                        types::Color::White => GameResult(Results::BLACK_VICTORY),
                        types::Color::Black => GameResult(Results::WHITE_VICTORY),
                    };
                    println!("{:?} wins by checkmate!", !pos.state.active_player);
                    println!("FEN: {}", fen_from_pos(&pos));
                    return pos.move_history;
                }
                make_engine_move(&mut pos, depth);
                if pos.state.active_player == Color::Black { pos.state.full_move_counter += 1; }
            }
            println!("FEN: {}", fen_from_pos(&pos));
            return pos.move_history;
        },
        1 => {
            println!("Human vs AI game.");
            while pos.state.game_result.is_ongoing() {
                pos.print_position();

                let eval = match pos.state.active_player {
                    Color::White => evaluation::main_evaluation(&mut pos),
                    Color::Black => -evaluation::main_evaluation(&mut pos),
                };
                if i32::MIN + 1 < eval && eval < i32::MAX {
                    println!("Current evaluation: {}", eval);   
                }
                if is_in_checkmate(&mut pos) {
                    pos.state.game_result = match pos.state.active_player {
                        types::Color::White => GameResult(Results::BLACK_VICTORY),
                        types::Color::Black => GameResult(Results::WHITE_VICTORY),
                    };
                    println!("{:?} wins by checkmate!", !pos.state.active_player);
                    println!("FEN: {}", fen_from_pos(&pos));
                    return pos.move_history;
                }
                // Get user input in the format of "a1 a2"
                let mut input = String::new();
                println!("Enter a legal move, type 'legal' to get a list of legal moves or press enter to have the engine move.");
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();let input_legality = parse_input::user_input_to_square_index(input);
                match input_legality {
                    Ok(o) => {
                        if o == [99, 99] {
                            continue;
                        } else if o == [98, 98] {
                            let moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, &mut pos);
                            println!("Legal moves:");
                            for legal_move in moves {
                                let from_string = string_from_square(legal_move.0);
                                let to_string = string_from_square(legal_move.1);
                                print!("{} {}, ", from_string, to_string);
                            }
                            continue;
                        }
                        else if o == [97, 97] {
                            make_engine_move(&mut pos, depth);
                            if pos.state.active_player == Color::Black { pos.state.full_move_counter += 1; }
                            continue;
                        } else if o == [96, 96] {
                            println!("FEN: {}", fen_from_pos(&pos));
                            continue;
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                }

                let squares = input_legality.unwrap();
                let square = squares[0];
                let target_square = squares[1];
                
                match make_player_move(&mut pos, square, target_square) {
                    Ok(_) => 
                        if pos.state.active_player == Color::Black { pos.state.full_move_counter += 1; },
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                }
            }
            println!("FEN: {}", fen_from_pos(&pos));
            return pos.move_history;
        },
        2 => {
            println!("Running benchmark mode for 2 moves.");
            for _i in 0..2 {
                pos.print_position();
                
                let eval = match pos.state.active_player {
                    Color::White => evaluation::main_evaluation(&mut pos),
                    Color::Black => -evaluation::main_evaluation(&mut pos),
                };
                if i32::MIN + 1 < eval && eval < i32::MAX {
                    println!("Current evaluation: {}", eval);   
                }

                if is_in_checkmate(&mut pos) {
                    pos.state.game_result = match pos.state.active_player {
                        types::Color::White => GameResult(Results::BLACK_VICTORY),
                        types::Color::Black => GameResult(Results::WHITE_VICTORY),
                    };
                    println!("{:?} wins by checkmate!", !pos.state.active_player);
                    println!("FEN: {}", fen_from_pos(&pos));
                    return pos.move_history;
                }
                make_engine_move(&mut pos, depth);
            }
            println!("FEN: {}", fen_from_pos(&pos));
            return pos.move_history;
        }
        _ => panic!("Invalid number of human players."),
    }
}

/* Find all sliders that are attacking the given square by using a fictitious queen that can move in all directions,
getting all possible moves for that piece and then filtering out the sliders from the resulting bitboard. */
pub fn get_attacking_sliders(pos: &mut Position, from: u8) -> BitBoard {
    // TODO: This returns too many sliders - assumes every slider can move in all directions
    let mut attacking_sliders = movegen::pseudolegal_slider_moves(3, from, pos);
    attacking_sliders &= pos.piece_bitboards[0] | pos.piece_bitboards[2] | pos.piece_bitboards[3];
    attacking_sliders
}

pub fn update_attackers(pos: &mut Position, attackers: BitBoard) {
    let mut attacker_board = attackers & (pos.color_bitboards[0] | pos.color_bitboards[1]);

    while attacker_board != BitBoard::empty() {
        let index = attacker_board.trailing_zeros() as u8;
        if let Some(piece) = pos.piece_type_at(index) {
            let attacks = match piece {
                0 | 2 | 3 => {
                    movegen::pseudolegal_slider_moves(piece, index, pos)
                },
                1 => {
                    movegen::get_pseudolegal_knight_moves(index)
                },
                4 => {
                    movegen::get_king_moves(index, pos)
                },
                5 => {
                    movegen::pawn_attacks(index, pos.piece_color(index) as usize)
                },
                _ => panic!("Invalid piece type found in update_attackers.")
            };
            pos.update_attack_maps(index, attacks);
        } else {
            unreachable!("Trying to update attackers on empty square {:?}.",
            index);
        }
        attacker_board.clear_lsb();
    }
    
}

pub fn would_give_check(pos: &mut Position, from: u8, to: u8) -> bool {
    let mut new_pos = pos.clone();
    apply_move(&mut new_pos, from, to);
    new_pos.check
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

pub fn make_player_move(pos: &mut Position, from: u8, to: u8) -> Result<(), &'static str> {
    // Check if the targetted piece contains a piece of the active player's color
    if let Some(piece) = pos.piece_at(from) {
        if piece.1 != pos.state.active_player {
            return Err("Piece does not belong to active player.");
        }
    } else {
        return Err("Illegal move: no piece on origin square.");
    }

    let legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);

    // Check if the move is legal by checking if it is in the list of legal moves
    if !legal_moves.contains(&(from, to)) {
        return Err("Not a legal move.");
    }

    apply_move(pos, from, to);
    pos.move_history.push((from, to));

    Ok(())
}

pub fn make_engine_move(pos: &mut Position, depth: u8) {  
    let best_move = negamax::find_best_move(pos, depth);
    let (from, to) = best_move;

    println!("AI move: {} {}", string_from_square(from), string_from_square(to));

    apply_move(pos, from, to);
    pos.move_history.push((from, to));
}

pub fn apply_move(pos: &mut Position, from: u8, to: u8) {
    let mut attackers_to_update = BitBoard::empty();

    // Add sliders that are no longer blocked by the moved piece to the list of pieces to update
    attackers_to_update |= get_attacking_sliders(pos, from);

    let is_pawn = pos.piece_bitboards[5].contains(from);
    let is_king = pos.piece_bitboards[4].contains(from);

    let ep_square: Option<u8> = pos.en_passant_square;

    pos.make_move(&from, &to);

    // Add sliders that now have their path blocked by the moved piece
    attackers_to_update |= get_attacking_sliders(pos, to);

    // If the move is a castling move, move the rook as well
    if is_king && ((from as i8 % 8) - (to as i8 % 8)).abs() > 1 {
        if from > to {
            pos.make_castling_move(&(to - 2), &(from - 1));
            attackers_to_update |= BitBoard::from_square(from - 1);
            attackers_to_update ^= BitBoard::from_square(to - 1);
        } else {
            pos.make_castling_move(&(to + 1), &(from + 1));
            attackers_to_update |= BitBoard::from_square(from + 1);
            attackers_to_update ^= BitBoard::from_square(to + 1);
        }
    }

    // Check for promotion, auto-promote to queen for now. Update slider blockers and attacks for the new queen
    if is_pawn {
        match !pos.state.active_player {
            types::Color::White => {
                if to / 8 == 7 {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                }
            },
            types::Color::Black => {
                if to / 8 == 0 {
                    pos.promote_pawn(to, types::Piece::QUEEN);
                }
            }
        }
        // Check if the move is en passant
        match ep_square {
            Some(ep_square) => {
                if to == ep_square {
                    let ep_target = match pos.state.active_player {
                        Color::White => to + 8,
                        Color::Black => to - 8,
                    };
                    attackers_to_update |= get_attacking_sliders(pos, ep_target);
                    pos.color_bitboards[pos.state.active_player as usize] ^= BitBoard::from_square(ep_target);
                    pos.piece_bitboards[5] ^= BitBoard::from_square(ep_target);
                }
            },
            None => (),
        }
    }
    
    attackers_to_update |= BitBoard::from_square(to);
    update_attackers(pos, attackers_to_update);

    let king_square = match pos.state.active_player {
        Color::White => pos.piece_bitboards[4] & pos.color_bitboards[0],
        Color::Black => pos.piece_bitboards[4] & pos.color_bitboards[1],
    };

    match king_square {
        BitBoard(0) => {
            pos.print_position();
            panic!("No king found for active player {:?} after move {:?} -> {:?}", pos.state.active_player, from, to);
        },
        _ => 
            // Check if the move puts the enemy king in check
            pos.check = pos.is_square_attacked_by_color(king_square.trailing_zeros() as u8, !pos.state.active_player),
    }
}

pub fn is_quiet_position(pos: &mut Position) -> bool {
    let legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    for (_from, to) in legal_moves {
        if pos.is_capture(&to) {
            return false;
        }
    }
    true
}