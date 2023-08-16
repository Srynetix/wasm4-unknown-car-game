use wasm4_sx::{
    wasm4::{rect, SCREEN_SIZE},
    *,
};

const ACCELERATION: f32 = 50.0;
const MOVEMENT_SPEED: f32 = 150.0;

const CAR_WIDTH: u32 = 6;
const CAR_HEIGHT: u32 = 8;
const CAR_WHEEL_WIDTH: u32 = 2;
const CAR_WHEEL_HEIGHT: u32 = 2;
const CAR_MIN_SPEED: f32 = 5.0;
const CAR_MAX_SPEED: f32 = 300.0;

const BACKGROUND_TILE_WIDTH: u32 = 8;
const BACKGROUND_TILE_HEIGHT: u32 = 8;
const BACKGROUND_PADDING: u32 = BACKGROUND_TILE_WIDTH * 2;

struct Timer {
    wait_time: f32,
    elapsed_time: f32,
    triggered: bool,
}

impl Timer {
    pub const fn new(wait_time: f32) -> Self {
        Self {
            wait_time,
            elapsed_time: 0.0,
            triggered: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.triggered {
            return;
        }

        self.elapsed_time += dt;
        if self.elapsed_time >= self.wait_time {
            self.triggered = true;
        }
    }

    pub fn remaining_time(&self) -> f32 {
        self.wait_time - self.elapsed_time
    }
}

#[derive(Copy, Clone)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    pub fn draw(&self) {
        rect(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }

    pub fn is_colliding_with(&self, other: Rect) -> bool {
        self.right() >= other.x
            && self.x <= other.right()
            && self.bottom() >= other.y
            && self.y <= other.bottom()
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }
}

pub struct Game {
    car: Car,
    background: Background,
    obstacles: [Option<Obstacle>; 16],
    score: u32,
    game_timer: Timer,
}

pub static GAME: W4RefCell<Game> = W4RefCell::new(Game::new());

#[derive(Clone, Copy)]
struct Car {
    x: f32,
    y: f32,
    speed: f32,
}

struct Background {
    y: f32,
}

#[derive(Clone, Copy)]
struct Obstacle {
    x: f32,
    y: f32,
    velocity: Vec2,
    destroyed: bool,
}

impl Obstacle {
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity: Vec2::ZERO,
            destroyed: false,
        }
    }

    fn draw(&self) {
        self.get_rect().draw();
    }

    fn is_out_of_bounds(&self) -> bool {
        self.y as u32 > SCREEN_SIZE
            || self.y < -(SCREEN_SIZE as f32 / 4.0)
            || self.x < 0.0
            || self.x > SCREEN_SIZE as f32
    }

    fn get_rect(&self) -> Rect {
        Rect {
            x: self.x - 5.0 / 2.0,
            y: self.y - 5.0 / 2.0,
            w: 5.0,
            h: 5.0,
        }
    }
}

impl Background {
    pub const fn new() -> Self {
        Self { y: 0.0 }
    }

    fn process(&mut self, dt: f32, speed: f32) {
        self.y += speed * dt;

        if self.y > BACKGROUND_TILE_HEIGHT as f32 * 2.0 {
            self.y -= BACKGROUND_TILE_HEIGHT as f32 * 2.0;
        }

        self.draw();
    }

    fn draw(&self) {
        let offset = self.y;

        for x in [BACKGROUND_PADDING, SCREEN_SIZE - BACKGROUND_PADDING] {
            for y in -2i32..(SCREEN_SIZE / BACKGROUND_TILE_HEIGHT + 2) as i32 {
                if y % 2 == 0 {
                    Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P2);
                } else {
                    Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P3);
                }

                rect(
                    x as i32 - (BACKGROUND_TILE_WIDTH / 2) as i32,
                    (y as f32 * BACKGROUND_TILE_HEIGHT as f32 + offset) as i32,
                    BACKGROUND_TILE_WIDTH,
                    BACKGROUND_TILE_HEIGHT,
                );
            }
        }
    }

    pub fn is_colliding_with(&self, rect: Rect) -> bool {
        let left_rect = Rect {
            x: (BACKGROUND_PADDING - BACKGROUND_TILE_WIDTH / 2) as f32,
            y: 0.0,
            w: BACKGROUND_TILE_WIDTH as f32,
            h: SCREEN_SIZE as f32,
        };

        let right_rect = Rect {
            x: (SCREEN_SIZE - BACKGROUND_PADDING - BACKGROUND_TILE_WIDTH / 2) as f32,
            y: 0.0,
            w: BACKGROUND_TILE_WIDTH as f32,
            h: SCREEN_SIZE as f32,
        };

        rect.is_colliding_with(left_rect) || rect.is_colliding_with(right_rect)
    }
}

