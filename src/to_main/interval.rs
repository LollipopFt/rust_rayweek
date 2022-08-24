pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn new(i: f32, j: f32) -> Self {
        Self { min: i, max: j }
    }
    pub const EMPTY: Self = Self { min: f32::INFINITY, max: f32::NEG_INFINITY };
    pub const UNIVERSE: Self =
        Self { min: f32::NEG_INFINITY, max: f32::INFINITY };
}
