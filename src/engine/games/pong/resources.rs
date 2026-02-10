use crate::engine::ecs::entity::EntityId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PongPhase {
    Playing,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PongPlayer {
    One,
    Two,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PongState {
    pub ball: EntityId,
    pub paddles: [EntityId; 2],
    pub player_one_score: u32,
    pub player_two_score: u32,
    pub phase: PongPhase,
    pub winner: Option<PongPlayer>,
    pub conceded_by: Option<PongPlayer>,
    pub winning_score: u32,
    pub serve_delay_remaining: f32,
}

impl PongState {
    pub fn ball_visible(&self) -> bool {
        self.phase == PongPhase::Playing && self.serve_delay_remaining <= 0.0
    }
}
