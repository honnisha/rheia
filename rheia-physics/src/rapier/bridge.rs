use common::chunks::position::{IntoNetworkVector, Vector3};
use rapier3d::na::Vector3 as NaVector3;

pub trait IntoNaVector3<T> {
    fn to_na(&self) -> NaVector3<T>;
}

impl IntoNaVector3<f32> for Vector3 {
    fn to_na(&self) -> NaVector3<f32> {
        NaVector3::new(self.x, self.y, self.z)
    }
}

pub(crate) fn na_to_network(other: &NaVector3<f32>) -> Vector3 {
    Vector3::new(other.x, other.y, other.z)
}
