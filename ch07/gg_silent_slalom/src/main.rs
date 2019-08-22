use ggez::{
    conf,
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::{DrawMode, Rect},
    timer, Context, ContextBuilder, GameResult,
};
use rand::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

type Point2 = nalgebra::Point2<f32>;

const SCREEN_WIDTH: f32 = 800.; // in pixels
const SCREEN_HEIGHT: f32 = 600.; // in pixels
const SKI_WIDTH: f32 = 10.; // in pixels
const SKI_LENGTH: f32 = 50.; // in pixels
const SKI_TIP_LEN: f32 = 20.; // in pixels
const N_GATES_IN_SCREEN: usize = 3;
const GATE_POLE_RADIUS: f32 = 4.; // in pixels
const GATE_WIDTH: f32 = 150.; // in pixels
const STEERING_SPEED: f32 = 110. / 180. * PI; // in radians/second
const MAX_ANGLE: f32 = 75. / 180. * PI; // in radians
const SKI_MARGIN: f32 = 12.; // in pixels
const ALONG_ACCELERATION: f32 = 20.; // in pixels/second/second
const DRAG_FACTOR: f32 = 0.88; // in fraction-of-speed/second
const TOTAL_N_GATES: usize = 8;

#[derive(Debug)]
enum Mode {
    Ready,
    Running,
    Finished,
    Failed,
}

#[derive(Debug)]
struct InputState {
    to_turn: f32,
    started: bool,
}

struct Screen {
    gates: Vec<(f32, f32)>,
    ski_across_offset: f32,  // in pixels
    direction: f32,          // in radians
    forward_speed: f32,      // in pixels per second
    gates_along_offset: f32, // in pixels
    previous_frame_time: Duration,
    period_in_sec: f32, // in seconds
    mode: Mode,
    entered_gate: bool,
    disappeared_gates: usize,
    input: InputState,
}

impl Screen {
    fn new(_ctx: &mut Context) -> GameResult<Screen> {
        let mut gates = Vec::new();
        for i in 0..TOTAL_N_GATES {
            gates.push(Self::get_random_gate(i % 2 == 0));
        }
        let s = Screen {
            gates,
            ski_across_offset: 0.,
            direction: 0.,
            forward_speed: 0.,
            gates_along_offset: 0.,
            previous_frame_time: Duration::from_secs(0),
            period_in_sec: 0.,
            mode: Mode::Ready,
            entered_gate: false,
            disappeared_gates: 0,
            input: InputState {
                to_turn: 0.0,
                started: false,
            },
        };
        Ok(s)
    }

    fn get_random_gate(gate_is_at_right: bool) -> (f32, f32) {
        let mut rng = thread_rng();
        let pole_pos = rng.gen_range(-GATE_WIDTH / 2., SCREEN_WIDTH / 2. - GATE_WIDTH * 1.5);
        if gate_is_at_right {
            (pole_pos, pole_pos + GATE_WIDTH)
        } else {
            (-pole_pos - GATE_WIDTH, -pole_pos)
        }
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
            match self.mode {
                Mode::Ready => {
                    if self.input.started {
                        self.mode = Mode::Running;
                    }
                }
                Mode::Running => {
                    self.forward_speed = (self.forward_speed
                        + ALONG_ACCELERATION * self.period_in_sec * self.direction.cos())
                        * DRAG_FACTOR.powf(self.period_in_sec);
                    let along_speed = self.forward_speed * self.direction.cos();
                    self.ski_across_offset +=
                        self.forward_speed * self.period_in_sec * self.direction.sin();
                    if self.ski_across_offset < -SCREEN_WIDTH / 2. + SKI_MARGIN {
                        self.ski_across_offset = -SCREEN_WIDTH / 2. + SKI_MARGIN;
                    }
                    if self.ski_across_offset > SCREEN_WIDTH / 2. - SKI_MARGIN {
                        self.ski_across_offset = SCREEN_WIDTH / 2. - SKI_MARGIN;
                    }
                    self.gates_along_offset += along_speed * self.period_in_sec;
                    let max_gates_along_offset = SCREEN_HEIGHT / N_GATES_IN_SCREEN as f32;
                    if self.gates_along_offset > max_gates_along_offset {
                        self.gates_along_offset -= max_gates_along_offset;
                        self.disappeared_gates += 1;
                    }

                    // If the ski tip is over a gate, and before it wasn't,
                    // check whether it is within the gate.
                    let ski_tip_along = SCREEN_HEIGHT * 15. / 16. - SKI_LENGTH / 2. - SKI_TIP_LEN;

                    let ski_tip_across = SCREEN_WIDTH / 2. + self.ski_across_offset;

                    let n_next_gate = self.disappeared_gates;
                    let next_gate = &self.gates[n_next_gate];
                    let left_pole_offset = SCREEN_WIDTH / 2. + next_gate.0 + GATE_POLE_RADIUS;
                    let right_pole_offset = SCREEN_WIDTH / 2. + next_gate.1 - GATE_POLE_RADIUS;
                    let next_gate_along = self.gates_along_offset + SCREEN_HEIGHT
                        - SCREEN_HEIGHT / N_GATES_IN_SCREEN as f32;

                    if ski_tip_along <= next_gate_along {
                        if !self.entered_gate {
                            if ski_tip_across < left_pole_offset
                                || ski_tip_across > right_pole_offset
                            {
                                self.mode = Mode::Failed;
                            } else if self.disappeared_gates == TOTAL_N_GATES - 1 {
                                self.mode = Mode::Finished;
                            }
                            self.entered_gate = true;
                        }
                    } else {
                        self.entered_gate = false;
                    }
                }
                Mode::Failed | Mode::Finished => {
                    self.forward_speed = 0.;
                    if !self.input.started {
                        *self = Screen::new(ctx).unwrap();
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let normal_pole = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(0., 0.),
            GATE_POLE_RADIUS,
            0.05,
            [0., 0., 1., 1.].into(),
        )?;
        let finish_pole = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(0., 0.),
            GATE_POLE_RADIUS,
            0.05,
            [0., 1., 0., 1.].into(),
        )?;

        for i_gate in self.disappeared_gates..self.disappeared_gates + N_GATES_IN_SCREEN {
            if i_gate >= TOTAL_N_GATES {
                break;
            }
            let gate = self.gates[i_gate];
            let pole = if i_gate == TOTAL_N_GATES - 1 {
                &finish_pole
            } else {
                &normal_pole
            };
            let gates_along_pos = self.gates_along_offset
                + SCREEN_HEIGHT / N_GATES_IN_SCREEN as f32
                    * (self.disappeared_gates + N_GATES_IN_SCREEN - 1 - i_gate) as f32;
            graphics::draw(
                ctx,
                pole,
                (Point2::new(SCREEN_WIDTH / 2. + gate.0, gates_along_pos),),
            )?;
            graphics::draw(
                ctx,
                pole,
                (Point2::new(SCREEN_WIDTH / 2. + gate.1, gates_along_pos),),
            )?;
        }
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
            KeyCode::Space => {
                self.input.started = true;
            }
            KeyCode::R => {
                self.input.started = false;
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
