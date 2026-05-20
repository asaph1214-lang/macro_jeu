// IMPORTATION DU NECCESSAIRE POUR LE RENDU GRAPHIQUE
use macroquad::prelude::*;

// CREATION D'UNE STRUCTURE POUR GERER LES FORMES
// 1- UNE FORME A UNE VITESSE
// 2- UNE FORME A UNE TAILLE
// 3- UNE FORME A UNE POSITION EN X ET EN Y
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

// IMPLEMENTAION DES METHODES POUR UNE FORMES
// 1- UNE FORME PEUT ENTRER EN COLLISION AVEC D'AUTRES FORMES
// 2- UNE FORME PEUT DONNER SES LIMITATION GEOMETRIQUES
impl Shape {

    // POUR SAVOIR SI LA FORME EST ENTRER EN COLLISION OU PAS
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    // POUR EXPOSER LES LIMITATIONS GEOMETRIQUES DE LA FORME
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

// INITIALISATION DU JEU A LA FACON MACROQUAD JE MENSIONNE LE NOM
#[macroquad::main("My game")]
async fn main() {

    // JE DEFINI LA VITESSE DE MA FORME(MON CERCLE)
    const MOVEMENT_SPEED: f32 = 200.0;

    // J'INITIALISE LE GENERATEUR DE NBRE ALEATOIRE
    // POUR L'INSTANT DE DEPART ( MAINTENANT )
    rand::srand(miniquad::date::now() as u64);

    // J'INITIALISE LA COLLECTION DE CARREE ENEMIES
    let mut squares = vec![];

    // J'INITIALISE LA COLLECTION DE BALLE
    let mut bullets: Vec<Shape> = vec![];


    // CONSTRUCTION DU CERCLE HERO
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };

    // INITIALISATION DU DRAPEAU QUI DEFINI QUAND LE JOUEUR A PERDUE
    let mut gameover = false;

    // BOUCLE DE JEU
    // 1- RECUPERATION DES EVENEMENTS
    // 2- MISE AJOUR DE LA LOGIQUE
    // 3- RESOLUTION DES REGLES DU JEU
    // 1 ET 2 SE FONT PENDANT QUE LE JEU TOURNE !GAMEOVER
    // 3 EST INDEPENDANT DE TOUS LE RESTE 
    loop {

        // ON NETOIE L'ECRAN EN DARKPURPLE
        clear_background(DARKPURPLE);

        // NOUS SOMMES DANS LE CAS OU LE JOUEUR EST ENCORE EN VIE
        if !gameover {

            // ON RECUPERE LE TEMPS MIS PAR L'IMAGE POUR S'AFFICHER 
            let delta_time = get_frame_time();

            // ON QUESTIONNE LA LISTE DES EVENEMENTS 
            // POUR SAVOIR CEUX QUI ONT ETE DECLENCHER
            // ON CONVERTIS LES EVENEMENT EN INTENTIONS
            if is_key_down(KeyCode::Right) {circle.x += MOVEMENT_SPEED * delta_time;}
            if is_key_down(KeyCode::Left)  {circle.x -= MOVEMENT_SPEED * delta_time;}
            if is_key_down(KeyCode::Down)  {circle.y += MOVEMENT_SPEED * delta_time;}
            if is_key_down(KeyCode::Up)    {circle.y -= MOVEMENT_SPEED * delta_time;}
            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    size: 5.0, 
                    speed: circle.speed * 2.0, 
                    x: circle.x, 
                    y: circle.y, 
                    collided: false 
                });
            }

            // ON LIMITE LES MOUVEMENT DU CERCLE POUR
            // QU'IL NE SORTENT PAS DE L'ECRAN
            circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
            circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

            // NOUS DEFINISSONS EXPLICITEMENT LA PROBABILITER 
            // D'APPARITION DES CARREE ENEMEIES
            if rand::gen_range(0, 99) >= 95 {

                // GENERATION D'UNE TAILLE ALEATOIRE
                let size = rand::gen_range(16.0, 64.0);

                // ON AJOUTE L'ENEMIE CREER A LA LISTE
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(100.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    collided: false,
                });
            }

            // ON MET A JOUR LA HAUTEUR DES CARREE ENEMIES
            // EN CONTINUE 
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // ON MET A JOUR LA HAUTEUR DES BULLET
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            // ON SUPPRIME LES CARREE QUI SORTENT DE LA LIMITES
            squares.retain(|square| square.y < screen_height() + square.size);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);
        }


        // ON VERFIE SI IL Y A EU COLLISION
        // SI OUI LE JEU S'ARRETE GAMEOVER = TRUE
        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }
        // ON APPUIE SUR ESPACE POUR RECOMMENCER
        if gameover && is_key_pressed(KeyCode::Space) {
            bullets.clear();
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }

        // ON DESSINE TOUT CE QU'IL FAUT DESSINER
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        // QUE FAIRE SI IL Y A GAMEOVER
        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}
