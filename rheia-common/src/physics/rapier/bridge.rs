use rapier3d::na::Vector3 as NaVector3;
use crate::network::messages::{IntoNetworkVector, Vector3 as NetworkVector3};

pub trait IntoNaVector3<T> {
    fn to_na(&self) -> NaVector3<T>;
}

impl IntoNaVector3<f32> for NetworkVector3 {
    fn to_na(&self) -> NaVector3<f32> {
        NaVector3::new(self.x, self.y, self.z)
    }
}

impl IntoNetworkVector for NaVector3<f32> {
    fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x, self.y, self.z)
    }
}
