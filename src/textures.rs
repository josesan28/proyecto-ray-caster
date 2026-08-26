use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    images: HashMap<char, Image>,
    textures: HashMap<char, Texture2D>,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        let texture_files = [
            ('#', "assets/textures/zaculeu_stone.png"),
            ('J', "assets/textures/zaculeu_jungle.png"),
            ('S', "assets/textures/zaculeu_stela.png"),
            ('D', "assets/textures/zaculeu_door.png"),
        ];

        for (character, path) in texture_files {
            let mut image = Image::load_image(path)
                .unwrap_or_else(|_| panic!("No se pudo cargar la textura {path}"));
            image.resize(256, 256);

            let texture = rl
                .load_texture_from_image(thread, &image)
                .expect("No se pudo crear una textura");
            images.insert(character, image);
            textures.insert(character, texture);
        }

        let mut enemy_image = Image::load_image("assets/textures/zaculeu_enemy.png")
            .expect("No se pudo cargar la textura de Kukulkán");
        enemy_image.resize(256, 256);
        let enemy_texture = rl
            .load_texture_from_image(thread, &enemy_image)
            .expect("No se pudo crear la textura de Kukulkán");
        images.insert('e', enemy_image);
        textures.insert('e', enemy_texture);

        let mut artifact_image = Image::load_image("assets/textures/zaculeu_artifact.png")
            .expect("No se pudo cargar la textura del artefacto");
        artifact_image.resize(128, 128);
        let artifact_texture = rl
            .load_texture_from_image(thread, &artifact_image)
            .expect("No se pudo crear la textura de la máscara");
        images.insert('a', artifact_image);
        textures.insert('a', artifact_texture);

        for digit in ['2', '5', '0'] {
            let image = create_clue_image(digit);
            let texture = rl
                .load_texture_from_image(thread, &image)
                .expect("No se pudo crear la textura de una pista maya");
            images.insert(digit, image);
            textures.insert(digit, texture);
        }

        Self { images, textures }
    }

    pub fn get_pixel_color(&self, character: char, tx: u32, ty: u32) -> Color {
        let Some(image) = self.images.get(&character) else {
            return Color::WHITE;
        };

        let x = tx.min(image.width.saturating_sub(1) as u32) as i32;
        let y = ty.min(image.height.saturating_sub(1) as u32) as i32;
        image.get_color(x, y)
    }

    pub fn get_size(&self, character: char) -> (u32, u32) {
        self.images
            .get(&character)
            .map(|image| (image.width as u32, image.height as u32))
            .unwrap_or((1, 1))
    }

    pub fn get_texture(&self, character: char) -> Option<&Texture2D> {
        self.textures.get(&character)
    }
}

fn create_clue_image(digit: char) -> Image {
    let mut image = Image::gen_image_color(64, 64, Color::BLANK);
    let outline = Color::new(69, 52, 38, 255);
    let stone = Color::new(198, 175, 126, 255);
    let light = Color::new(232, 213, 164, 255);
    let jade = Color::new(35, 120, 91, 255);

    image.draw_rectangle(14, 6, 36, 52, outline);
    image.draw_rectangle(10, 12, 44, 40, outline);
    image.draw_rectangle(16, 8, 32, 48, stone);
    image.draw_rectangle(12, 14, 40, 36, stone);
    image.draw_rectangle(16, 10, 28, 4, light);
    image.draw_rectangle(46, 18, 4, 20, Color::new(146, 119, 82, 255));

    match digit {
        '2' => {
            for center_x in [24, 40] {
                image.draw_rectangle(center_x - 5, 26, 10, 6, outline);
                image.draw_rectangle(center_x - 3, 24, 6, 10, outline);
                image.draw_rectangle(center_x - 3, 26, 6, 6, jade);
            }
        }
        '5' => {
            image.draw_rectangle(15, 25, 34, 14, outline);
            image.draw_rectangle(18, 28, 28, 8, jade);
            image.draw_rectangle(20, 28, 24, 3, Color::new(78, 173, 129, 255));
        }
        '0' => {
            image.draw_rectangle(17, 20, 30, 24, outline);
            image.draw_rectangle(13, 25, 38, 14, outline);
            image.draw_rectangle(20, 23, 24, 18, jade);
            image.draw_rectangle(17, 28, 30, 8, jade);
            image.draw_rectangle(25, 27, 14, 10, stone);
            image.draw_rectangle(28, 30, 8, 7, outline);
        }
        _ => {}
    }

    image
}
