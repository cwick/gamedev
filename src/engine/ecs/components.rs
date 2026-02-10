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
pub struct Collider {
    pub half_w: f32,
    pub half_h: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spin {
    pub value: f32,
}
