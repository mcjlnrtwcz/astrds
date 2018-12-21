extern crate ggez;
use ggez::graphics::DrawMode;
use ggez::*;
use rand::Rng;

struct MainState {}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState {};
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
        for _i in 0..10 {
            rect.x = rand::thread_rng().gen_range(0.0, 800.0);
            rect.y = rand::thread_rng().gen_range(0.0, 600.0);
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
