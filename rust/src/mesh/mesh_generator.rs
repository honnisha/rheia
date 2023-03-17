use block_mesh::{ndshape::{ConstShape3u32, ConstShape}, VoxelVisibility, Voxel, MergeVoxel, ilattice::prelude::Vec3A, RIGHT_HANDED_Y_UP_CONFIG, UnitQuadBuffer, visible_block_faces};

#[derive(Clone, Copy, Eq, PartialEq)]
struct BoolVoxel(bool);

const EMPTY: BoolVoxel = BoolVoxel(false);

impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BoolVoxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }

    //fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
    //    *self
    //}
}

fn into_domain(array_dim: u32, [x, y, z]: [u32; 3]) -> Vec3A {
    (2.0 / array_dim as f32) * Vec3A::new(x as f32, y as f32, z as f32) - 1.0
}

fn sphere(radius: f32, p: Vec3A) -> BoolVoxel {
    BoolVoxel(p.length() < radius)
}

pub fn generate_buffer() -> UnitQuadBuffer {

    type SampleShape = ConstShape3u32<34, 34, 34>;

    let mut samples = [EMPTY; SampleShape::SIZE as usize];
    for i in 0u32..(SampleShape::SIZE) {
        let p = into_domain(32, SampleShape::delinearize(i));
        samples[i as usize] = sphere(0.9, p);
    }

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        &samples,
        &SampleShape {},
        [0; 3],
        [33; 3],
        &faces,
        &mut buffer,
    );
    buffer
}
