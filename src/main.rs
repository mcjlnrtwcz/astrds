extern crate ggez;
use ggez::graphics::Point2;
use ggez::*;
use rand::Rng;

struct MainState {
    stars: [Point2; 100],
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // Generate initial stars - background
        let mut stars = [Point2::new(0.0, 0.0); 100];
        for i in 0..100 {
            stars[i].x = rand::thread_rng().gen_range(0.0, 800.0);
            stars[i].y = rand::thread_rng().gen_range(0.0, 600.0);
        }

        let state = MainState { stars: stars };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        for star in self.stars.iter_mut() {
            if star.y >= 600.0 {
                star.x = rand::thread_rng().gen_range(0.0, 800.0);
                star.y = 0.0;
            } else {
                star.y += 1.0;
            }
        }
        graphics::points(ctx, &self.stars, 1.0)?;
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
