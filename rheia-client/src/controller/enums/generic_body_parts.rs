use strum_macros::Display;

#[derive(Display)]
pub enum BodyPart {
    Chest,
    Hands,
    Pants,
    Boots,
    Head,
}
