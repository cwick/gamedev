use rand::Rng;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SnapshotField {
    BallX = 0,
    BallY = 1,
    BallVx = 2,
    BallVy = 3,
    Paddle1X = 4,
    Paddle1Y = 5,
    Paddle2X = 6,
    Paddle2Y = 7,
    PlayerOneScore = 8,
    PlayerTwoScore = 9,
    FieldWidth = 10,
    FieldHeight = 11,
    GamePhase = 12,
    Winner = 13,
    BallVisible = 14,
    PaddleWidth = 15,
    PaddleHeight = 16,
    Count = 17,
}

impl SnapshotField {
    const fn idx(self) -> usize {
        self as usize
    }
}

const PADDLE_SPEED: f32 = 300.0;
const MAX_DT: f32 = 0.05;
const STATE_FIELDS: usize = SnapshotField::Count as usize;
const AI_DEAD_ZONE: f32 = 10.0;
const DEFAULT_WINNING_SCORE: u32 = 11;
const SERVE_DELAY_MIN: f32 = 1.0;
const SERVE_DELAY_MAX: f32 = 3.0;

const INPUT_UP: u32 = 0b001;
const INPUT_DOWN: u32 = 0b010;
const INPUT_ACTION: u32 = 0b100;

const BALL_RADIUS: f32 = 8.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 60.0;
const PADDLE1_X: f32 = 20.0;
const PADDLE2_X: f32 = 770.0;
const MAX_BOUNCE_ANGLE: f32 = std::f32::consts::FRAC_PI_3;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GamePhase {
    Playing,
    GameOver,
}

