
pub enum GenericAnimations {
    Idle,
    Walk,
}

impl GenericAnimations {
    pub fn as_str(&self) -> &'static str {
        match self {
            GenericAnimations::Idle => "animation_model_idle",
            GenericAnimations::Walk => "animation_model_walk",
        }
    }
}
