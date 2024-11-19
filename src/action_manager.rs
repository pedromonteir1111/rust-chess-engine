use std::time::Duration;

use chess::{BoardStatus, ChessMove, Color, File, MoveGen, Piece, Rank, Square};
use eframe::egui;
use super::ChessApp;
use crate::uiboard::detect_clicked_square;
use crate::best_move::best_move;

pub enum TurnStates {
    PieceSelection,
    SquareSelection,
    OpponentMoves,
    Checkmate,
    Stalemate
}

// implement functions that handle player action
impl ChessApp {
    pub fn action_manager(&mut self, ui: &mut egui::Ui, squares: &[[egui::Rect; 8]; 8]){
        match self.turn_state {
            TurnStates::PieceSelection => self.select_piece(ui, squares),
            TurnStates::SquareSelection => self.select_square(ui, squares),
            TurnStates::OpponentMoves => self.move_opponent(),
            TurnStates::Checkmate => self.game_is_over = true,
            TurnStates::Stalemate => self.game_is_over = true,
        }
    }

    fn select_piece(&mut self, ui: &mut egui::Ui, squares: &[[egui::Rect; 8]; 8]) {
        
        if self.board.status() == BoardStatus::Ongoing{
            match detect_clicked_square(ui, squares) {
                Some((row, col)) => {
                    let clicked_square = row_col_to_square(row, col);

                    if self.board.piece_on(clicked_square).is_some() {
                        self.source_square = Some(clicked_square);

                        let legal_moves = MoveGen::new_legal(&self.board);
            
                        self.legal_moves_from_source = legal_moves
                            .filter(|m| m.get_source() == clicked_square)
                            .collect();

                        if !self.legal_moves_from_source.is_empty() {
                            self.turn_state = TurnStates::SquareSelection;
                        }    

                    } else {
                        self.legal_moves_from_source = Vec::new();
                    }
                },
                None => ()
            }
        } else if self.board.status() == BoardStatus::Checkmate {
            self.checkmate();
        } else {
            self.turn_state = TurnStates::Stalemate;
    }
        
    }

    fn select_square(&mut self, ui: &mut egui::Ui, squares: &[[egui::Rect; 8]; 8]) {
        
        if self.board.status() == BoardStatus::Ongoing{
            match detect_clicked_square(ui, squares) {
                Some((row, col)) => {
                    let clicked_square = row_col_to_square(row, col);
    
                    let mut possible_move = ChessMove::new(self.source_square.unwrap(), clicked_square, None);
    
                    let piece = self.board.piece_on(self.source_square.unwrap());
                    if let Some(piece) = piece {
                        if piece == Piece::Pawn {
                            let promotion_rank = if self.board.side_to_move() == Color::White {
                                Rank::Eighth
                            } else {
                                Rank::First
                            };

                            if clicked_square.get_rank() == promotion_rank {
                                possible_move = ChessMove::new(self.source_square.unwrap(), clicked_square, Some(Piece::Queen));
                            }
                        }
                    }

                    if self.legal_moves_from_source.contains(&possible_move) {
                        
                        if self.board.piece_on(clicked_square).is_some(){
                            self.black_slain_pieces.push(self.board.piece_on(clicked_square).unwrap());
                        } 

                        self.board = self.board.make_move_new(possible_move);
                        self.turn_state = TurnStates::OpponentMoves;
                    } else {
                        if self.board.piece_on(clicked_square).is_some() {
                            self.source_square = Some(clicked_square);
        
                            let legal_moves = MoveGen::new_legal(&self.board);
                
                            self.legal_moves_from_source = legal_moves
                                .filter(|m| m.get_source() == clicked_square)
                                .collect();
                        } else {
                            self.turn_state = TurnStates::PieceSelection;
                            self.source_square = None;
                            self.legal_moves_from_source = Vec::new();
                        }
                    }
                },
                None => ()
            }  
        } else if self.board.status() == BoardStatus::Checkmate {
            self.checkmate();
        } else {
            self.turn_state = TurnStates::Stalemate;
        }
        
    }

    fn move_opponent(&mut self) {
        
        if self.board.status() == BoardStatus::Ongoing{
            let (_, best_move) = best_move(
                &self.pruning,
                &self.board,
                self.depth,
                self.board.side_to_move() == Color::White,
                &mut self.count,
                &mut self.time_elapsed,
            );
    
            match best_move {
                Some(best_move) => {
                    
                    if self.board.piece_on(best_move.get_dest()).is_some() {
                        self.white_slain_pieces.push(self.board.piece_on(best_move.get_dest()).unwrap());
                    }
                    
                    self.board = self.board.make_move_new(best_move);   
                },
                None => (),
            }
    
            self.turn_state = TurnStates::PieceSelection;
            self.source_square = None;
            self.legal_moves_from_source = Vec::new();

        } else if self.board.status() == BoardStatus::Checkmate {
            self.checkmate();
        } else {
            self.turn_state = TurnStates::Stalemate;
        }
     
    }

    fn checkmate(&mut self) {
        self.turn_state = TurnStates::Checkmate;

        let checkmated_player = self.board.side_to_move();
        self.winner = Some(
            if checkmated_player == Color::White {
                Color::Black
            } else {
                Color::White
            }
        );

        self.reset_info();
    }

    pub fn reset_info(&mut self) {
        self.time_elapsed = Duration::ZERO;
        self.count = 0;
        self.source_square = None;
        self.legal_moves_from_source = Vec::new();
        self.white_slain_pieces = Vec::new();
        self.black_slain_pieces = Vec::new();

    }
}

fn row_col_to_square(row: usize, col: usize) -> Square {
        
    let rank = Rank::from_index(row);
    let file = File::from_index(col); 
    Square::make_square(rank, file)
}