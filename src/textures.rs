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

        let mut enemy_image = Image::gen_image_color(64, 64, Color::BLANK);
        enemy_image.draw_rectangle(20, 20, 24, 30, Color::new(180, 70, 90, 255));
        enemy_image.draw_rectangle(14, 12, 12, 12, Color::new(180, 70, 90, 255));
        enemy_image.draw_rectangle(38, 12, 12, 12, Color::new(180, 70, 90, 255));
        enemy_image.draw_rectangle(25, 25, 5, 5, Color::WHITE);
        enemy_image.draw_rectangle(34, 25, 5, 5, Color::WHITE);
        enemy_image.draw_rectangle(29, 40, 6, 5, Color::new(70, 40, 50, 255));
        let enemy_texture = rl
            .load_texture_from_image(thread, &enemy_image)
            .expect("No se pudo crear la textura del enemigo");
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
