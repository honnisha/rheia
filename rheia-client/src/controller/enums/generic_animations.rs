
pub enum GenericAnimations {
    Idle,
    Run,
    Walk,
    Jump,
    Fall,
}

impl ToString for GenericAnimations {
    fn to_string(&self) -> String {
        let s = match self {
            GenericAnimations::Idle => "idle",
            GenericAnimations::Run => "run",
            GenericAnimations::Walk => "walk",
            GenericAnimations::Jump => "jump",
            GenericAnimations::Fall => "fall",
        };
        s.to_string()
    }
}
