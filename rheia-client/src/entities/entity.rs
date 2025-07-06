use common::chunks::rotation::Rotation;
use godot::{global::lerp, prelude::*};
use network::{
    entities::EntityNetworkComponent,
    messages::{NetworkEntitySkin, NetworkEntityTag},
};

use super::{entity_tag::EntityTag, enums::generic_animations::GenericAnimations, generic_skin::GenericSkin};

enum EntitySkinContainer {
    Generic(Gd<GenericSkin>),
}

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct Entity {
    pub base: Base<Node3D>,

    pitch: f32,

    skin: EntitySkinContainer,
    tag: Option<Gd<EntityTag>>,
    target_position: Option<Vector3>,
}

impl Entity {
    pub fn create(base: Base<Node3D>, components: Vec<EntityNetworkComponent>) -> Self {
        let mut skin: Option<NetworkEntitySkin> = None;
        let mut tag: Option<Gd<EntityTag>> = None;
        for component in components {
            match component {
                EntityNetworkComponent::Tag(t) => {
                    if let Some(t) = t {
                        tag = Some(Gd::<EntityTag>::from_init_fn(|base| EntityTag::create(base, t)));
                    }
                }
                EntityNetworkComponent::Skin(c) => skin = c,
            }
        }
        let skin = skin.expect("imposible to create entity without tag");
        let skin_container = match skin {
            NetworkEntitySkin::Generic => {
                let skin = Gd::<GenericSkin>::from_init_fn(|base| GenericSkin::create(base));
                EntitySkinContainer::Generic(skin)
            }
            NetworkEntitySkin::Fixed(_) => {
                todo!()
            }
        };
        Self {
            base,
            pitch: 0.0,
            skin: skin_container,
            tag,
            target_position: Default::default(),
        }
    }

    pub fn get_current_animation(&self) -> String {
        match &self.skin {
            EntitySkinContainer::Generic(skin) => skin.bind().get_current_animation(),
        }
    }

    /// Horizontal degrees of character look
    pub fn get_yaw(&self) -> f32 {
        self.base().get_rotation_degrees().y
    }

    /// Vertical degrees of character look
    pub fn get_pitch(&self) -> f32 {
        self.pitch.clone()
    }

    pub fn change_tag(&mut self, tag: Option<NetworkEntityTag>) {
        match tag {
            Some(t) => {
                match self.tag.as_mut() {
                    Some(old_tag) => {
                        // Update old tag
                        old_tag.bind_mut().update(t);
                    }
                    None => {
                        // create new tag
                        let new_tag_obj = Gd::<EntityTag>::from_init_fn(|base| EntityTag::create(base, t));
                        self.base_mut().add_child(&new_tag_obj);
                        self.tag = Some(new_tag_obj);
                    }
                }
            }
            None => {
                // Remove tag
                if let Some(mut old_tag) = self.tag.take() {
                    old_tag.queue_free();
                }
            }
        }
    }

    pub fn change_skin(&mut self, _skin: NetworkEntitySkin) {
        unimplemented!("change_skin is not implemented");
    }

    pub fn change_position(&mut self, position: Vector3) {
        self.target_position = Some(position);
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let mut r = self.base().get_rotation_degrees();
        r.y = rotation.yaw % 360.0;
        self.pitch = rotation.pitch;
        self.base_mut().set_rotation_degrees(r);
    }

    pub fn set_pitch(&mut self, new_pitch: f32) {
        self.pitch = new_pitch;
    }

    pub fn get_transform(&self) -> Transform3D {
        self.base().get_transform()
    }

    pub fn trigger_animation(&mut self, animation: GenericAnimations) {
        match &mut self.skin {
            EntitySkinContainer::Generic(skin) => skin.bind_mut().trigger_animation(animation),
        }
    }

    /// Handler responsible for movememt
    ///
    /// movement: new position - old position
    ///
    /// Can be called from player controller or network sync
    pub fn handle_movement(&mut self, movement: Vector3) {
        // let movement = position - e.get_position();
        match &mut self.skin {
            EntitySkinContainer::Generic(skin) => skin.bind_mut().handle_movement(movement),
        }
    }
}

#[godot_api]
impl INode3D for Entity {
    fn ready(&mut self) {
        let mut base = self.base_mut().clone();
        match &self.skin {
            EntitySkinContainer::Generic(skin) => base.add_child(skin),
        }
        if let Some(tag) = self.tag.as_ref() {
            base.add_child(tag);
        }
    }

    fn process(&mut self, _delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("entity");

        let now = std::time::Instant::now();

        // target_position is onlt for network sync
        if let Some(target_position) = self.target_position {
            let current_position = self.base().get_position();

            if current_position.distance_to(target_position) >= 10.0 {
                self.base_mut().set_position(target_position);
                self.target_position = None;
            }

            let l = lerp(
                &current_position.to_variant(),
                &target_position.to_variant(),
                &(0.5).to_variant(),
            );
            let new_position = Vector3::from_variant(&l);
            let old_position = self.base_mut().get_position();
            self.handle_movement(new_position - old_position);
            self.base_mut().set_position(new_position);

            if new_position == target_position {
                self.target_position = None;
            }
        }

        let elapsed = now.elapsed();
        #[cfg(debug_assertions)]
        if elapsed >= crate::WARNING_TIME {
            log::warn!(target: "entity", "&7process lag: {:.2?}", elapsed);
        }
    }
}