impl Car {
    pub const fn new() -> Self {
        Self {
            x: (SCREEN_SIZE / 2) as f32,
            y: (SCREEN_SIZE - 10) as f32,
            speed: CAR_MIN_SPEED,
        }
    }

    fn draw(&self) {
        self.draw_wheels();

        Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P3);
        rect(
            (self.x - (CAR_WIDTH as f32 / 2.0)) as i32,
            (self.y - (CAR_HEIGHT as f32 / 2.0)) as i32,
            CAR_WIDTH,
            CAR_HEIGHT,
        );

        self.draw_decorations();
    }

    fn draw_wheels(&self) {
        Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P2);

        // Top left
        rect(
            (self.x - (CAR_WIDTH as f32 / 2.0 + CAR_WHEEL_WIDTH as f32 / 2.0)) as i32,
            (self.y - (CAR_HEIGHT as f32 / 2.0) + (CAR_WHEEL_HEIGHT as f32 / 2.0)) as i32,
            CAR_WHEEL_WIDTH,
            CAR_WHEEL_HEIGHT,
        );

        // Top right
        rect(
            (self.x + (CAR_WIDTH as f32 / 2.0 - CAR_WHEEL_WIDTH as f32 / 2.0)) as i32,
            (self.y - (CAR_HEIGHT as f32 / 2.0) + (CAR_WHEEL_HEIGHT as f32 / 2.0)) as i32,
            CAR_WHEEL_WIDTH,
            CAR_WHEEL_HEIGHT,
        );

        // Bottom left
        rect(
            (self.x - (CAR_WIDTH as f32 / 2.0 + CAR_WHEEL_WIDTH as f32 / 2.0)) as i32,
            (self.y + (CAR_HEIGHT as f32 / 8.0)) as i32,
            CAR_WHEEL_WIDTH,
            CAR_WHEEL_HEIGHT,
        );

        // Bottom right
        rect(
            (self.x + (CAR_WIDTH as f32 / 2.0 - CAR_WHEEL_WIDTH as f32 / 2.0)) as i32,
            (self.y + (CAR_HEIGHT as f32 / 8.0)) as i32,
            CAR_WHEEL_WIDTH,
            CAR_WHEEL_HEIGHT,
        );
    }

    fn draw_decorations(&self) {
        Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P2);
        rect(
            (self.x - (CAR_WIDTH as f32 / 2.0)) as i32 + 1,
            (self.y - (CAR_HEIGHT as f32 / 2.0)) as i32 + 1,
            CAR_WIDTH - 2,
            CAR_HEIGHT / 4,
        )
    }

    fn get_rect(&self) -> Rect {
        Rect {
            x: self.x - CAR_WIDTH as f32 / 2.0,
            y: self.y - CAR_HEIGHT as f32 / 2.0,
            w: CAR_WIDTH as f32,
            h: CAR_HEIGHT as f32,
        }
    }
}

impl Game {
    const fn new() -> Game {
        Game {
            car: Car::new(),
            obstacles: [None; 16],
            background: Background::new(),
            score: 0,
            game_timer: Timer::new(30.0),
        }
    }

    fn spawn_obstacle(&mut self) {
        // First empty obstacle
        let pos = self.obstacles.iter().position(|o| o.is_none());
        if let Some(pos) = pos {
            // Spawn!
            let obstacle = Obstacle::new(
                rand_u8(BACKGROUND_PADDING as u8..(SCREEN_SIZE - BACKGROUND_PADDING) as u8) as f32,
                0.0,
            );
            self.obstacles[pos] = Some(obstacle);
        }
    }

    fn play_hit_sound() {
        Tone::builder()
            .with_duration(Adsr::new(0, 2, 2, 1))
            .with_frequency(FrequencySlide::new(220))
            .with_volume(Volume::new(100))
            .with_flags(ToneFlags::builder().with_channel(Channel::Noise).build())
            .play();
    }