impl GamePhase {
    const fn as_snapshot_value(self) -> f32 {
        match self {
            GamePhase::Playing => 0.0,
            GamePhase::GameOver => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    One,
    Two,
}

impl Player {
    const fn as_snapshot_value(self) -> f32 {
        match self {
            Player::One => 1.0,
            Player::Two => 2.0,
        }
    }
}

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
    field_width: f32,
    field_height: f32,
    player_one_score: u32,
    player_two_score: u32,
    phase: GamePhase,
    winner: Option<Player>,
    conceded_by: Option<Player>,
    winning_score: u32,
    serve_delay_remaining: f32,
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
                Paddle {
                    position_y: 0.0,
                    velocity_y: 0.0,
                },
                Paddle {
                    position_y: 0.0,
                    velocity_y: 0.0,
                },
            ],
            field_width: 0.0,
            field_height: 0.0,
            player_one_score: 0,
            player_two_score: 0,
            phase: GamePhase::Playing,
            winner: None,
            conceded_by: None,
            winning_score: DEFAULT_WINNING_SCORE,
            serve_delay_remaining: 0.0,
            snapshot: [0.0; STATE_FIELDS],
        }
    }

    pub fn init(&mut self, width: f32, height: f32) {
        self.field_width = width;
        self.field_height = height;
        self.player_one_score = 0;
        self.player_two_score = 0;
        self.phase = GamePhase::Playing;
        self.winner = None;
        self.winning_score = DEFAULT_WINNING_SCORE;
        self.conceded_by = None;
        self.serve_delay_remaining = 0.0;
        self.launch_ball();
        self.paddles[0].position_y = height / 2.0;
        self.paddles[0].velocity_y = 0.0;
        self.paddles[1].position_y = height / 2.0;
        self.paddles[1].velocity_y = 0.0;
    }

    fn center_ball(&mut self, side: f32) {
        self.ball.position_x = self.field_width / 2.0 + self.field_width * 0.1 * side;
        self.ball.position_y = self.field_height / 2.0;
        self.ball.velocity_x = 0.0;
        self.ball.velocity_y = 0.0;
    }

    fn launch_ball(&mut self) {
        let mut rng = rand::thread_rng();
        let half_cone = std::f32::consts::FRAC_PI_4;
        let spread = rng.gen_range(-half_cone..half_cone);
        let launch_side = match self.conceded_by.take() {
            Some(Player::One) => 1.0,
            Some(Player::Two) => -1.0,
            None => {
                if rng.gen_bool(0.5) {
                    1.0
                } else {
                    -1.0
                }
            }
        };
        let base_angle = if launch_side > 0.0 {
            std::f32::consts::PI
        } else {
            0.0
        };
        let angle = base_angle + spread;
        let speed = rng.gen_range(350.0..500.0);

        self.center_ball(launch_side);
        self.ball.velocity_x = angle.cos() * speed;
        self.ball.velocity_y = angle.sin() * speed;
        self.serve_delay_remaining = 0.0;
    }

    fn tick_serve_delay(&mut self, dt: f32) {
        if self.serve_delay_remaining <= 0.0 {
            return;
        }
        self.serve_delay_remaining = (self.serve_delay_remaining - dt).max(0.0);
        if self.serve_delay_remaining <= 0.0 {
            self.launch_ball();
        }
    }

    pub fn ball_visible(&self) -> bool {
        self.serve_delay_remaining <= 0.0
    }

    pub fn step(&mut self, dt_seconds: f32, p1_input: u32) {
        if self.phase == GamePhase::GameOver {
            if (p1_input & INPUT_ACTION) != 0 {
                self.init(self.field_width, self.field_height);
            }
            return;
        }
        let dt = dt_seconds.clamp(0.0, MAX_DT);
        let p2_input = self.compute_ai_input();
        self.apply_input(p1_input, p2_input);
        self.move_entities(dt);
        if self.ball_visible() {
            self.collide_paddles();
            self.collide_walls();
            self.resolve_scoring();
        } else {
            self.tick_serve_delay(dt);
        }
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
        if self.ball_visible() {
            self.ball.position_x += self.ball.velocity_x * dt;
            self.ball.position_y += self.ball.velocity_y * dt;
        }

        let paddle_half_height = PADDLE_HEIGHT / 2.0;
        for paddle in &mut self.paddles {
            paddle.position_y += paddle.velocity_y * dt;
            paddle.position_y = paddle
                .position_y
                .clamp(paddle_half_height, self.field_height - paddle_half_height);
        }
    }

    fn collide_paddles(&mut self) {
        let paddle_positions = [PADDLE1_X, PADDLE2_X];

        for (i, &paddle_x) in paddle_positions.iter().enumerate() {
            if !Self::check_paddle_collision(
                self.ball.position_x,
                self.ball.position_y,
                paddle_x,
                self.paddles[i].position_y,
            ) {
                continue;
            }

            let speed = self.ball.velocity_x.hypot(self.ball.velocity_y);
            let paddle_half_h = PADDLE_HEIGHT / 2.0;
            let offset = ((self.ball.position_y - self.paddles[i].position_y) / paddle_half_h)
                .clamp(-1.0, 1.0);
            let angle = offset * MAX_BOUNCE_ANGLE;
            let direction = if i == 0 { 1.0_f32 } else { -1.0 };

            self.ball.velocity_x = direction * speed * angle.cos();
            self.ball.velocity_y = speed * angle.sin();

            let paddle_top = self.paddles[i].position_y - paddle_half_h;
            let paddle_bottom = self.paddles[i].position_y + paddle_half_h;
            let is_face_hit =
                self.ball.position_y >= paddle_top && self.ball.position_y <= paddle_bottom;

            if is_face_hit {
                self.ball.position_x = paddle_x + direction * (PADDLE_WIDTH / 2.0 + BALL_RADIUS);
            } else if self.ball.position_y < paddle_top {
                self.ball.position_y = paddle_top - BALL_RADIUS;
            } else {
                self.ball.position_y = paddle_bottom + BALL_RADIUS;
            }
        }
    }

    fn collide_walls(&mut self) {
        if self.ball.position_x - BALL_RADIUS <= 0.0 {
            self.conceded_by = Some(Player::One);
        } else if self.ball.position_x + BALL_RADIUS >= self.field_width {
            self.conceded_by = Some(Player::Two);
        }

        if self.ball.position_y - BALL_RADIUS <= 0.0
            || self.ball.position_y + BALL_RADIUS >= self.field_height
        {
            self.ball.velocity_y = -self.ball.velocity_y;
            self.ball.position_y = self
                .ball
                .position_y
                .clamp(BALL_RADIUS, self.field_height - BALL_RADIUS);
        }
    }

    fn resolve_scoring(&mut self) {
        let Some(conceder) = self.conceded_by else {
            return;
        };
        let scorer = match conceder {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
        match scorer {
            Player::One => self.player_one_score += 1,
            Player::Two => self.player_two_score += 1,
        }
        let score = match scorer {
            Player::One => self.player_one_score,
            Player::Two => self.player_two_score,
        };

        if score >= self.winning_score {
            self.phase = GamePhase::GameOver;
            self.winner = Some(scorer);
            self.serve_delay_remaining = 0.0;
            self.launch_ball();
        } else {
            let mut rng = rand::thread_rng();
            self.serve_delay_remaining = rng.gen_range(SERVE_DELAY_MIN..SERVE_DELAY_MAX);
            self.center_ball(0.0);
        }
    }

    fn check_paddle_collision(ball_x: f32, ball_y: f32, paddle_x: f32, paddle_y: f32) -> bool {
        let paddle_half_h = PADDLE_HEIGHT / 2.0;
        let paddle_half_w = PADDLE_WIDTH / 2.0;

        let closest_x = ball_x.clamp(paddle_x - paddle_half_w, paddle_x + paddle_half_w);
        let closest_y = ball_y.clamp(paddle_y - paddle_half_h, paddle_y + paddle_half_h);

        let dist_x = ball_x - closest_x;
        let dist_y = ball_y - closest_y;
        let dist_squared = dist_x * dist_x + dist_y * dist_y;

        dist_squared < (BALL_RADIUS * BALL_RADIUS)
    }

    pub fn update_snapshot(&mut self) {
        use SnapshotField::*;

        let ball_visible = self.ball_visible();
        let snapshot = &mut self.snapshot;
        snapshot[BallX.idx()] = self.ball.position_x;
        snapshot[BallY.idx()] = self.ball.position_y;
        snapshot[BallVx.idx()] = self.ball.velocity_x;
        snapshot[BallVy.idx()] = self.ball.velocity_y;
        snapshot[Paddle1X.idx()] = PADDLE1_X;
        snapshot[Paddle1Y.idx()] = self.paddles[0].position_y;
        snapshot[Paddle2X.idx()] = PADDLE2_X;
        snapshot[Paddle2Y.idx()] = self.paddles[1].position_y;
        snapshot[PlayerOneScore.idx()] = self.player_one_score as f32;
        snapshot[PlayerTwoScore.idx()] = self.player_two_score as f32;
        snapshot[FieldWidth.idx()] = self.field_width;
        snapshot[FieldHeight.idx()] = self.field_height;
        snapshot[GamePhase.idx()] = self.phase.as_snapshot_value();
        snapshot[Winner.idx()] = match self.winner {
            Some(player) => player.as_snapshot_value(),
            None => 0.0,
        };
        snapshot[BallVisible.idx()] = if ball_visible { 1.0 } else { 0.0 };
        snapshot[PaddleWidth.idx()] = PADDLE_WIDTH;
        snapshot[PaddleHeight.idx()] = PADDLE_HEIGHT;
    }

    pub fn snapshot_ptr(&self) -> *const f32 {
        self.snapshot.as_ptr()
    }

    pub fn snapshot_len(&self) -> usize {
        STATE_FIELDS
    }
}

#[cfg(test)]
mod tests;
