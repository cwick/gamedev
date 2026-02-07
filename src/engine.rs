use rand::Rng;

const PADDLE_SPEED: f32 = 300.0;
const MAX_DT: f32 = 0.05;
const STATE_FIELDS: usize = 10;

pub struct GameState {
    ball_x: f32,
    ball_y: f32,
    ball_vx: f32,
    ball_vy: f32,
    paddle1_y: f32,
    paddle2_y: f32,
    score1: f32,
    score2: f32,
    field_w: f32,
    field_h: f32,
    snapshot: [f32; STATE_FIELDS],
}

impl GameState {
    pub const fn new() -> Self {
        Self {
            ball_x: 0.0,
            ball_y: 0.0,
            ball_vx: 0.0,
            ball_vy: 0.0,
            paddle1_y: 0.0,
            paddle2_y: 0.0,
            score1: 0.0,
            score2: 0.0,
            field_w: 0.0,
            field_h: 0.0,
            snapshot: [0.0; STATE_FIELDS],
        }
    }

    pub fn init(&mut self, width: f32, height: f32) {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(150.0..250.0);

        self.field_w = width;
        self.field_h = height;
        self.ball_x = width / 2.0;
        self.ball_y = height / 2.0;
        self.ball_vx = angle.cos() * speed;
        self.ball_vy = angle.sin() * speed;
        self.paddle1_y = height / 2.0;
        self.paddle2_y = height / 2.0;
        self.score1 = 0.0;
        self.score2 = 0.0;
    }

    pub fn step(&mut self, dt_seconds: f32, input_bits: u32) {
        let dt = dt_seconds.min(MAX_DT);

        let p1_up = (input_bits & 0b01) != 0;
        let p1_down = (input_bits & 0b10) != 0;

        let dir = (p1_down as i32) - (p1_up as i32);
        self.paddle1_y += dir as f32 * PADDLE_SPEED * dt;

        let paddle_half_h = 40.0;
        self.paddle1_y = self.paddle1_y.clamp(paddle_half_h, self.field_h - paddle_half_h);

        self.ball_x += self.ball_vx * dt;
        self.ball_y += self.ball_vy * dt;

        if self.ball_x <= 0.0 || self.ball_x >= self.field_w {
            self.ball_vx = -self.ball_vx;
            self.ball_x = self.ball_x.clamp(0.0, self.field_w);
        }

        if self.ball_y <= 0.0 || self.ball_y >= self.field_h {
            self.ball_vy = -self.ball_vy;
            self.ball_y = self.ball_y.clamp(0.0, self.field_h);
        }
    }

    pub fn update_snapshot(&mut self) {
        self.snapshot[0] = self.ball_x;
        self.snapshot[1] = self.ball_y;
        self.snapshot[2] = self.ball_vx;
        self.snapshot[3] = self.ball_vy;
        self.snapshot[4] = self.paddle1_y;
        self.snapshot[5] = self.paddle2_y;
        self.snapshot[6] = self.score1;
        self.snapshot[7] = self.score2;
        self.snapshot[8] = self.field_w;
        self.snapshot[9] = self.field_h;
    }

    pub fn snapshot_ptr(&self) -> *const f32 {
        self.snapshot.as_ptr()
    }

    pub fn snapshot_len(&self) -> usize {
        STATE_FIELDS
    }
}