    fn process_obstacles(&mut self, dt: f32) {
        for obs in self.obstacles.iter_mut().filter(|x| x.is_some()) {
            let obs_obj = obs.as_mut().unwrap();

            if obs_obj.destroyed {
                obs_obj.y -= self.car.speed * dt;
            } else {
                obs_obj.y += self.car.speed * dt;
            }

            // Background collision
            if obs_obj.destroyed && self.background.is_colliding_with(obs_obj.get_rect()) {
                Self::play_hit_sound();
                obs_obj.velocity.x *= -1.0;
            }

            obs_obj.x += obs_obj.velocity.x * dt;
            obs_obj.draw();

            if !obs_obj.destroyed && obs_obj.get_rect().is_colliding_with(self.car.get_rect()) {
                self.score += 100;
                obs_obj.velocity.x = rand_i32(-200..200) as f32;
                obs_obj.destroyed = true;

                Self::play_hit_sound();
            } else if obs_obj.is_out_of_bounds() {
                // Cleanup
                *obs = None;
            }
        }
    }

    fn show_score(&self) {
        let value = format_w4!("Score: {}", self.score);

        Engine::draw_colors().set_index(DrawColorsIndex::I1, PaletteColor::P4);
        Text::new(value.as_bytes())
            .with_horizontal_alignment(TextHorizontalAlignment::Center)
            .with_vertical_alignment(TextVerticalAligment::Top)
            .with_padding_y(10)
            .draw();
    }

    fn show_timer(&self) {
        let time = format_w4!("{0:.0}", self.game_timer.remaining_time());
        Text::new(time.as_bytes())
            .with_horizontal_alignment(TextHorizontalAlignment::Center)
            .with_vertical_alignment(TextVerticalAligment::Middle)
            .draw();
    }

    fn show_instructions(&self) {
        Text::new(b"\x86 to accelerate\n\x87 to brake\n\n\nHit the obstacles!\n")
            .with_horizontal_alignment(TextHorizontalAlignment::Center)
            .with_vertical_alignment(TextVerticalAligment::Bottom)
            .with_line_separation(2)
            .draw();
    }

    fn process_car(&mut self, ctx: &FrameContext) {
        self.handle_input(ctx);
        self.car.draw();
    }

    fn process_background(&mut self, dt: f32) {
        self.background.process(dt, self.car.speed);
    }

    /// Run game for a frame.
    pub fn run_game_frame(&mut self, ctx: &FrameContext) {
        if !self.game_timer.triggered {
            let spawn_frequency = ((1.0 - (self.car.speed / CAR_MAX_SPEED)) * 100.0 + 5.0) as u64;
            if Engine::frame_count() % spawn_frequency == 0 {
                self.spawn_obstacle();
            }

            self.process_background(ctx.delta_time());
            self.process_car(ctx);
            self.process_obstacles(ctx.delta_time());

            self.game_timer.update(ctx.delta_time());

            self.show_score();
            self.show_timer();
        } else {
            Text::new("Game Over")
                .with_horizontal_alignment(TextHorizontalAlignment::Center)
                .with_vertical_alignment(TextVerticalAligment::Middle)
                .draw();

            self.show_score();
        }

        if Engine::frame_count() < Engine::FPS * 5 {
            self.show_instructions();
        }
    }

    fn handle_input(&mut self, ctx: &FrameContext) {
        let mut movement = 0.0;
        let mut speed = 0.0;

        if ctx
            .gamepad(GamepadIndex::I1)
            .is_button_pressed(GamepadButton::Left)
        {
            movement -= MOVEMENT_SPEED * ctx.delta_time();
        } else if ctx
            .gamepad(GamepadIndex::I1)
            .is_button_pressed(GamepadButton::Right)
        {
            movement += MOVEMENT_SPEED * ctx.delta_time();
        }

        if ctx
            .gamepad(GamepadIndex::I1)
            .is_button_pressed(GamepadButton::Up)
        {
            speed += ACCELERATION * ctx.delta_time();
        } else if ctx
            .gamepad(GamepadIndex::I1)
            .is_button_pressed(GamepadButton::Down)
        {
            speed -= ACCELERATION * ctx.delta_time();
        }

        self.car.x += movement;
        self.car.speed = (self.car.speed + speed).clamp(CAR_MIN_SPEED, CAR_MAX_SPEED);
    }
}
