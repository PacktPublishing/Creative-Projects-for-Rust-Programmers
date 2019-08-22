use ggez::{
    audio,
    audio::SoundSource,
    conf,
    event::{self, EventHandler, MouseButton},
    graphics,
    graphics::{DrawParam, Font, Rect},
    input::mouse,
    timer, Context, ContextBuilder, GameResult,
};
use rand::prelude::*;
use std::rc::Rc;
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
const WIDGET_TOP_MARGIN: f32 = 8.;
const WIDGET_BOTTOM_MARGIN: f32 = 8.;
const WIDGET_LEFT_MARGIN: f32 = 10.;
const WIDGET_RIGHT_MARGIN: f32 = 10.;
const BUTTON_FONT_SIZE: f32 = 44.;
const BUTTON_PRESS_SHIFT: f32 = 4.;
const DESIRED_FPS: u32 = 20;
const MALLET_SCALE: f32 = 0.3;
const MOLE_SCALE: f32 = 0.3;

#[derive(Debug)]
enum Mode {
    Ready,
    Raising,
    Lowering,
}

struct Screen {
    mode: Mode,
    start_time: Option<Duration>,
    active_mole_column: usize,
    active_mole_row: usize,
    active_mole_position: f32,
    n_hit_moles: u32,
    random_generator: ThreadRng,
    mallet_image: graphics::Image,
    lawn_image: graphics::Image,
    mole_image: graphics::Image,
    font: graphics::Font,
    appearance_sound: audio::Source,
    hit_sound: audio::Source,
    miss_sound: audio::Source,
    finish_sound: audio::Source,
    mouse_down_at: Option<Point2>,
    mouse_up_at: Option<Point2>,
    start_button: Button,
}

impl Screen {
    fn new(ctx: &mut Context) -> GameResult<Screen> {
        let font = Font::new(ctx, "/font.ttf")?;
        let button_image = Rc::new(graphics::Image::new(ctx, "/button.png")?);
        Ok(Screen {
            mode: Mode::Ready,
            start_time: None,
            active_mole_column: 0,
            active_mole_row: 0,
            active_mole_position: 0.,
            n_hit_moles: 0,
            random_generator: thread_rng(),
            mallet_image: graphics::Image::new(ctx, "/mallet.png")?,
            lawn_image: graphics::Image::new(ctx, "/lawn.jpg")?,
            mole_image: graphics::Image::new(ctx, "/mole.png")?,
            font,
            appearance_sound: audio::Source::new(ctx, "/cry.ogg")?,
            hit_sound: audio::Source::new(ctx, "/click.ogg")?,
            miss_sound: audio::Source::new(ctx, "/bump.ogg")?,
            finish_sound: audio::Source::new(ctx, "/two_notes.ogg")?,
            mouse_down_at: None,
            mouse_up_at: None,
            start_button: Button::new(
                ctx,
                "Start",
                Point2::new(600., 40.),
                font,
                button_image.clone(),
            ),
        })
    }

    fn get_active_mole_bounding_box(&self) -> Rect {
        Rect::new(
            FIRST_COLUMN_X + self.active_mole_column as f32 * COLUMNS_STEP,
            FIRST_ROW_Y + self.active_mole_row as f32 * ROWS_STEP
                - MOLE_SCALE * self.active_mole_position * f32::from(self.mole_image.height()),
            MOLE_SCALE * f32::from(self.mole_image.height()),
            MOLE_SCALE * self.active_mole_position * f32::from(self.mole_image.height()),
        )
    }

    fn raise_another_mole(&mut self) {
        loop {
            let new_active_mole_column = self.random_generator.gen_range(0, N_COLUMNS);
            let new_active_mole_row = self.random_generator.gen_range(0, N_ROWS);
            if new_active_mole_column != self.active_mole_column
                || new_active_mole_row != self.active_mole_row
            {
                self.active_mole_column = new_active_mole_column;
                self.active_mole_row = new_active_mole_row;
                break;
            }
        }
        self.active_mole_position = 0.;
        self.mode = Mode::Raising;
        let _ = self.appearance_sound.play();
    }
}

