use ggez::graphics::Color;

pub struct Colours {
    pub primary_text: Color,
    pub secondary_text: Color,
    pub text_background: Color,

    pub ship: Color,
    pub missile: Color,
    pub asteroid: Color,
    pub star: Color,
}

impl Colours {
    pub fn new() -> Colours {
        Colours {
            primary_text: Color::from_rgb(255, 255, 255),
            secondary_text: Color::from_rgb(189, 189, 189),
            text_background: Color::from_rgba(33, 33, 33, 191),

            ship: Color::from_rgb(236, 239, 241),
            missile: Color::from_rgb(216, 27, 96),
            asteroid: Color::from_rgb(78, 52, 46),
            star: Color::from_rgb(225, 245, 254),
        }
    }
}
