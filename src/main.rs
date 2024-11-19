mod best_move;
mod uiboard;
mod action_manager;
mod piece_square_tables;
use chess::{Board, ChessMove, Color, Piece, Square};
use eframe::egui::{self, FontId, RichText, Color32};
use egui_extras;
use std::ops::RangeInclusive;
use std::time::Duration;
use thousands::Separable;
use action_manager::TurnStates;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(900.0, 650.0)),
        min_window_size: Some(egui::Vec2::new(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "rust-chess-engine",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals {
                window_fill: Color32::from_rgb(92, 84, 112),
                panel_fill: Color32::from_rgb(92, 84, 112), 
                override_text_color: Some(Color32::from_rgb(185, 180, 199)),
                ..Default::default()
            });
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(ChessApp::default())
        }),
    )
}

struct ChessApp {
    board: Board,
    count: i64,
    depth: u32,
    time_elapsed: Duration,
    pruning: bool,
    game_is_over: bool,
    turn_state: TurnStates,
    white_slain_pieces: Vec<Piece>,
    black_slain_pieces: Vec<Piece>,
    source_square: Option<Square>,
    legal_moves_from_source: Vec<ChessMove>,
    winner: Option<Color>
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            board: Board::default(),
            count: 0,
            depth: 3,
            time_elapsed: Duration::ZERO,
            pruning: true,
            game_is_over: false,
            turn_state: TurnStates::PieceSelection,
            white_slain_pieces: Vec::new(),
            black_slain_pieces: Vec::new(),
            source_square: None,
            legal_moves_from_source: Vec::new(),
            winner: None
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut pieces: Vec<egui::Image<'_>> = Vec::new();
        let mut icons: Vec<egui::Image<'_>> = Vec::new();
        
        {   // ugly image importing
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

            icons.push(egui::Image::new(egui::include_image!("assets/dot.png")));
            icons.push(egui::Image::new(egui::include_image!("assets/x.png")));
        }
        
        let image = egui::Image::new(egui::include_image!("assets/chess_board.png"));

        let top_panel_height = 100.0;
        let left_panel_width = 180.0;

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(left_panel_width)
            .frame(egui::containers::Frame {
                fill: Color32::from_rgb(92, 84, 112),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(RichText::new(format!("settings:")).font(FontId::proportional(35.0)));

                ui.label(RichText::new("").font(FontId::proportional(2.0)));

                ui.horizontal(|ui| {
                    ui.label(RichText::new("depth:").font(FontId::proportional(25.0)));
                    ui.add(
                        egui::widgets::DragValue::new(&mut self.depth)
                            .speed(0.05)
                            .clamp_range(RangeInclusive::new(1, 7)),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("alpha-beta:").font(FontId::proportional(25.0)));
                    toggle_ui(ui, &mut self.pruning);
                });

                ui.label(RichText::new("").font(FontId::proportional(5.0)));

                ui.vertical_centered(|ui| {
                    if ui.button("Reset").clicked() {
                        self.board = Board::default();
                        self.reset_info();
                        self.game_is_over = false;
                        self.winner = None;
                        self.turn_state = TurnStates::PieceSelection;
                    };
                });               
            });

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .min_height(top_panel_height)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.vertical_centered( |ui| {
                    ui.label(
                        if !self.game_is_over{
                                RichText::new(format!(
                                    "\n{} nodes searched in {}.{} seconds",
                                    self.count.separate_with_commas(),
                                    self.time_elapsed.as_secs(),
                                    self.time_elapsed.subsec_millis()
                                ))
                            } else if self.game_is_over && self.winner == None{
                                RichText::new(format!(
                                    "\nit's a draw!",
                                ))
                            } else {
                                RichText::new(format!(
                                    "\n{:#?} won!",
                                    self.winner.unwrap(),
                                ))
                            }
                        
                        .font(FontId::proportional(35.0)),
                    );
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let tiles: [[egui::Rect; 8]; 8] = self.display_board(
                ui,
                ctx,
                image,
                top_panel_height
            );

            self.display_pieces(
                ui,
                &pieces,
                &tiles
            );

            self.display_possible_actions(ui,
                &tiles,
                &icons);

            self.action_manager(ui, &tiles);
            
        });
    }
}

// taken from egui widget demos:
fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), "")
    });

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();

        // custom coloring
        let bg_color = if *on {
            Color32::from_rgb(100, 200, 100)
        } else {
            visuals.bg_fill
        };
        let circle_color = if *on {
            Color32::WHITE 
        } else {
            visuals.bg_fill
        };

        ui.painter()
            .rect(rect, radius, bg_color, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, circle_color, visuals.fg_stroke);
    }

    response
}
