use ggez::{
    audio,
    audio::SoundSource,
    conf,
    event::{self, EventHandler, KeyCode, KeyMods, MouseButton},
    graphics,
    graphics::{DrawMode, Font, Rect},
    input::mouse,
    timer, Context, ContextBuilder, GameResult,
};
use rand::prelude::*;
use std::time::Duration;

type Vector2 = nalgebra::Vector2<f32>;
type Point2 = nalgebra::Point2<f32>;

const N_COLUMNS: usize = 5;
const N_ROWS: usize = 3;
const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;
const FIRST_COLUMN_X: f32 = 60.;
const COLUMNS_STEP: f32 = 140.;
const FIRST_ROW_Y: f32 = 140.;
const ROWS_STEP: f32 = 150.;
const GAME_DURATION: Duration = Duration::from_secs(40);

#[derive(Debug)]
enum Mode {
    Ready,
    Raising,
    Lowering,
    Finished,
}

struct Screen {
    mode: Mode,
    start_time: Duration,
    active_mole_column: usize,
    active_mole_row: usize,
    active_mole_position: f32,
    mallet_image: graphics::Image,
    lawn_image: graphics::Image,
    mole_image: graphics::Image,
    font: graphics::Font,
    appearance_sound: audio::Source,
    hit_sound: audio::Source,
    miss_sound: audio::Source,
    finish_sound: audio::Source,
    clicked_at: Option<Point2>,
}

impl Screen {
    fn new(ctx: &mut Context) -> GameResult<Screen> {
        Ok(Screen {
            mode: Mode::Ready,
            start_time: Duration::from_secs(0),
            active_mole_column: 0,
            active_mole_row: 0,
            active_mole_position: 0.,
            mallet_image: graphics::Image::new(ctx, "/mallet.png")?,
            lawn_image: graphics::Image::new(ctx, "/lawn.jpg")?,
            mole_image: graphics::Image::new(ctx, "/mole.png")?,
            font: Font::new(ctx, "/font.ttf")?,
            appearance_sound: audio::Source::new(ctx, "/click.ogg")?,
            hit_sound: audio::Source::new(ctx, "/click.ogg")?,
            miss_sound: audio::Source::new(ctx, "/click.ogg")?,
            finish_sound: audio::Source::new(ctx, "/click.ogg")?,
            clicked_at: None,
        })
    }

    fn get_active_mole_bounding_box(&self) -> Rect {
        Rect::new(
            FIRST_COLUMN_X + self.active_mole_column as f32 * COLUMNS_STEP,
            FIRST_ROW_Y + self.active_mole_row as f32 * ROWS_STEP
                - 0.2 * self.active_mole_position * self.mole_image.height() as f32,
            0.2 * self.mole_image.width() as f32,
            0.2 * self.active_mole_position * self.mole_image.height() as f32,
        )
    }
}

impl EventHandler for Screen {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut rng = thread_rng();

