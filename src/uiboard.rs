use crate::{action_manager::TurnStates, best_move};
use super::ChessApp;
use chess::{BitBoard, Color, Piece};
use eframe::egui::{self, Pos2, Rect, Vec2, Color32};

enum PiecesAndColors {
    WhitePawn,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackQueen,
    BlackKing
}

// bloco de implementacao para mostrar o tabuleiro

impl ChessApp {
    pub fn display_board (
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        board_image: egui::Image<'_>,
        top_panel_height: f32,
    ) -> [[Rect; 8]; 8]  {
        let panel_size = ui.available_size();
    
        let board_size = panel_size.y * 0.8;
        let square_size = board_size / 8.0;
        let board_upperleft = Pos2::new(
            ((panel_size.x - board_size) / 2.0) + 171.3,
            ((panel_size.y - board_size) / 2.0) + top_panel_height,
        );
        let board_rect = Rect::from_min_size(board_upperleft, Vec2::new(board_size, board_size));
    
        let mut tiles: [[Rect ; 8] ; 8] = [[Rect::NOTHING ; 8] ; 8];
        
        for row in 0..8 {
            for col in 0..8 {
                tiles[row][col] = Rect::from_min_size(
                    Pos2::new(
                        board_upperleft.x + square_size * col as f32,
                        board_upperleft.y + square_size * row as f32),
                    Vec2::new(square_size, square_size));
            }
        }        

        ui.put(board_rect, board_image);

        self.draw_evaluation_bar(ctx, ui, Pos2::new(board_upperleft.x + board_size + 5.0, board_upperleft.y), Vec2::new(30.0, board_size));

        tiles
    }

    pub fn display_pieces(
        &self, 
        ui: &mut egui::Ui,
        piece_images: &Vec<egui::Image<'_>>,
        tiles: &[[Rect; 8]; 8]) {
        
        let bitboards: [(BitBoard, PiecesAndColors) ; 12] = [
            (self.board.pieces(Piece::Pawn).clone() & self.board.color_combined(Color::White), PiecesAndColors::WhitePawn),
            (self.board.pieces(Piece::Bishop).clone() & self.board.color_combined(Color::White), PiecesAndColors::WhiteBishop),
            (self.board.pieces(Piece::Knight).clone() & self.board.color_combined(Color::White), PiecesAndColors::WhiteKnight),
            (self.board.pieces(Piece::Rook).clone() & self.board.color_combined(Color::White), PiecesAndColors::WhiteRook),
            (self.board.pieces(Piece::Queen).clone() & self.board.color_combined(Color::White), PiecesAndColors::WhiteQueen),
            (self.board.pieces(Piece::King).clone() & self.board.color_combined(Color::White), PiecesAndColors::WhiteKing),
            (self.board.pieces(Piece::Pawn).clone() & self.board.color_combined(Color::Black), PiecesAndColors::BlackPawn),
            (self.board.pieces(Piece::Bishop).clone() & self.board.color_combined(Color::Black), PiecesAndColors::BlackBishop),
            (self.board.pieces(Piece::Knight).clone() & self.board.color_combined(Color::Black), PiecesAndColors::BlackKnight),
            (self.board.pieces(Piece::Rook).clone() & self.board.color_combined(Color::Black), PiecesAndColors::BlackRook),
            (self.board.pieces(Piece::Queen).clone() & self.board.color_combined(Color::Black), PiecesAndColors::BlackQueen),
            (self.board.pieces(Piece::King).clone() & self.board.color_combined(Color::Black), PiecesAndColors::BlackKing),
        ];

        for (bitboard, bb_type) in bitboards {
            for rank in (0..8).rev() {
                for file in 0..8 {
                    let square_index = (7 - rank) * 8 + file;
                    let bit = (bitboard.0 >> square_index) & 1;
                    
                    if bit == 1 {
                        ui.put(tiles[rank][file], piece_images[piece_to_index(&bb_type)].clone());
                    } 
                }
            }           
        }
    }

    pub fn draw_evaluation_bar(&self, ctx: &egui::Context, ui: &mut egui::Ui, position: Pos2, size: Vec2) {
       
        let evaluation = best_move::evaluate_board(&self.board);

        let max_eval = 3000;
        let min_eval = -3000;
        let eval_clamped = if !matches!(self.turn_state, TurnStates::Checkmate) {evaluation.clamp(min_eval, max_eval) as f32} else {max_eval as f32};
        let eval_percent = (eval_clamped - min_eval as f32) / (max_eval - min_eval) as f32;
        let rect = Rect::from_min_size(position, size);
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("evaluation_bar")));

        let mid_y = rect.bottom() - (rect.height() * eval_percent as f32);
    
        painter.rect_filled(rect, 0.0, Color32::BLACK);
        painter.rect_filled(Rect::from_min_max(Pos2::new(rect.left(), mid_y), rect.max), 0.0, Color32::WHITE);
    
        let response = ui.interact(rect, egui::Id::new("evaluation_bar_interaction"), egui::Sense::hover());

    
        if response.hovered() {
            egui::show_tooltip(ctx, response.id, |ui| {
                ui.label(format!("Evaluation: {:.2}", evaluation));
            });
        }
    }

    pub fn display_possible_actions(&mut self, ui: &mut egui::Ui, tiles: &[[Rect; 8]; 8], icons: &Vec<egui::Image<'_>>) {
        if !self.legal_moves_from_source.is_empty() {
            for mv in self.legal_moves_from_source.clone() {
                let dest_square = mv.get_dest();
                let (row, col) = square_to_row_col(dest_square);
                
                let mut index: usize = 0;

                if self.board.piece_on(dest_square).is_some() {
                    index = 1;
                }

                ui.put(tiles[row][col], icons[index].clone());
            }
        }
    }

}

pub fn detect_clicked_square(ui: &mut egui::Ui, tiles: &[[Rect; 8]; 8]) -> Option<(usize, usize)> {

    let mouse_pos = match ui.input(|i| i.pointer.interact_pos()) {
        Some(pos) => pos,
        None => Pos2::ZERO
    };

    if ui.input(|i| i.pointer.primary_pressed()) {
        for row in 0..8 {
            for col in 0..8 {
                let rect = tiles[row][col];
                
                if rect.contains(mouse_pos) {
                    return Some((7 - row, col));
                }
            }
        }
    }
    None
}

fn piece_to_index(bb_type: &PiecesAndColors) -> usize {
    match bb_type {
        PiecesAndColors::WhitePawn => 0,
        PiecesAndColors::BlackPawn => 1,
        PiecesAndColors::WhiteBishop => 2,
        PiecesAndColors::BlackBishop => 3,
        PiecesAndColors::WhiteKnight => 4,
        PiecesAndColors::BlackKnight => 5,
        PiecesAndColors::WhiteRook => 6,
        PiecesAndColors::BlackRook => 7,
        PiecesAndColors::WhiteQueen => 8,
        PiecesAndColors::BlackQueen => 9,
        PiecesAndColors::WhiteKing => 10,
        PiecesAndColors::BlackKing => 11,
    }
}

fn square_to_row_col(square: chess::Square) -> (usize, usize) {
    let row = square.get_rank().to_index();
    let col = square.get_file().to_index();
    (7 - row, col)
}

