use eframe::egui;

pub fn display_board(ui: &mut egui::Ui) {

    let panel_size = ui.available_size();

    let light_square_color = egui::Color32::from_rgb(234, 178, 160);
    let dark_square_color = egui::Color32::from_rgb(167, 111, 111);
    let board_size = panel_size.y * 0.9;
    let square_size = board_size / 8.0;

    //let mut rects: [[egui::Rect; 8]; 8];
    
    egui::Grid::new("chess_board")
        .spacing([0.0, 0.0])
        .show(ui, |ui| {
            for row in 0..8 {
                for col in 0..8 {
                    
                    let is_light_square = (row + col) % 2 == 0;
                    let square_color = if is_light_square { light_square_color } else { dark_square_color };

                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(square_size, square_size),
                        egui::Sense::click(),
                    );
                    ui.painter().rect_filled(rect, 0.0, square_color);
                }
                ui.end_row();
            }
        });
}