        const DESIRED_FPS: u32 = 25;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            match self.mode {
                Mode::Ready => {
                    // If clicked on the Start button,
                    // choose a mole to raise, and go to Raising mode.
                    if let Some(mouse_pos) = self.clicked_at {
                        if mouse_pos.y < 100. && mouse_pos.x > 200. {
                            self.mode = Mode::Raising;
                            self.active_mole_column = rng.gen_range(0, N_COLUMNS);
                            self.active_mole_row = rng.gen_range(0, N_ROWS);
                            self.start_time = timer::time_since_start(ctx);
                            //println!(
                            //    "  R={} C={}.",
                            //    self.active_mole_row, self.active_mole_column
                            //);
                            let _ = self.appearance_sound.play();
                        }
                    }
                }
                Mode::Raising => {
                    if timer::time_since_start(ctx) - self.start_time >= GAME_DURATION {
                        self.mode = Mode::Finished;
                        let _ = self.finish_sound.play();
                    } else {
                        // Raise the active mole, without exceeding 1.
                        self.active_mole_position = (self.active_mole_position + 0.16).min(1.);

                        // If clicked on the active mole,
                        // go to Lowering mode.
                        if let Some(mouse_pos) = self.clicked_at {
                            //println!("mouse_pos={} {}.", mouse_pos.x, mouse_pos.y);
                            /*
                            let mole_position = Point2::new(
                                FIRST_COLUMN_X + self.active_mole_column as f32 * COLUMNS_STEP,
                                FIRST_ROW_Y + self.active_mole_row as f32 * ROWS_STEP
                                    - 0.2
                                        * self.active_mole_position
                                        * self.mole_image.height() as f32,
                            );
                            */
                            /*
                            if mouse_pos.x >= mole_position.x
                                && mouse_pos.x
                                    <= mole_position.x + 0.2 * self.mole_image.width() as f32
                                && mouse_pos.y >= mole_position.y
                                && mouse_pos.y
                                    <= mole_position.y
                                        + 0.2
                                            * self.active_mole_position
                                            * self.mole_image.height() as f32
                            */
                            if self.get_active_mole_bounding_box().contains(mouse_pos) {
                                self.mode = Mode::Lowering;
                                let _ = self.hit_sound.play();
                            } else {
                                let _ = self.miss_sound.play();
                            }
                        }
                    }
                }
                Mode::Lowering => {
                    // If completely lowered,
                    // choose a mole to raise, and go to Raising mode.
                    self.active_mole_position -= 0.22;
                    if self.active_mole_position <= 0. {
                        self.active_mole_position = 0.;
                        self.mode = Mode::Raising;
                        self.active_mole_column = rng.gen_range(0, N_COLUMNS);
                        self.active_mole_row = rng.gen_range(0, N_ROWS);
                        //println!("R={} C={}.", self.active_mole_row, self.active_mole_column);
                        let _ = self.appearance_sound.play();
                    }
                }
                Mode::Finished => {
                    if let Some(mouse_pos) = self.clicked_at {
                        if mouse_pos.y < 100. && mouse_pos.x < 200. {
                            *self = Screen::new(ctx).unwrap();
                        }
                    }
                }
            }
            self.clicked_at = None;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let area = graphics::drawable_size(ctx);

        // Draw the lawn.
        let lawn_params = graphics::DrawParam::new().scale(Vector2::new(
            area.0 / self.lawn_image.width() as f32,
            area.1 / self.lawn_image.height() as f32,
        ));
        graphics::draw(ctx, &self.lawn_image, lawn_params)?;

        let bounding_box = self.get_active_mole_bounding_box();

        // Draw the active mole.
        graphics::draw(
            ctx,
            &self.mole_image,
            graphics::DrawParam::new()
                .src(Rect::new(0., 0., 1., self.active_mole_position))
                .dest(Point2::new(bounding_box.left(), bounding_box.top()))
                .scale(Vector2::new(0.2, 0.2)),
        )?;

        // Check if the mouse is on the active mole.
        if bounding_box.contains(mouse::position(ctx)) {
            //mouse::set_cursor_type(ctx, mouse::MouseCursor::Hand);
            //mouse::set_cursor_type(ctx, mouse::MouseCursor::Crosshair);
            mouse::set_cursor_hidden(ctx, true);
            graphics::draw(
                ctx,
                &self.mallet_image,
                graphics::DrawParam::new()
                    .dest(mouse::position(ctx))
                    .scale(Vector2::new(0.2, 0.2)),
            )?;
        } else {
            mouse::set_cursor_hidden(ctx, false);
            //mouse::set_cursor_type(ctx, mouse::MouseCursor::Default);
        }
        //mouse::set_cursor_hidden(ctx, .y < 100.);
        //ggez::input::mouse::MouseCursor::Grab

        /*
                let elapsed_shown_text = format!(
                    "Remaining time: {:.2} s,\n\
                     Hit moles: {:.2} pixel/s,\n\
                     {}",
                    self.remaining_time.as_millis() as f32 / 1000.,
                    self.n_hit_moles,
                );

                let text = graphics::Text::new((elapsed_shown_text, self.font, 16.0));
                graphics::draw(ctx, &text, (Point2::new(4.0, 4.0), 0.0, graphics::BLACK))?;
        */
        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.clicked_at = Some(Point2::new(x, y));
        }
    }
}

pub fn main() -> GameResult {
    let (context, animation_loop) = &mut ContextBuilder::new("whac-a-mole", "ggez")
        .window_setup(conf::WindowSetup::default().title("Whac-a-Mole"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("static")
        .build()?;
    let game = &mut Screen::new(context)?;
    event::run(context, animation_loop, game)
}
