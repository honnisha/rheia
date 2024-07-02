
pub enum GenericAnimations {
    Idle,
    Walk,
}

impl GenericAnimations {
    pub fn as_str(&self) -> &'static str {
        match self {
            GenericAnimations::Idle => "idle",
            GenericAnimations::Walk => "walk",
        }
    }
}
