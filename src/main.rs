pub mod game;

use ply_engine::{
    layout::{Padding, Sizing},
    prelude::*,
};
use strum::IntoEnumIterator;

use crate::game::{board::Square, pieces, simulator::Simulator};

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

#[macroquad::main(window_conf)]
async fn main() {
    static DEFAULT_FONT: FontAsset = font!("assets/fonts/inter.ttf");
    let mut ply = Ply::new(&DEFAULT_FONT).await;

    let mut simulator = Simulator::new();

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
                render_pieces(ui, &mut simulator);

                render_board(ui, &mut simulator);
            });

        ui.show().await;

        next_frame().await;
    }
}

pub fn render_pieces(ui: &mut Ui, _simulator: &mut Simulator) {
    ui.element()
        .width(fixed!(380.))
        .height(percent!(1.0))
        .overflow(|o| o.scroll_y())
        .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).gap(4))
        .children(|ui| {
            ui.text("Blokus", |t| t.font_size(32).color(0xFFFFFF));

            for piece in pieces::Pieces::iter() {
                let shape = pieces::get_occupation(&piece);

                ui.text(&piece.to_string(), |t| t.font_size(16).color(0xFFFFFF));

                let width_to_height = shape.width as f32 / shape.height as f32;

                ui.element()
                    .width(fixed!(80. * width_to_height))
                    .height(fixed!(80.))
                    .children(|ui| {
                        draw_grid(
                            ui,
                            shape.width as usize,
                            shape.height as usize,
                            |x, y, ui| {
                                // dbg!(piece.to_string(), x, y);
                                let index = y + x * shape.width as usize;
                                if index >= shape.map.len() {
                                    return;
                                }

                                let color = if shape.map[index] { 0xFFFFFF } else { 0x222222 };

                                ui.element()
                                    .width(grow!())
                                    .height(grow!())
                                    .background_color(color)
                                    .empty();
                            },
                        );
                    });
            }
        });
}

pub fn render_board(ui: &mut Ui, simulator: &mut Simulator) {
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
            let board = simulator.board_mut();
            draw_grid(ui, 20, 20, |x, y, ui| {
                let square = board.get_squares()[x][y].clone();

                let color = match square {
                    Square::Empty => 0xFFFFFF,
                    Square::Blue => 0x0000FF,
                    Square::Green => 0x00FF00,
                    Square::Red => 0xFF0000,
                    Square::Yellow => 0xFFFF00,
                };

                if ui.just_pressed() {
                    // Rotate through possible colors
                    let next_color = match square {
                        Square::Empty => Square::Blue,
                        Square::Blue => Square::Green,
                        Square::Green => Square::Red,
                        Square::Red => Square::Yellow,
                        Square::Yellow => Square::Empty,
                    };

                    board.set_square(x, y, next_color);
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
            for x in 0..height {
                ui.element()
                    .height(Sizing::Percent(1. / height as f32))
                    .width(grow!())
                    // .background_color(0xCCCCCC)
                    .layout(|l| l.direction(LayoutDirection::LeftToRight).gap(1))
                    .children(|ui| {
                        for y in 0..width {
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
