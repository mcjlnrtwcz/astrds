extern crate ggez;
use ggez::graphics::DrawMode;
use ggez::*;
use rand::Rng;

struct MainState {
    stars: Vec<(f32, f32)>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // Generate initial stars - background
        let mut stars: Vec<(f32, f32)> = Vec::new();
        for _i in 0..100 {
            stars.push((
                rand::thread_rng().gen_range(0.0, 800.0),
                rand::thread_rng().gen_range(0.0, 600.0),
            ));
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
        let mut rect = graphics::Rect::one();
        for star in self.stars.iter_mut() {
            if star.1 >= 600.0 {
                star.0 = rand::thread_rng().gen_range(0.0, 800.0);
                star.1 = 0.0;
            } else {
                star.1 += 1.0;
            }
            rect.x = star.0;
            rect.y = star.1;
            graphics::rectangle(ctx, DrawMode::Fill, rect)?;
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
