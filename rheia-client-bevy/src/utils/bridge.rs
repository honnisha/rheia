use bevy::transform::components::Transform;
use common::network::messages::Vector3 as NetworkVector3;

pub trait IntoBevyVector {
    fn to_transform(&self) -> Transform;
}

impl IntoBevyVector for NetworkVector3 {
    fn to_transform(&self) -> Transform {
        Transform::from_xyz(self.x, self.y, self.z)
    }
}
