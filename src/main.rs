extern crate ggez;
use ggez::graphics::{DrawMode, Point2};
use ggez::*;
use rand::Rng;
mod entities;

const BACKGROUND_STARS_NUM: usize = 100;

type BackgroundStars = [Point2; BACKGROUND_STARS_NUM];

struct MainState {
    width: f32,
    height: f32,
    stars: BackgroundStars,
    ship: entities::Ship,
    missiles: Vec<entities::Missile>,
    asteroids: Vec<entities::Asteroid>,
    asteroids_generated_at: usize,
    game_over: bool,
}

impl MainState {
    fn generate_initial_stars(screen_width: f32, screen_height: f32) -> BackgroundStars {
        let mut stars = [Point2::new(0.0, 0.0); BACKGROUND_STARS_NUM];
        for i in 0..100 {
            stars[i].x = rand::thread_rng().gen_range(0.0, screen_width);
            stars[i].y = rand::thread_rng().gen_range(0.0, screen_height);
        }
        stars
    }

    fn generate_initial_asteroids(
        screen_width: f32,
        screen_height: f32,
        number: usize,
    ) -> Vec<entities::Asteroid> {
        let mut asteroids: Vec<entities::Asteroid> = Vec::new();
        for _i in 0..number {
            let x = rand::thread_rng().gen_range(0.0, screen_width);
            let y = rand::thread_rng().gen_range(0.0, (screen_height) / 2.0);
            asteroids.push(entities::Asteroid::new(x, y));
        }
        asteroids
    }

    fn generate_asteroids(&self, number: usize) -> Vec<entities::Asteroid> {
        let mut asteroids: Vec<entities::Asteroid> = Vec::new();
        for _i in 0..number {
            let x = rand::thread_rng().gen_range(0.0, self.width);
            let y = 0.0;
            asteroids.push(entities::Asteroid::new(x, y));
        }
        asteroids
    }

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (width, height) = graphics::get_size(ctx);
        let width = width as f32;
        let height = height as f32;
        let state = MainState {
            width: width,
            height: height,
            stars: MainState::generate_initial_stars(width, height),
            ship: entities::Ship::new(30.0, 8.0, height),
            missiles: Vec::new(),
            asteroids: MainState::generate_initial_asteroids(width, height, 10),
            asteroids_generated_at: 0,
            game_over: false,
        };
        Ok(state)
    }

    fn move_stars(&mut self) {
        for star in self.stars.iter_mut() {
            if star.y >= self.height {
                star.x = rand::thread_rng().gen_range(0.0, self.width);
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
            .retain(|missile| missile.rect.y > -missile.rect.h && !missile.should_be_deleted);
    }

    fn move_asteroids(&mut self) {
        self.asteroids
            .iter_mut()
            .for_each(|asteroid| asteroid.rect.y += 1.0);
        let height = self.height;
        self.asteroids.retain(|asteroid| asteroid.rect.y < height);
    }

    fn rects_touching_horizontally(first: graphics::Rect, second: graphics::Rect) -> bool {
        let first_middle = first.x + first.w / 2.0;
        let second_middle = second.x + second.w / 2.0;
        let diff = (first_middle - second_middle).abs();
        let max_diff = first.w / 2.0 + second.w / 2.0;
        diff <= max_diff
    }

    fn handle_collisions(&mut self) {
        // Did asteroid hit the ship?
        for asteroid in self.asteroids.iter() {
            let asteroid_bottom = asteroid.rect.y + asteroid.rect.h;
            let ship_bottom = self.ship.rect.y + self.ship.rect.h;
            let touching_top = asteroid_bottom >= self.ship.rect.y && asteroid_bottom < ship_bottom;
            if touching_top {
                if MainState::rects_touching_horizontally(self.ship.rect, asteroid.rect) {
                    self.game_over = true;
                    return;
                }
            }
        }
        // Did missile hit the asteroid?
        for missile in self.missiles.iter_mut() {
            let asteroids_before = self.asteroids.len();
            self.asteroids.retain(|asteroid| {
                let touching_bottom = missile.rect.y >= asteroid.rect.y
                    && missile.rect.y <= asteroid.rect.y + asteroid.rect.h;
                !(touching_bottom && MainState::rects_touching_horizontally(missile.rect, asteroid.rect))
            });
            // Should missile be deleted?
            let asteroids_after = self.asteroids.len();
            missile.should_be_deleted = asteroids_before > asteroids_after;
        }
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
        if keycode == event::Keycode::Right && self.ship.rect.x < self.width - self.ship.rect.w {
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

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 30) {
            self.move_stars();
            self.move_missiles();
            self.move_asteroids();
            self.handle_collisions();

            // Add new asteroids
            let ticks = timer::get_ticks(ctx);
            if ticks - self.asteroids_generated_at > 40 {
                self.asteroids_generated_at = ticks;
                let mut new_asteroids = self.generate_asteroids(1);
                self.asteroids.append(&mut new_asteroids);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        if !self.game_over {
            // Draw stars
            graphics::set_color(ctx, graphics::Color::from_rgb(255, 255, 255))?;
            graphics::points(ctx, &self.stars, 1.0)?;

            // Draw ship
            graphics::set_color(ctx, graphics::Color::from_rgb(236, 239, 241))?;
            graphics::rectangle(ctx, DrawMode::Fill, self.ship.rect)?;

            // Draw missiles
            graphics::set_color(ctx, graphics::Color::from_rgb(216, 27, 96))?;
            for missile in self.missiles.iter() {
                graphics::rectangle(ctx, DrawMode::Fill, missile.rect)?;
            }

            // Draw asteroids
            graphics::set_color(ctx, graphics::Color::from_rgb(78, 52, 46))?;
            for asteroid in self.asteroids.iter() {
                graphics::rectangle(ctx, DrawMode::Fill, asteroid.rect)?;
            }
        }

        graphics::present(ctx);
        timer::yield_now();
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
