use chess::Board;
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
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::BottomUp),
                    |ui| {
                        ui.label(
                            RichText::new(format!(
                                "{} nodes searched in {}.{} seconds",
                                self.count,
                                self.time_elapsed.as_secs(),
                                self.time_elapsed.subsec_millis()
                            ))
                            .font(FontId::proportional(35.0)),
                        );
                    },
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut pieces: Vec<egui::Image<'_>> = Vec::new();
            pieces.push(egui::Image::new(egui::include_image!("assets/147065.svg")));
            let image = egui::Image::new(egui::include_image!("assets/chess_board.png"));

            uiboard::display_board(
                ui,
                &self.board,
                image,
                pieces,
                top_panel_height,
                &mut self.count,
                self.depth,
                &mut self.time_elapsed,
            );
        });
    }
}
