use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use common::chunks::utils::SectionsData;
use godot::prelude::Gd;
use parking_lot::RwLock;

use super::{chunk_generator::ChunksGenerationType, chunk_column::ChunkColumn};

pub type ColumnDataType = Arc<RwLock<SectionsData>>;
use flume::{Receiver, Sender};

/// Container of godot chunk entity and blocks data
pub struct Chunk {
    // Godot entity
    chunk_column: Option<Gd<ChunkColumn>>,

    // Chunk data
    data: ColumnDataType,

    pub(crate) update_tx: Sender<ChunksGenerationType>,
    pub(crate) update_rx: Receiver<ChunksGenerationType>,

    sended: Arc<AtomicBool>,
    loaded: Arc<AtomicBool>,
}

impl Chunk {
    pub(crate) fn create(sections: SectionsData) -> Self {
        let (update_tx, update_rx) = flume::bounded(1);
        Self {
            chunk_column: None,
            data: Arc::new(RwLock::new(sections)),
            sended: Arc::new(AtomicBool::new(false)),
            loaded: Arc::new(AtomicBool::new(false)),
            update_tx: update_tx,
            update_rx: update_rx,
        }
    }

    pub fn get_chunk_column_mut(&mut self) -> Option<&mut Gd<ChunkColumn>> {
        match self.chunk_column.as_mut() {
            Some(c) => Some(c),
            None => None,
        }
    }

    pub fn set_chunk_column(&mut self, column: Gd<ChunkColumn>) {
        self.chunk_column = Some(column);
    }

    pub fn get_chunk_data(&self) -> &ColumnDataType {
        &self.data
    }

    pub fn is_sended(&self) -> bool {
        self.sended.load(Ordering::Relaxed)
    }

    pub fn set_sended(&self) {
        self.sended.store(true, Ordering::Relaxed);
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed)
    }

    pub fn set_loaded(&self) {
        self.loaded.store(true, Ordering::Relaxed);
    }
}
