use crate::best_move;
use chess::{Board, Color};
use eframe::egui;
use eframe::egui::{Pos2, Rect, Vec2};
use std::time::Duration;

pub fn display_board(
    ui: &mut egui::Ui,
    board: &Board,
    board_image: egui::Image<'_>,
    piece_images: Vec<egui::Image<'_>>,
    top_panel_height: f32,
    count: &mut i32,
    depth: u32,
    time_elapsed: &mut Duration,
) {
    let panel_size = ui.available_size();

    let board_size = panel_size.y * 0.8;
    let square_size = board_size / 8.0;
    let board_upperleft = Pos2::new(
        ((panel_size.x - board_size) / 2.0) + 161.3,
        ((panel_size.y - board_size) / 2.0) + top_panel_height,
    );
    let board_rect = Rect::from_min_size(board_upperleft, Vec2::new(board_size, board_size));

    let rect = Rect::from_min_size(board_upperleft, Vec2::new(square_size, square_size));

    ui.put(board_rect, board_image);
    ui.put(rect, piece_images[0].clone());

    let response = ui.interact(
        board_rect,
        ui.make_persistent_id("board_rect"),
        egui::Sense::click(),
    );
    let response2 = ui.interact(
        rect,
        ui.make_persistent_id("board_rect"),
        egui::Sense::click(),
    );

    if response.clicked() {
        let (_, best_move) = best_move::best_move(
            true,
            board,
            depth,
            board.side_to_move() == Color::White,
            count,
            time_elapsed,
        );

        match best_move {
            Some(best_move) => println!("{}", best_move),
            None => (),
        }
    }

    if response2.clicked() {
        println!("clicou2");
    }
}
