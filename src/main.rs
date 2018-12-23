extern crate ggez;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;
use rand::Rng;

const BACKGROUND_STARS_NUM: usize = 100;

type BackgroundStars = [Point2; BACKGROUND_STARS_NUM];

struct Missile {
    rect: graphics::Rect,
}

impl Missile {
    fn new(ship_x: f32, ship_y: f32, ship_size: f32) -> Missile {
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

struct Ship {
    rect: graphics::Rect,
    velocity: f32,
}

impl Ship {
    fn new(size: f32, velocity: f32, screen_height: f32) -> Ship {
        Ship {
            rect: graphics::Rect::new(0.0, screen_height - size * 2.0, size, size),
            velocity: velocity,
        }
    }
}

struct MainState {
    width: u32,
    height: u32,
    stars: BackgroundStars,
    ship: Ship,
    missiles: Vec<Missile>,
}

impl MainState {
    fn generate_stars(width: u32, height: u32) -> BackgroundStars {
        let mut stars = [Point2::new(0.0, 0.0); BACKGROUND_STARS_NUM];
        for i in 0..100 {
            // TODO: Stars proximity
            stars[i].x = rand::thread_rng().gen_range(0.0, width as f32);
            stars[i].y = rand::thread_rng().gen_range(0.0, height as f32);
        }
        stars
    }

    fn move_stars(&mut self, width: u32, height: u32) -> &BackgroundStars {
        for star in self.stars.iter_mut() {
            if star.y >= height as f32 {
                star.x = rand::thread_rng().gen_range(0.0, width as f32);
                star.y = 0.0;
            } else {
                star.y += 1.0;
            }
        }
        &self.stars
    }

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (width, height) = graphics::get_size(ctx);
        let state = MainState {
            width: width,
            height: height,
            stars: MainState::generate_stars(width, height),
            ship: Ship::new(30.0, 8.0, height as f32),
            missiles: Vec::new(),
        };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool,
    ) {
        // Move ship
        if keycode == event::Keycode::Right
            && self.ship.rect.x < self.width as f32 - self.ship.rect.w
        {
            self.ship.rect.x += self.ship.velocity; // TODO: As Ship's method
        } else if keycode == event::Keycode::Left && self.ship.rect.x > 0.0 {
            self.ship.rect.x -= self.ship.velocity; // TODO: As Ship's method
        } else if keycode == event::Keycode::Space {
            // Shoot
            self.missiles.push(Missile::new(
                self.ship.rect.x,
                self.ship.rect.y,
                self.ship.rect.w,
            ));
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::points(ctx, self.move_stars(self.width, self.height), 1.0)?;
        // Draw ship
        graphics::rectangle(ctx, DrawMode::Fill, self.ship.rect)?;
        // Move missiles, delete invisible missiles
        self.missiles
            .iter_mut()
            .for_each(|missile| missile.rect.y -= 3.0);
        self.missiles
            .retain(|missile| missile.rect.y > -missile.rect.h);
        for missile in self.missiles.iter() {
            graphics::rectangle(ctx, DrawMode::Fill, missile.rect)?;
        }
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut config = conf::Conf::new();
    config.window_setup.title = "astrds".to_owned();

    let ctx = &mut Context::load_from_conf("astrds", "mcjlnrtwcz", config).unwrap();
    graphics::set_background_color(ctx, graphics::Color::from_rgb(0, 0, 0));
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
