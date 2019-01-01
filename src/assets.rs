use ggez::*;

pub struct Assets {
    pub font: graphics::Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Assets {
        Assets {
            font: graphics::Font::new(ctx, "/0xA000-Regular.ttf", 12).unwrap(),
        }
    }
}
