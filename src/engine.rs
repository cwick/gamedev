use rand::Rng;

const PADDLE_SPEED: f32 = 300.0;
const MAX_DT: f32 = 0.05;
const STATE_FIELDS: usize = 10;
const AI_DEAD_ZONE: f32 = 10.0;

const INPUT_UP: u32 = 0b01;
const INPUT_DOWN: u32 = 0b10;

const BALL_RADIUS: f32 = 8.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE1_X: f32 = 20.0;
const PADDLE2_X: f32 = 770.0;

struct Ball {
    position_x: f32,
    position_y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

struct Paddle {
    position_y: f32,
    velocity_y: f32,
}

pub struct GameState {
    ball: Ball,
    paddles: [Paddle; 2],
    score_left: f32,
    score_right: f32,
    field_width: f32,
    field_height: f32,
    snapshot: [f32; STATE_FIELDS],
}

impl GameState {
    pub const fn new() -> Self {
        Self {
            ball: Ball {
                position_x: 0.0,
                position_y: 0.0,
                velocity_x: 0.0,
                velocity_y: 0.0,
            },
            paddles: [
                Paddle { position_y: 0.0, velocity_y: 0.0 },
                Paddle { position_y: 0.0, velocity_y: 0.0 },
            ],
            score_left: 0.0,
            score_right: 0.0,
            field_width: 0.0,
            field_height: 0.0,
            snapshot: [0.0; STATE_FIELDS],
        }
    }

    pub fn init(&mut self, width: f32, height: f32) {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(350.0..500.0);

        self.field_width = width;
        self.field_height = height;
        self.ball.position_x = width / 2.0;
        self.ball.position_y = height / 2.0;
        self.ball.velocity_x = angle.cos() * speed;
        self.ball.velocity_y = angle.sin() * speed;
        self.paddles[0].position_y = height / 2.0;
        self.paddles[0].velocity_y = 0.0;
        self.paddles[1].position_y = height / 2.0;
        self.paddles[1].velocity_y = 0.0;
        self.score_left = 0.0;
        self.score_right = 0.0;
    }

    pub fn step(&mut self, dt_seconds: f32, p1_input: u32) {
        let dt = dt_seconds.min(MAX_DT);
        let p2_input = self.compute_ai_input();
        self.apply_input(p1_input, p2_input);
        self.move_entities(dt);
        self.collide_paddles();
        self.collide_walls();
    }

    fn apply_input(&mut self, p1_input: u32, p2_input: u32) {
        for (i, &input_bits) in [p1_input, p2_input].iter().enumerate() {
            let up = (input_bits & INPUT_UP) != 0;
            let down = (input_bits & INPUT_DOWN) != 0;

            let dir = (down as i32) - (up as i32);
            self.paddles[i].velocity_y = dir as f32 * PADDLE_SPEED;
        }
    }

    fn compute_ai_input(&self) -> u32 {
        let ball_y = self.ball.position_y;
        let paddle_y = self.paddles[1].position_y;
        let diff = ball_y - paddle_y;

        if diff.abs() < AI_DEAD_ZONE {
            0
        } else if diff > 0.0 {
            INPUT_DOWN
        } else {
            INPUT_UP
        }
    }

    fn move_entities(&mut self, dt: f32) {
        self.ball.position_x += self.ball.velocity_x * dt;
        self.ball.position_y += self.ball.velocity_y * dt;

        let paddle_half_height = PADDLE_HEIGHT / 2.0;
        for paddle in &mut self.paddles {
            paddle.position_y += paddle.velocity_y * dt;
            paddle.position_y = paddle.position_y.clamp(paddle_half_height, self.field_height - paddle_half_height);
        }
    }

    fn collide_paddles(&mut self) {
        let paddle_positions = [PADDLE1_X, PADDLE2_X];

        for (i, &paddle_x) in paddle_positions.iter().enumerate() {
            if Self::check_paddle_collision(self.ball.position_x, self.ball.position_y, paddle_x, self.paddles[i].position_y) {
                if i == 0 {
                    self.ball.velocity_x = self.ball.velocity_x.abs();
                    self.ball.position_x = paddle_x + (PADDLE_WIDTH / 2.0) + BALL_RADIUS;
                } else {
                    self.ball.velocity_x = -self.ball.velocity_x.abs();
                    self.ball.position_x = paddle_x - (PADDLE_WIDTH / 2.0) - BALL_RADIUS;
                }
            }
        }
    }

    fn collide_walls(&mut self) {
        if self.ball.position_x <= 0.0 || self.ball.position_x >= self.field_width {
            self.ball.velocity_x = -self.ball.velocity_x;
            self.ball.position_x = self.ball.position_x.clamp(0.0, self.field_width);
        }

        if self.ball.position_y <= 0.0 || self.ball.position_y >= self.field_height {
            self.ball.velocity_y = -self.ball.velocity_y;
            self.ball.position_y = self.ball.position_y.clamp(0.0, self.field_height);
        }
    }

    fn check_paddle_collision(
        ball_x: f32,
        ball_y: f32,
        paddle_x: f32,
        paddle_y: f32,
    ) -> bool {
        let paddle_half_h = PADDLE_HEIGHT / 2.0;
        let paddle_half_w = PADDLE_WIDTH / 2.0;

        let closest_x = ball_x.clamp(
            paddle_x - paddle_half_w,
            paddle_x + paddle_half_w
        );
        let closest_y = ball_y.clamp(
            paddle_y - paddle_half_h,
            paddle_y + paddle_half_h
        );

        let dist_x = ball_x - closest_x;
        let dist_y = ball_y - closest_y;
        let dist_squared = dist_x * dist_x + dist_y * dist_y;

        dist_squared < (BALL_RADIUS * BALL_RADIUS)
    }

    pub fn update_snapshot(&mut self) {
        self.snapshot[0] = self.ball.position_x;
        self.snapshot[1] = self.ball.position_y;
        self.snapshot[2] = self.ball.velocity_x;
        self.snapshot[3] = self.ball.velocity_y;
        self.snapshot[4] = self.paddles[0].position_y;
        self.snapshot[5] = self.paddles[1].position_y;
        self.snapshot[6] = self.score_left;
        self.snapshot[7] = self.score_right;
        self.snapshot[8] = self.field_width;
        self.snapshot[9] = self.field_height;
    }

    pub fn snapshot_ptr(&self) -> *const f32 {
        self.snapshot.as_ptr()
    }

    pub fn snapshot_len(&self) -> usize {
        STATE_FIELDS
    }
}
