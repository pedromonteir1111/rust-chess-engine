use chess::{Board, ChessMove, Square};
use eframe::egui;
use eframe::egui::{FontId, RichText};
use egui_extras;
use std::ops::RangeInclusive;
use std::time::Duration;
mod best_move;
mod uiboard;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::Vec2::new(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "rust-chess-engine",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals {
                window_fill: egui::Color32::from_rgb(92, 84, 112),
                panel_fill: egui::Color32::from_rgb(92, 84, 112), 
                override_text_color: Some(egui::Color32::from_rgb(185, 180, 199)),
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
    time_elapsed: Duration,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            board: Board::default(),
            count: 0,
            depth: 3,
            time_elapsed: Duration::from_secs(0),
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut pieces: Vec<egui::Image<'_>> = Vec::new();
        
        {   // ugly code
            pieces.push(egui::Image::new(egui::include_image!("assets/tile005.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile011.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile002.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile008.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile003.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile009.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile004.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile010.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile001.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile007.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile000.png")));
            pieces.push(egui::Image::new(egui::include_image!("assets/tile006.png")));
        }
        
        let image = egui::Image::new(egui::include_image!("assets/chess_board.png"));

        let top_panel_height = 100.0;
        let left_panel_width = 150.0;

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(left_panel_width)
            .frame(egui::containers::Frame {
                fill: egui::Color32::from_rgb(92, 84, 112),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(RichText::new(format!("settings:")).font(FontId::proportional(35.0)));
                ui.horizontal(|ui| {
                    ui.label(RichText::new("depth:").font(FontId::proportional(25.0)));
                    ui.add(
                        egui::widgets::DragValue::new(&mut self.depth)
                            .speed(0.05)
                            .clamp_range(RangeInclusive::new(1, 7)),
                    );
                })
            });

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .min_height(top_panel_height)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.vertical_centered( |ui| {
                    ui.label(
                        RichText::new(format!(
                            "\n{} nodes searched in {}.{} seconds",
                            self.count,
                            self.time_elapsed.as_secs(),
                            self.time_elapsed.subsec_millis()
                        ))
                        .font(FontId::proportional(35.0)),
                    );
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let squares = self.display_board(
                ui,
                image,
                top_panel_height
            );

            self.display_pieces(
                ui,
                &pieces,
                &squares
            );
        });
    }
}
