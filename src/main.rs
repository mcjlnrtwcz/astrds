extern crate ggez;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;
use rand::Rng;
mod entities;

const BACKGROUND_STARS_NUM: usize = 100;

type BackgroundStars = [Point2; BACKGROUND_STARS_NUM];

struct MainState {
    width: u32,
    height: u32,
    stars: BackgroundStars,
    ship: entities::Ship,
    missiles: Vec<entities::Missile>,
    asteroids: Vec<entities::Asteroid>,
    asteroids_generated_at: usize,
}

impl MainState {
    fn generate_stars(screen_width: u32, screen_height: u32) -> BackgroundStars {
        let mut stars = [Point2::new(0.0, 0.0); BACKGROUND_STARS_NUM];
        for i in 0..100 {
            // TODO: Stars proximity
            stars[i].x = rand::thread_rng().gen_range(0.0, screen_width as f32);
            stars[i].y = rand::thread_rng().gen_range(0.0, screen_height as f32);
        }
        stars
    }

    fn generate_initial_asteroids(
        screen_width: u32,
        screen_height: u32,
        number: usize,
    ) -> Vec<entities::Asteroid> {
        let mut asteroids: Vec<entities::Asteroid> = Vec::new();
        for _i in 0..number {
            let x = rand::thread_rng().gen_range(0.0, screen_width as f32);
            let y = rand::thread_rng().gen_range(0.0, (screen_height as f32) / 2.0);
            asteroids.push(entities::Asteroid::new(x, y));
        }
        asteroids
    }

    fn generate_asteroids(&self, number: usize) -> Vec<entities::Asteroid> {
        let mut asteroids: Vec<entities::Asteroid> = Vec::new();
        for _i in 0..number {
            let x = rand::thread_rng().gen_range(0.0, self.width as f32);
            let y = 0.0;
            asteroids.push(entities::Asteroid::new(x, y));
        }
        asteroids
    }

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (width, height) = graphics::get_size(ctx);
        let state = MainState {
            width: width,
            height: height,
            stars: MainState::generate_stars(width, height),
            ship: entities::Ship::new(30.0, 8.0, height as f32),
            missiles: Vec::new(),
            asteroids: MainState::generate_initial_asteroids(width, height, 10),
            asteroids_generated_at: 0,
        };
        Ok(state)
    }

    fn move_stars(&mut self) {
        for star in self.stars.iter_mut() {
            if star.y >= self.height as f32 {
                star.x = rand::thread_rng().gen_range(0.0, self.width as f32);
                star.y = 0.0;
            } else {
                star.y += 0.5;
            }
        }
    }

    fn move_missiles(&mut self) {
        self.missiles
            .iter_mut()
            .for_each(|missile| missile.rect.y -= 3.0);
        self.missiles
            .retain(|missile| missile.rect.y > -missile.rect.h);
    }

    fn move_asteroids(&mut self) {
        self.asteroids
            .iter_mut()
            .for_each(|asteroid| asteroid.rect.y += 1.0);
        let height = self.height;
        self.asteroids
            .retain(|asteroid| asteroid.rect.y < height as f32);
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
            self.missiles.push(entities::Missile::new(
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

        // Draw stars
        graphics::set_color(ctx, graphics::Color::from_rgb(255, 255, 255))?;
        self.move_stars();
        graphics::points(ctx, &self.stars, 1.0)?;

        // Draw ship
        graphics::set_color(ctx, graphics::Color::from_rgb(236, 239, 241))?;
        graphics::rectangle(ctx, DrawMode::Fill, self.ship.rect)?;

        // Move missiles, delete invisible missiles
        graphics::set_color(ctx, graphics::Color::from_rgb(216, 27, 96))?;
        self.move_missiles();
        for missile in self.missiles.iter() {
            graphics::rectangle(ctx, DrawMode::Fill, missile.rect)?;
        }

        // Draw asteroids
        graphics::set_color(ctx, graphics::Color::from_rgb(78, 52, 46))?;
        self.move_asteroids();
        for asteroid in self.asteroids.iter() {
            graphics::rectangle(ctx, DrawMode::Fill, asteroid.rect)?;
        }
        let ticks = timer::get_ticks(ctx);
        if ticks - self.asteroids_generated_at > 80 {
            self.asteroids_generated_at = ticks;
            let mut new_asteroids = self.generate_asteroids(2);
            self.asteroids.append(&mut new_asteroids);
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
