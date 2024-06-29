use std::fmt::{self, Display, Formatter};

pub enum BodyPart {
    Chest,
    Hands,
    Pants,
    Boots,
    Head,
}

impl Display for BodyPart {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Chest => write!(f, "chest"),
            Self::Hands => write!(f, "hands"),
            Self::Pants => write!(f, "pants"),
            Self::Boots => write!(f, "boots"),
            Self::Head => write!(f, "head"),
        }
    }
}
