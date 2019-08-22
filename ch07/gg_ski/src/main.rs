use ggez::{
    conf,
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::{DrawMode, Rect},
    timer, Context, ContextBuilder, GameResult,
};
use std::f32::consts::PI;
use std::time::Duration;

type Point2 = nalgebra::Point2<f32>;

const SCREEN_WIDTH: f32 = 800.; // in pixels
const SCREEN_HEIGHT: f32 = 600.; // in pixels
const SKI_WIDTH: f32 = 10.; // in pixels
const SKI_LENGTH: f32 = 50.; // in pixels
const SKI_TIP_LEN: f32 = 20.; // in pixels
const STEERING_SPEED: f32 = 110. / 180. * PI; // in radians/second
const MAX_ANGLE: f32 = 75. / 180. * PI; // in radians

#[derive(Debug)]
struct InputState {
    to_turn: f32,
    started: bool,
}

struct Screen {
    ski_across_offset: f32, // in pixels
    direction: f32,         // in radians
    previous_frame_time: Duration,
    period_in_sec: f32, // in seconds
    input: InputState,
}

impl Screen {
    fn new(_ctx: &mut Context) -> GameResult<Screen> {
        let s = Screen {
            ski_across_offset: 0.,
            direction: 0.,
            previous_frame_time: Duration::from_secs(0),
            period_in_sec: 0.,
            input: InputState {
                to_turn: 0.0,
                started: false,
            },
        };
        Ok(s)
    }

    fn steer(&mut self, side: f32) {
        if side == 0. {
            return;
        }
        self.direction += STEERING_SPEED * self.period_in_sec * side;
        if self.direction > MAX_ANGLE {
            self.direction = MAX_ANGLE;
        } else if self.direction < -MAX_ANGLE {
            self.direction = -MAX_ANGLE;
        }
    }
}

impl EventHandler for Screen {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 25;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let now = timer::time_since_start(ctx);
            self.period_in_sec = (now - self.previous_frame_time).as_millis() as f32 / 1000.;
            self.previous_frame_time = now;
            self.steer(self.input.to_turn);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let ski = graphics::MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect {
                    x: -SKI_WIDTH / 2.,
                    y: SKI_TIP_LEN,
                    w: SKI_WIDTH,
                    h: SKI_LENGTH,
                },
                [1., 0., 1., 1.].into(),
            )
            .polygon(
                DrawMode::fill(),
                &[
                    Point2::new(-SKI_WIDTH / 2., SKI_TIP_LEN),
                    Point2::new(SKI_WIDTH / 2., SKI_TIP_LEN),
                    Point2::new(0., 0.),
                ],
                [0.5, 0., 1., 1.].into(),
            )?
            .build(ctx)?;
        graphics::draw(
            ctx,
            &ski,
            graphics::DrawParam::new()
                .dest(Point2::new(
                    SCREEN_WIDTH / 2. + self.ski_across_offset,
                    SCREEN_HEIGHT * 15. / 16. - SKI_LENGTH / 2. - SKI_TIP_LEN,
                ))
                .rotation(self.direction),
        )?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => {
                self.input.to_turn = -1.0;
            }
            KeyCode::Right => {
                self.input.to_turn = 1.0;
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left | KeyCode::Right => {
                self.input.to_turn = 0.0;
            }
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let (context, animation_loop) = &mut ContextBuilder::new("slalom", "ggez")
        .window_setup(conf::WindowSetup::default().title("Slalom"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("static")
        .build()?;
    let game = &mut Screen::new(context)?;
    event::run(context, animation_loop, game)
}
