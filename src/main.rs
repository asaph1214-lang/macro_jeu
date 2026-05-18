use macroquad::prelude::*;
// bienvenu dans le jeu la crate la plus use au monde
#[macroquad::main("Mon jeu")]
async fn main() {

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    const MOUVEMENT_SPEED :f32 = 200.0;
    
    loop {

        clear_background(DARKBLUE);
        
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Right) { x += MOUVEMENT_SPEED*delta_time; }
        if is_key_down(KeyCode::Left)  { x -= MOUVEMENT_SPEED*delta_time; }
        if is_key_down(KeyCode::Down)  { y += MOUVEMENT_SPEED*delta_time; }
        if is_key_down(KeyCode::Up)    { y -= MOUVEMENT_SPEED*delta_time; }

        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());

       draw_circle(x, y, 16.0, YELLOW);

        next_frame().await;
    }
}
