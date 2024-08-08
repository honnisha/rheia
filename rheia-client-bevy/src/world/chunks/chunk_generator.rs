use std::borrow::Borrow;

use crate::world::worlds_manager::WorldsManager;

use super::chunk_data_formatter::format_chunk_data_with_boundaries;
use super::chunk_section::ChunkSection;
use super::chunks_map::ChunkMap;
use super::mesh::mesh_generator::generate_chunk_geometry;
use super::near_chunk_data::NearChunksData;
use bevy::prelude::*;
use bevy::prelude::{Assets, Commands, Event, Events, Res, ResMut};
use common::chunks::chunk_position::ChunkPosition;
use common::VERTICAL_SECTIONS;
use log::error;

#[derive(Event)]
pub struct GenerateChunkEvent {
    chunk_position: ChunkPosition,
    near_chunks_data: NearChunksData,
}

impl GenerateChunkEvent {
    pub fn new(chunk_position: ChunkPosition, near_chunks_data: NearChunksData) -> Self {
        Self {
            chunk_position,
            near_chunks_data,
        }
    }
}

pub fn send_chunks_to_load(
    mut worlds_manager: ResMut<WorldsManager>,
    mut chunk_generate_event: EventWriter<GenerateChunkEvent>,
) {
    if let Some(world) = worlds_manager.get_world_mut() {
        let chunks_map: &ChunkMap = world.get_chunks_map().borrow();
        for (chunk_position, chunk_lock) in chunks_map.iter() {
            let c = chunk_lock.read();
            if c.is_sended() {
                continue;
            }

            let near_chunks_data = NearChunksData::new(&world.get_chunks_map().borrow(), &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                continue;
            }

            c.set_sended();
            let e = GenerateChunkEvent::new(chunk_position.clone(), near_chunks_data);
            chunk_generate_event.send(e);
        }
    }
}

pub fn chunk_generator(
    mut commands: Commands,
    mut chunk_generator_event: ResMut<Events<GenerateChunkEvent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in chunk_generator_event.drain() {
        let world = match worlds_manager.get_world() {
            Some(w) => w,
            None => {
                continue;
            }
        };
        let chunk = match world.get_chunks_map().get_chunk(&event.chunk_position) {
            Some(c) => c,
            None => {
                error!("Chunk generator not found chunk: {}", event.chunk_position);
                continue;
            }
        };

        let mut c = chunk.write();
        for y in 0..VERTICAL_SECTIONS {
            let bordered_chunk_data =
                format_chunk_data_with_boundaries(Some(&event.near_chunks_data), &c.get_data(), y);

            // Create test sphere
            // let bordered_chunk_data = get_test_sphere();

            let mesh = generate_chunk_geometry(&bordered_chunk_data);

            let mut pbr = PbrBundle {
                transform: c.get_transform(y as u8),
                material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
                visibility: Visibility::Hidden,
                ..default()
            };
            if mesh.count_vertices() != 0 {
                pbr.mesh = meshes.add(mesh);
                pbr.visibility = Visibility::Visible;
            }

            let entity = commands.spawn(pbr);
            let section = ChunkSection::new(entity.id());
            c.insert_section(section);
        }
    }
}
