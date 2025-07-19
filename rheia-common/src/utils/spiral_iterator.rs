use spiral::ChebyshevIterator;

use crate::chunks::chunk_position::ChunkPosition;

pub struct SpiralIterator {
    center: ChunkPosition,
    distance: i64,
    iterator: ChebyshevIterator<i64>,
}

impl SpiralIterator {
    pub fn new(x: i64, y: i64, distance: i64) -> Self {
        let center = ChunkPosition::new(x.clone(), y.clone());
        let iterator = ChebyshevIterator::new(x.clone(), y.clone(), distance.clone());
        Self { center, distance, iterator }
    }
}

impl Iterator for SpiralIterator {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(pos) = self.iterator.next() else {
                break;
            };
            let (x, y) = pos;
            if self.center.get_distance(&ChunkPosition::new(x, y)) < self.distance as f32 {
                return Some((x, y));
            }
        }
        None
    }
}
