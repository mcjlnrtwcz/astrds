use ggez::*;

pub struct Missile {
    pub rect: graphics::Rect,
}

impl Missile {
    pub fn new(ship_x: f32, ship_y: f32, ship_size: f32) -> Missile {
        Missile {
            rect: graphics::Rect::new(
                ship_x + ship_size / 2.0 - 10.0 / 2.0,
                ship_y - 40.0,
                10.0,
                40.0,
            ),
        }
    }
}

pub struct Ship {
    pub rect: graphics::Rect,
    pub velocity: f32,
}

impl Ship {
    pub fn new(size: f32, velocity: f32, screen_height: f32) -> Ship {
        Ship {
            rect: graphics::Rect::new(0.0, screen_height - size * 2.0, size, size),
            velocity: velocity,
        }
    }
}

pub struct Asteroid {
    pub rect: graphics::Rect,
    pub velocity: f32,
}

impl Asteroid {
    pub fn new(x: f32, y: f32) -> Asteroid {
        Asteroid {
            rect: graphics::Rect::new(x, y, 20.0, 20.0),
            velocity: 1.0,
        }
    }
}
