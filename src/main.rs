use ply_engine::prelude::*;

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Hello Ply!".to_owned(),
            window_width: 800,
            window_height: 600,
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
    static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/inter.ttf");
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;

    loop {
        clear_background(BLACK);

        let mut ui = ply.begin();

        ui.element()
            .width(grow!())
            .height(grow!())
            .layout(|l| l.align(CenterX, CenterY))
            .children(|ui| {
                ui.text("Hello, Ply!", |t| t.font_size(32).color(0xFFFFFF));
            });

        ui.show(|_| {}).await;

        next_frame().await;
    }
}