struct Button {
    base_image: Rc<graphics::Image>,
    bounding_box: Rect,
    drawable_text: graphics::Text,
}

impl Button {
    fn new(
        ctx: &mut Context,
        caption: &str,
        center: Point2,
        font: Font,
        base_image: Rc<graphics::Image>,
    ) -> Self {
        let drawable_text = graphics::Text::new((caption, font, BUTTON_FONT_SIZE));
        let (width, height) = drawable_text.dimensions(ctx);
        let bounding_box = Rect::new(
            center.x - width as f32 * 0.5 - WIDGET_LEFT_MARGIN,
            center.y - height as f32 * 0.5 - WIDGET_TOP_MARGIN,
            width as f32 + WIDGET_LEFT_MARGIN + WIDGET_RIGHT_MARGIN,
            height as f32 + WIDGET_TOP_MARGIN + WIDGET_BOTTOM_MARGIN,
        );
        Button {
            base_image,
            bounding_box,
            drawable_text,
        }
    }

    fn contains(&self, pt: Point2) -> bool {
        self.bounding_box.contains(pt)
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut rect = self.bounding_box;
        let is_pressed = mouse::button_pressed(ctx, MouseButton::Left);
        let is_inside = rect.contains(mouse::position(ctx));

        if is_pressed && is_inside {
            rect.y += BUTTON_PRESS_SHIFT;
        }
        let mut area_draw_param = DrawParam::new().dest(rect.point()).scale(Vector2::new(
            rect.w / f32::from(self.base_image.width()),
            rect.h / f32::from(self.base_image.height()),
        ));
        if is_pressed && is_inside {
            area_draw_param =
                area_draw_param.src(Rect::new(0., 0., 1., 1. - BUTTON_PRESS_SHIFT / rect.h));
        }

        // Draw the empty button.
        graphics::draw(ctx, &*self.base_image, area_draw_param)?;

        // Draw the caption.
        graphics::draw(
            ctx,
            &self.drawable_text,
            DrawParam::new()
                .dest(Point2::new(
                    rect.left() + WIDGET_LEFT_MARGIN,
                    rect.top() + WIDGET_TOP_MARGIN,
                ))
                .color(if is_inside {
                    [0.8, 0., 0., 1.].into()
                } else {
                    graphics::BLACK
                }),
        )?;

        Ok(())
    }
}

