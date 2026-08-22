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

        let texture_colors = [
            ('+', Color::new(107, 142, 91, 255)),
            ('-', Color::new(125, 155, 100, 255)),
            ('|', Color::new(90, 130, 80, 255)),
            ('g', Color::new(150, 120, 170, 255)),
            ('#', Color::new(107, 142, 91, 255)),
        ];

        for (character, color) in texture_colors {
            let mut image = Image::gen_image_color(64, 64, color);
            image.draw_rectangle(0, 0, 64, 5, Color::new(255, 255, 255, 35));
            image.draw_rectangle(0, 59, 64, 5, Color::new(0, 0, 0, 35));
            image.draw_rectangle(8, 0, 4, 64, Color::new(255, 255, 255, 25));
            image.draw_rectangle(40, 0, 4, 64, Color::new(0, 0, 0, 25));

            let texture = rl
                .load_texture_from_image(thread, &image)
                .expect("No se pudo crear una textura");
            images.insert(character, image);
            textures.insert(character, texture);
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

    pub fn get_texture(&self, character: char) -> Option<&Texture2D> {
        self.textures.get(&character)
    }
}
