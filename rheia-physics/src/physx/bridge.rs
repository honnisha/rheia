use crate::network::messages::{IntoNetworkVector, Vector3 as NetworkVector3};
use physx::prelude::*;

pub trait IntoPxVec3 {
    fn to_physx(&self) -> PxVec3;
    fn to_physx_sys(&self) -> physx_sys::PxVec3;
}

impl IntoPxVec3 for NetworkVector3 {
    fn to_physx(&self) -> PxVec3 {
        PxVec3::new(self.x, self.y, self.z)
    }

    fn to_physx_sys(&self) -> physx_sys::PxVec3 {
        physx_sys::PxVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl IntoNetworkVector for PxVec3 {
    fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x(), self.y(), self.z())
    }
}

impl IntoNetworkVector for physx_sys::PxVec3 {
    fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x, self.y, self.z)
    }
}
