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

        let mural_image = create_mural_image();
        let mural_texture = rl
            .load_texture_from_image(thread, &mural_image)
            .expect("No se pudo crear la textura del mural");
        images.insert('m', mural_image);
        textures.insert('m', mural_texture);

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

fn create_mural_image() -> Image {
    let mut image = Image::gen_image_color(96, 96, Color::BLANK);
    let outline = Color::new(54, 59, 57, 255);
    let stone = Color::new(128, 134, 130, 255);
    let shadow = Color::new(82, 88, 85, 255);
    let light = Color::new(177, 184, 178, 255);
    let carving = Color::new(62, 68, 65, 255);
    let moss = Color::new(67, 93, 77, 255);

    image.draw_rectangle(30, 3, 36, 7, outline);
    image.draw_rectangle(25, 7, 46, 78, outline);
    image.draw_rectangle(21, 14, 54, 64, outline);
    image.draw_rectangle(33, 6, 30, 5, light);
    image.draw_rectangle(28, 10, 40, 72, stone);
    image.draw_rectangle(24, 17, 48, 58, stone);

    image.draw_rectangle(29, 20, 38, 53, shadow);
    image.draw_rectangle(32, 23, 32, 47, Color::new(110, 116, 112, 255));
    image.draw_rectangle(32, 38, 32, 3, carving);
    image.draw_rectangle(32, 54, 32, 3, carving);

    image.draw_rectangle(40, 29, 6, 4, carving);
    image.draw_rectangle(50, 29, 6, 4, carving);
    image.draw_rectangle(38, 46, 20, 5, carving);
    image.draw_rectangle(39, 61, 18, 7, carving);
    image.draw_rectangle(43, 58, 10, 13, carving);
    image.draw_rectangle(46, 61, 4, 7, Color::new(110, 116, 112, 255));

    image.draw_rectangle(17, 82, 62, 10, outline);
    image.draw_rectangle(21, 85, 54, 5, stone);
    image.draw_rectangle(29, 12, 4, 13, light);
    image.draw_rectangle(64, 47, 4, 18, shadow);
    image.draw_rectangle(26, 69, 5, 8, moss);
    image.draw_rectangle(69, 22, 4, 11, moss);
    image
}
