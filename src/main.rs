pub mod game;

use ply_engine::{
    layout::{Padding, Sizing},
    prelude::*,
};
use strum::IntoEnumIterator;

use crate::game::{
    board::Square,
    pieces::{self, Rotation, PIECE_SHAPES, SHAPES_BY_ROTATION},
    simulator::Simulator,
};

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "blokus-sim".to_owned(),
            window_width: 1200,
            window_height: 800,
            high_dpi: true,
            sample_count: 4,
            platform: miniquad::conf::Platform {
                webgl_version: miniquad::conf::WebGLVersion::WebGL2,
                ..Default::default()
            },
            ..Default::default()
        },
        draw_call_vertex_capacity: 100000,
        draw_call_index_capacity: 100000,
        ..Default::default()
    }
}

pub struct UiState {
    selected_piece: Option<usize>,
    display_special_shapes: bool,
    current_color: pieces::Colors,
    current_rotation: Rotation,
}

#[macroquad::main(window_conf)]
async fn main() {
    static DEFAULT_FONT: FontAsset = font!("assets/fonts/inter.ttf");
    let mut ply = Ply::new(&DEFAULT_FONT).await;

    let mut simulator = Simulator::new();
    let mut ui_state = UiState {
        selected_piece: None,
        display_special_shapes: false,
        current_color: pieces::Colors::Blue,
        current_rotation: Rotation::Normal,
    };

    simulator.place_open_corner_overlay();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::F12) {
            let current = ply.is_debug_mode();
            ply.set_debug_mode(!current);
        }

        let mut ui = ply.begin();

        ui.element()
            .width(grow!())
            .height(grow!())
            .layout(|l| l.direction(LayoutDirection::LeftToRight))
            .children(|ui| {
                draw_pieces(ui, &mut simulator, &mut ui_state);
                draw_controls(ui, &mut simulator, &mut ui_state);

                draw_board(ui, &mut simulator, &mut ui_state);
            });

        ui.show().await;

        next_frame().await;
    }
}

pub fn draw_pieces(ui: &mut Ui, _simulator: &mut Simulator, ui_state: &mut UiState) {
    ui.element()
        .width(fixed!(200.))
        .height(percent!(1.0))
        .overflow(|o| o.scroll_y())
        .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).gap(4))
        .children(|ui| {
            ui.text("Select piece:", |t| t.font_size(24).color(0xFFFFFF));

            for (index, piece) in pieces::Pieces::iter().enumerate() {
                let shape = &PIECE_SHAPES[index];
                let special_shape =
                    &SHAPES_BY_ROTATION.get(&ui_state.current_rotation).unwrap()[index];

                let (width, height) = if ui_state.display_special_shapes {
                    (special_shape.width, special_shape.height)
                } else {
                    (shape.width, shape.height)
                };

                ui.element()
                    .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).padding(1))
                    .border(|b| {
                        if ui_state
                            .selected_piece
                            .is_some_and(|selected| index == selected)
                        {
                            b.all(4).color(0xFFFFFF)
                        } else {
                            b
                        }
                    })
                    .corner_radius(4.)
                    .children(|ui| {
                        if ui.just_pressed() {
                            if ui_state.selected_piece == Some(index) {
                                ui_state.selected_piece = None;
                            } else {
                                ui_state.selected_piece = Some(index);
                            }
                        }

                        ui.text(&piece.to_string(), |t| t.font_size(16).color(0xFFFFFF));

                        let width_to_height = width as f32 / height as f32;

                        let size_mult = if 80. * width_to_height > 160. {
                            0.25
                        } else {
                            1.
                        };

                        ui.element()
                            .width(fixed!(80. * width_to_height * size_mult))
                            .height(fixed!(80. * size_mult))
                            .children(|ui| {
                                draw_grid(ui, width as usize, height as usize, |x, y, ui| {
                                    // dbg!(piece.to_string(), x, y);

                                    let color = if ui_state.display_special_shapes {
                                        match special_shape.value_at_pos(x as u8, y as u8) {
                                            pieces::SpecialShapePart::Blank______ => 0x222222,
                                            pieces::SpecialShapePart::Normal_____ => 0xFFFFFF,
                                            pieces::SpecialShapePart::InnerCorner => 0x00FF00,
                                            pieces::SpecialShapePart::OuterCorner => 0xFF0000,
                                        }
                                    } else {
                                        if shape.occupied_at_pos(x as u8, y as u8) {
                                            ui_state.current_color.to_rgb()
                                        } else {
                                            0x222222
                                        }
                                    };

                                    ui.element()
                                        .width(grow!())
                                        .height(grow!())
                                        .background_color(color)
                                        .empty();
                                });
                            });
                    });
            }
        });
}