impl EventHandler for Screen {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            match self.mode {
                Mode::Ready => {
                    // If clicked on the Start button,
                    // choose a mole to raise, and go to Raising mode.
                    if let Some(mouse_down_at) = self.mouse_down_at {
                        if let Some(mouse_up_at) = self.mouse_up_at {
                            if self.start_button.contains(mouse_down_at)
                                && self.start_button.contains(mouse_up_at)
                            {
                                self.mouse_down_at = None;
                                self.mouse_up_at = None;
                                self.start_time = Some(timer::time_since_start(ctx));
                                self.n_hit_moles = 0;
                                self.raise_another_mole();
                            }
                        }
                    }
                }
                Mode::Raising => {
                    if timer::time_since_start(ctx) - self.start_time.unwrap() >= GAME_DURATION {
                        self.mode = Mode::Ready;
                        self.active_mole_position = 0.;
                        self.mouse_down_at = None;
                        self.mouse_up_at = None;
                        let _ = self.finish_sound.play();
                    } else {
                        // Raise the active mole, without exceeding 1.
                        self.active_mole_position =
                            (self.active_mole_position + 2.4 / DESIRED_FPS as f32).min(1.);

                        // If clicked on the active mole,
                        // go to Lowering mode.
                        if let Some(mouse_pos) = self.mouse_down_at {
                            self.mouse_down_at = None;
                            if self.get_active_mole_bounding_box().contains(mouse_pos) {
                                self.mode = Mode::Lowering;
                                self.n_hit_moles += 1;
                                let _ = self.hit_sound.play();
                            } else {
                                let _ = self.miss_sound.play();
                            }
                        }
                    }
                }
                Mode::Lowering => {
                    self.mouse_down_at = None;
                    self.mouse_up_at = None;
                    // If completely lowered,
                    // choose a mole to raise, and go to Raising mode.
                    self.active_mole_position -= 3.6 / DESIRED_FPS as f32;
                    if self.active_mole_position <= 0. {
                        self.raise_another_mole();
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let area = graphics::drawable_size(ctx);

        // Draw the lawn.
        let lawn_params = DrawParam::new().scale(Vector2::new(
            area.0 / f32::from(self.lawn_image.width()),
            area.1 / f32::from(self.lawn_image.height()),
        ));
        graphics::draw(ctx, &self.lawn_image, lawn_params)?;

        if let Mode::Ready = self.mode {
            self.start_button.draw(ctx)?;
        }

        // Draw the active mole.
        let bounding_box = self.get_active_mole_bounding_box();
        graphics::draw(
            ctx,
            &self.mole_image,
            DrawParam::new()
                .src(Rect::new(0., 0., 1., self.active_mole_position))
                .dest(Point2::new(bounding_box.left(), bounding_box.top()))
                .scale(Vector2::new(MOLE_SCALE, MOLE_SCALE)),
        )?;

        if let Mode::Ready = self.mode {
            mouse::set_cursor_type(ctx, mouse::MouseCursor::Default);
        }
        // Check if the mouse is on the active mole.
        else if bounding_box.contains(mouse::position(ctx)) {
            mouse::set_cursor_type(ctx, mouse::MouseCursor::Crosshair);
            let angle_degrees = match self.mode {
                Mode::Lowering => 135. - 55. * self.active_mole_position,
                _ => 80.,
            };
            graphics::draw(
                ctx,
                &self.mallet_image,
                DrawParam::new()
                    .dest(
                        Point2::from(mouse::position(ctx))
                            + Vector2::new(f32::from(self.mallet_image.width()) * MALLET_SCALE, 0.),
                    )
                    .scale(Vector2::new(MALLET_SCALE, MALLET_SCALE))
                    .offset(Point2::new(0., 1.))
                    .rotation(angle_degrees / -180. * std::f32::consts::PI),
            )?;
        } else {
            //mouse::set_cursor_hidden(ctx, false);
            mouse::set_cursor_type(ctx, mouse::MouseCursor::NotAllowed);
        }

        let time_text = if let Some(start_time) = self.start_time {
            let elapsed_time = timer::time_since_start(ctx) - start_time;
            if elapsed_time < GAME_DURATION {
                format!(
                    "Remaining time: {} seconds",
                    (GAME_DURATION - elapsed_time).as_secs()
                )
            } else {
                "Game finished. Click on Start to play again.".to_string()
            }
        } else {
            "Click on Start to play.".to_string()
        };
        let text = format!(
            "{}\n\
             Hit moles: {}",
            time_text, self.n_hit_moles
        );
        let drawable_text = graphics::Text::new((text, self.font, 24.0));
        graphics::draw(
            ctx,
            &drawable_text,
            DrawParam::new()
                .dest(Point2::new(4.0, 4.0))
                .color(graphics::BLACK),
        )?;
        graphics::draw(
            ctx,
            &drawable_text,
            DrawParam::new()
                .dest(Point2::new(2.0, 2.0))
                .color(graphics::WHITE),
        )?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.mouse_down_at = Some(Point2::new(x, y));
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.mouse_up_at = Some(Point2::new(x, y));
        }
    }
}

fn main() -> GameResult {
    let (context, animation_loop) = &mut ContextBuilder::new("whac-a-mole", "ggez")
        .window_setup(conf::WindowSetup::default().title("Whac-a-Mole"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("assets")
        .build()?;
    let game = &mut Screen::new(context)?;
    event::run(context, animation_loop, game)
}
