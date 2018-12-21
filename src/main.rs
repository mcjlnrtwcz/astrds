extern crate ggez;
use ggez::graphics::Point2;
use ggez::*;
use rand::Rng;

const BACKGROUND_STARS_NUM: usize = 100;

type BackgroundStars = [Point2; BACKGROUND_STARS_NUM];

struct MainState {
    width: u32,
    height: u32,
    stars: BackgroundStars,
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
        };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::points(ctx, self.move_stars(self.width, self.height), 1.0)?;
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
