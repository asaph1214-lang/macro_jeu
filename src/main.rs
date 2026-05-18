use macroquad::prelude::*;
#[macroquad::main("Mon jeu")]
async fn main() {
    loop {
        clear_background(DARKBLUE);

        draw_circle(
            screen_width() / 2.0,
            screen_height() / 2.0,
            50.0, YELLOW
        );

        draw_text(
            "bonjour macroquad",
            20.0, 30.0, 30.0,
            WHITE
        );

        next_frame().await;
    }
}