pub fn draw_controls(ui: &mut Ui, _simulator: &mut Simulator, ui_state: &mut UiState) {
    ui.element()
        .width(fixed!(150.))
        .height(percent!(1.0))
        .overflow(|o| o.scroll_y())
        .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).gap(16))
        .children(|ui| {
            button(ui, fixed!(100.), fixed!(50.), |ui| {
                ui.text("Change Color", |t| {
                    t.font_size(16).color(0xEEEEEEE).alignment(CenterX)
                });
                if ui.just_pressed() {
                    let next_color = match ui_state.current_color {
                        pieces::Colors::Blue => pieces::Colors::Green,
                        pieces::Colors::Green => pieces::Colors::Red,
                        pieces::Colors::Red => pieces::Colors::Yellow,
                        pieces::Colors::Yellow => pieces::Colors::Blue,
                    };

                    ui_state.current_color = next_color;
                }
            });
            button(ui, fixed!(100.), fixed!(50.), |ui| {
                ui.text("Special shape display", |t| {
                    t.font_size(16).color(0xEEEEEEE).alignment(CenterX)
                });
                if ui.just_pressed() {
                    ui_state.display_special_shapes = !ui_state.display_special_shapes;
                }
            });
            button(ui, fixed!(100.), fixed!(50.), |ui| {
                ui.text("Rotate right", |t| {
                    t.font_size(16).color(0xEEEEEEE).alignment(CenterX)
                });
                if ui.just_pressed() {
                    ui_state.current_rotation = ui_state.current_rotation.rotated_right();
                }
            });
            button(ui, fixed!(100.), fixed!(50.), |ui| {
                ui.text("Mirror vertically", |t| {
                    t.font_size(16).color(0xEEEEEEE).alignment(CenterX)
                });
                if ui.just_pressed() {
                    ui_state.current_rotation = ui_state.current_rotation.mirrored_vertically();
                }
            });
            button(ui, fixed!(100.), fixed!(50.), |ui| {
                ui.text("Step Simulator", |t| {
                    t.font_size(16).color(0xEEEEEEE).alignment(CenterX)
                });
                if ui.just_pressed() {
                    _simulator.step();
                }
            });
        });
}

pub fn draw_board(ui: &mut Ui, simulator: &mut Simulator, ui_state: &mut UiState) {
    ui.element()
        .id("board container")
        .height(grow!())
        .width(ratio!(1.0))
        .contain(1.0)
        .overflow(|o| o.scroll_y())
        .layout(|l| l.padding(Padding::all(30)))
        .border(|b| {
            b.all(50)
                .position(BorderPosition::Inside)
                .color((0, 0, 0, 0))
        })
        .children(|ui| {
            draw_grid(ui, 20, 20, |x, y, ui| {
                let square = simulator.board().get_squares()[x][y].clone();

                let color = match square {
                    Square::Empty => match simulator.overlay_board_mut().get_squares()[x][y] {
                        Square::Empty => 0xFFFFFF,
                        Square::Blue => 0x7777FF,
                        Square::Green => 0x77FF77,
                        Square::Red => 0xFF7777,
                        Square::Yellow => 0xFFFF77,
                    },
                    Square::Blue => 0x0000FF,
                    Square::Green => 0x00FF00,
                    Square::Red => 0xFF0000,
                    Square::Yellow => 0xFFFF00,
                };

                if ui.just_pressed() {
                    if let Some(piece) = ui_state.selected_piece {
                        simulator.board_mut().place_piece(
                            &PIECE_SHAPES[piece],
                            &ui_state.current_color,
                            x,
                            y,
                        );
                    }
                }

                let overlay = simulator.overlay_board_mut();
                if ui.hovered() {
                    if let Some(piece) = ui_state.selected_piece {
                        overlay.reset();
                        overlay.place_piece(&PIECE_SHAPES[piece], &ui_state.current_color, x, y);
                        simulator.place_open_corner_overlay();
                    }
                }

                ui.element()
                    .width(grow!())
                    .height(grow!())
                    .border(|b| b.all(2).color(0x666666).position(Outside))
                    .background_color(color)
                    .empty();
            });
        });
}

fn draw_grid(
    ui: &mut Ui,
    width: usize,
    height: usize,
    mut square_cb: impl FnMut(usize, usize, &mut Ui),
) {
    ui.element()
        // .background_color(0x222222)
        .width(grow!())
        .height(grow!())
        .layout(|l| l.direction(LayoutDirection::TopToBottom).gap(1))
        .children(|ui| {
            for y in 0..height {
                ui.element()
                    .height(Sizing::Percent(1. / height as f32))
                    .width(grow!())
                    // .background_color(0xCCCCCC)
                    .layout(|l| l.direction(LayoutDirection::LeftToRight).gap(1))
                    .children(|ui| {
                        for x in 0..width {
                            ui.element()
                                .width(Sizing::Percent(1. / width as f32))
                                .height(grow!())
                                .children(|ui| {
                                    square_cb(x, y, ui);
                                });
                        }
                    });
            }
        });
}

fn button(ui: &mut Ui, width: Sizing, height: Sizing, mut children: impl FnMut(&mut Ui)) {
    ui.element()
        .width(width)
        .height(height)
        .background_color(0x333334)
        .layout(|l| l.align(CenterX, CenterY))
        .border(|b| b.all(2).color(0x424242))
        .corner_radius(4.)
        .children(|ui| children(ui));
}
