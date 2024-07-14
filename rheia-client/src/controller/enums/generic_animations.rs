
pub enum GenericAnimations {
    Idle,
    Walk,
}

impl ToString for GenericAnimations {
    fn to_string(&self) -> String {
        let s = match self {
            GenericAnimations::Idle => "idle",
            GenericAnimations::Walk => "walk",
        };
        s.to_string()
    }
}
