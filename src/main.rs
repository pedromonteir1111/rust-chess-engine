use std::ops::RangeInclusive;

use chess::Board;
use eframe::egui;
use eframe::egui::{RichText, FontId};
use egui_extras;
mod best_move;
mod uiboard;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "rust-chess-engine",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals {
                window_fill: egui::Color32::from_rgb(67, 91, 102),
                panel_fill: egui::Color32::from_rgb(67, 91, 102),
                ..Default::default()
            });
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(ChessApp::default())
        }),
    )
}

struct ChessApp {
    board: Board,
    count: i32,
    depth: u32,
    has_searched: bool
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            board: Board::default(),
            count: 0,
            depth: 3,
            has_searched: false
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        let top_panel_height = 75.0;

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .min_height(top_panel_height)
            .show_separator_line(false)
            .show(ctx, |ui| {   
            
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(format!("{} nodes searched", self.count)).font(FontId::proportional(30.0)));
                ui.label(RichText::new("depth:").font(FontId::proportional(25.0)));
                ui.add(egui::widgets::DragValue::new(&mut self.depth).speed(0.05).clamp_range(RangeInclusive::new(1, 7)));
                 
            })                    
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut pieces: Vec<egui::Image<'_>> = Vec::new();
            pieces.push(egui::Image::new(egui::include_image!("assets/147065.svg")));
            let image = egui::Image::new(egui::include_image!("assets/phprnyp9x.png"));

            uiboard::display_board(ui, &self.board, image, pieces, top_panel_height, &mut self.count, &mut self.has_searched, self.depth);
                  
        });
    }
}
