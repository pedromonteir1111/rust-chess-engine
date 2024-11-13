use chess::{Board, ChessMove, Color, Square};
use eframe::egui;
use egui_extras;
mod best_move;
mod uiboard;

fn main () -> Result<(), eframe::Error>{

    let options = eframe::NativeOptions::default();
    
    let mut board = Board::default();

    //teste basico do algoritmo

    board = board.make_move_new(ChessMove::new(Square::D2, Square::D4, None));
    println!("{}", board.combined());
    board = board.make_move_new(ChessMove::new(Square::B8, Square::C6, None));
    println!("{}", board.combined());
    board = board.make_move_new(ChessMove::new(Square::A2, Square::A4, None));
    println!("{}", board.combined());


    let (_, best_move) = best_move::minimax(&board, 3, i32::MIN, i32::MAX, board.side_to_move() == Color::White);

    match best_move {
        Some(best_move) => println!("{}", best_move),
        None => ()
    }

    eframe::run_native(
        "My eFrame App",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals {
                window_fill: egui::Color32::from_rgb(67, 91, 102),
                panel_fill: egui::Color32::from_rgb(67, 91, 102),
                ..Default::default()
            });
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(ChessApp::default())}),
    )
    
}

struct ChessApp {
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                uiboard::display_board(ui)
            });
        });
    }
}
