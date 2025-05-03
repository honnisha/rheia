use ahash::AHashMap;
use bevy::prelude::{Bundle, Component, Entity, EntityRef, EntityWorldMut, QueryState, World};
use bevy_ecs::{change_detection::Mut, component::Mutable, query::QueryData};
use common::{chunks::chunk_position::ChunkPosition, utils::vec_remove_item};

/// A wrapper around `bevy::prelude::World`
pub struct Ecs {
    ecs: World,

    // To speed up the speed of retrieving all objects within a chunk.
    chunks_entities: AHashMap<ChunkPosition, Vec<Entity>>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            ecs: World::new(),
            chunks_entities: Default::default(),
        }
    }

    // Registering an entity within a chunk
    fn insert_entity_inside_chunk(&mut self, chunk: ChunkPosition, entity: Entity) {
        self.chunks_entities.entry(chunk).or_default().push(entity);
    }

    // Removing an entity from a chunk
    fn remove_entity_from_chunk(&mut self, chunk: &ChunkPosition, entity: &Entity) {
        if let Some(vec) = self.chunks_entities.get_mut(chunk) {
            vec_remove_item(vec, entity);
        }
    }

    pub fn entity_moved_chunk(&mut self, entity: &Entity, old_chunk: &ChunkPosition, new_chunk: &ChunkPosition) {
        self.remove_entity_from_chunk(old_chunk, entity);
        self.insert_entity_inside_chunk(*new_chunk, *entity);
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B, chunk: ChunkPosition) -> Entity {
        let id = self.ecs.spawn(bundle).id();
        self.insert_entity_inside_chunk(chunk, id);
        id
    }

    pub fn despawn(&mut self, entity: Entity, chunk: Option<ChunkPosition>) -> bool {
        if let Some(c) = chunk {
            self.remove_entity_from_chunk(&c, &entity);
        }
        self.ecs.despawn(entity)
    }

    pub fn get_chunk_entities<'w>(&'w self, chunk: &ChunkPosition) -> Result<Vec<EntityRef<'w>>, ()> {
        if let Some(entities) = self.chunks_entities.get(chunk) {
            let mut borrows = Vec::with_capacity(entities.len());
            for &id in entities {
                borrows.push(self.ecs.get_entity(id).ok().unwrap());
            }
            return Ok(borrows);
        }
        let empty: Vec<EntityRef> = Default::default();
        return Ok(empty);
    }

    pub fn _get_mut<T: Component<Mutability = Mutable>>(&mut self, entity: Entity) -> Option<Mut<T>> {
        self.ecs.get_mut(entity)
    }

    pub fn get_entity(&self, entity: Entity) -> Option<EntityRef> {
        self.ecs.get_entity(entity).ok()
    }

    pub fn entity_mut(&mut self, entity: Entity) -> EntityWorldMut {
        self.ecs.entity_mut(entity)
    }

    pub fn _query<D: QueryData>(&mut self) -> QueryState<D, ()> {
        self.ecs.query::<D>()
    }
}
