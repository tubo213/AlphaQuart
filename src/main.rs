use eframe::egui;

mod game;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Quarto Game",
        options,
        Box::new(|_cc| Ok(Box::new(QuartoApp::new()))),
    )
}

struct QuartoApp {
    game: game::Game,
    selected_piece_index: Option<usize>,
}

impl QuartoApp {
    fn new() -> Self {
        Self {
            game: game::Game::new(),
            selected_piece_index: None,
        }
    }

    fn reset(&mut self) {
        self.game = game::Game::new();
        self.selected_piece_index = None;
    }

    fn draw_piece_ui(
        &self,
        ui: &mut egui::Ui,
        piece: &game::Piece,
        is_selected: bool,
    ) -> egui::Response {
        let size = match piece.height {
            game::Height::Tall => 50.0,  // 高い駒
            game::Height::Short => 30.0, // 低い駒
        };

        let (id, rect) = ui.allocate_space(egui::vec2(size, size)); // 駒のサイズに応じてスペースを割り当て
        let shape_color = match piece.color {
            game::Color::Black => egui::Color32::BLACK,
            game::Color::White => egui::Color32::WHITE,
        };

        let shape = match piece.shape {
            game::Shape::Round => {
                egui::Shape::circle_filled(rect.center(), size / 2.0, shape_color)
            }
            game::Shape::Square => egui::Shape::rect_filled(rect, 5.0, shape_color),
        };

        // 穴のある駒のアウトラインを強調
        let outline = if piece.surface == game::Surface::Hollow {
            egui::Shape::circle_stroke(
                rect.center(),
                size / 2.0 - 2.0,
                egui::Stroke::new(3.0, egui::Color32::from_rgb(200, 200, 200)), // 目立つ色に変更
            )
        } else {
            egui::Shape::Noop
        };

        ui.painter().add(shape);
        ui.painter().add(outline);

        if is_selected {
            // 選択されたピースに枠を追加
            ui.painter()
                .rect_stroke(rect, 5.0, egui::Stroke::new(3.0, egui::Color32::RED));
        }

        ui.interact(rect, id, egui::Sense::click())
    }
}

impl eframe::App for QuartoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Quarto Game");

            if ui.button("Reset Game").clicked() {
                self.reset();
            }

            ui.horizontal(|ui| {
                ui.label("Available Pieces:");
                for (index, piece) in self.game.available_pieces.iter().enumerate() {
                    let is_selected = self.selected_piece_index == Some(index);
                    if self.draw_piece_ui(ui, piece, is_selected).clicked() {
                        self.selected_piece_index = Some(index);
                    }
                }
            });

            ui.separator();

            ui.label(format!("Current Player: {:?}", self.game.current_player));

            // グリッド表示
            egui::Grid::new("game_grid").show(ui, |ui| {
                for row in 0..4 {
                    for col in 0..4 {
                        if let Some(piece) = &self.game.board.grid[row][col] {
                            self.draw_piece_ui(ui, piece, false);
                        } else {
                            if ui.button("[ ]").clicked() {
                                if let Some(piece_index) = self.selected_piece_index {
                                    if let Ok(_) = self.game.play_turn(row, col, piece_index) {
                                        self.selected_piece_index = None;
                                        if self.game.is_game_over() {
                                            ui.label(format!(
                                                "{:?} wins!",
                                                self.game.current_player
                                            ));
                                        } else {
                                            self.game.switch_player();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ui.end_row(); // ここで行の終わりを示す
                }
            });

            if self.game.is_game_over() {
                ui.heading(format!("{:?} wins!", self.game.current_player));
            }
        });
    }
}
