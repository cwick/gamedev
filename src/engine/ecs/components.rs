#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BounceCollider {
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spin {
    pub value: f32,
}
