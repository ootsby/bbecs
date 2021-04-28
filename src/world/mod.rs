pub mod bitmap;
mod entity_data;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use entity_data::EntityData;
use eyre::Result;
use ggez::audio::SoundData;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::{components::{CastComponents, ComponentData}};
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::ResourcesData;

use self::bitmap::BitMap;
//use self::entity_data::EntityDataTraits;

macro_rules! impl_world_trait {
    ($new_type:ty, $arm:ident) => {
        impl WorldMethods<$new_type> for World {
            fn with_component(&mut self, name: &str, data: $new_type) -> Result<&mut Self> {
                self.entity_data.insert(name, data)?;
                self.bitmap.insert(name)?;
                Ok(self)
            }

            fn add_resource(&mut self, name: String, data: $new_type) {
                self.resources.insert(name, Resource::$arm(data));
            }
        }
    };
}

const TO_BE_DELETED: &str = "to be deleted";
pub const ENTITY_ID: &str = "entity id";

pub type DataWrapper<T> = Rc<RefCell<T>>;

pub trait WorldMethods<T> {
    fn with_component(&mut self, name: &str, data: T) -> Result<&mut Self>;
    fn add_resource(&mut self, name: String, data: T);
}

pub struct World {
    pub entity_data: EntityData,
    resources: ResourcesData,
    is_empty: bool,
    id_reuse_queue: Vec<usize>,
    next_entity_id: usize,
    last_spawned_id: usize,
    bitmap: BitMap,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    ///Register a component name
    pub fn register<S: ToString>(&mut self, name: S) -> Result<()> {        
        
        self.entity_data.register(name.to_string())?;        
        self.bitmap.register(name.to_string())?;
        Ok(())
    }

    ///Spawn an entity - use chaining of with_component to add components
    pub fn spawn_entity(&mut self) -> Result<&mut Self> {
        self.last_spawned_id = if self.id_reuse_queue.is_empty() { self.next_entity_id } else { self.id_reuse_queue.pop().unwrap() };
        if self.last_spawned_id == self.next_entity_id{
            self.next_entity_id += 1;
        }
        //self.entity_data.insert(TO_BE_DELETED, self.last_spawned_id, ComponentData::from_raw_data(false))?;
        self.entity_data.insert(ENTITY_ID, self.last_spawned_id, ComponentData::from_raw_data(self.last_spawned_id))?;
        self.bitmap.insert(self.last_spawned_id, ENTITY_ID)?;
        self.is_empty = false;
        

        Ok(self)
    }

    ///Request active component lists for a set of names
    pub fn query(&self, names: Vec<&str>) -> Result<HashMap<String, Vec<&ComponentData>>> {
        let bitmap_query = self.bitmap.query(names.clone())?;
        // if bitmap_query.len() == 0{
        //     dbg!("waah!");
        // }
        self.entity_data.query(names, bitmap_query)
    }

    ///Get the resource registered under the provided name
    pub fn get_resource<S: Into<String>>(&self, name: S) -> Result<&Rc<RefCell<Resource>>> {
        self.resources.get(&name.into())
    }

    ///Do per-frame world operations.
    ///
    ///Just deletions for now.
    pub fn update(&mut self) -> Result<()> {
        let mut bitmap_indexes_to_delete = self.bitmap.query(vec![TO_BE_DELETED])?;

        self.bitmap
            .delete_entities_by_index(&bitmap_indexes_to_delete)?;

        self.entity_data
            .delete_entities_by_index(&bitmap_indexes_to_delete)?;

        self.id_reuse_queue.append(&mut bitmap_indexes_to_delete);
        Ok(())
    }

    ///Delete entity (deletions are queued and actioned on update call)
    pub fn delete_by_id(&mut self, id: usize) -> Result<()> {
        self.entity_data.insert(TO_BE_DELETED, id, ComponentData::from_raw_data(true))?;
        self.bitmap.insert(id, TO_BE_DELETED)?;
        
        // let query_results = self.query(vec![TO_BE_DELETED, ENTITY_ID])?;
        // let query_to_be_deleted = query_results.get(TO_BE_DELETED).unwrap();
        // let query_ids = query_results.get(ENTITY_ID).unwrap();

        // for (index, component_id) in query_ids.iter().enumerate() {
        //     let wrapped_component_id: &Rc<RefCell<usize>> = component_id.cast()?;
        //     let component_id = wrapped_component_id.borrow();

        //     if *component_id == id {
        //         let wrapped_to_be_deleted: &Rc<RefCell<bool>> =
        //             query_to_be_deleted[index].cast()?;
        //         let mut to_be_deleted = wrapped_to_be_deleted.borrow_mut();
        //         *to_be_deleted = true;
        //     }
        // }
        Ok(())
    }
}

impl Default for World {
    fn default() -> Self {
        let mut entity_data = EntityData::new();
        let mut bitmap = BitMap::new();

        entity_data.register(TO_BE_DELETED.into()).unwrap();
        bitmap.register(TO_BE_DELETED.into()).unwrap();
        entity_data.register(ENTITY_ID.into()).unwrap();
        bitmap.register(ENTITY_ID.into()).unwrap();

        Self {
            entity_data,
            resources: ResourcesData::new(),
            is_empty: true,
            id_reuse_queue: Vec::new(),
            next_entity_id: 0,
            last_spawned_id: 0,
            bitmap,
        }
    }
}

impl_world_trait!(Color, Color);
impl_world_trait!(Mesh, Mesh);
impl_world_trait!(Point, Point);
impl_world_trait!(u32, U32);
impl_world_trait!(f32, F32);
impl_world_trait!(usize, Usize);
impl_world_trait!(bool, Bool);
impl_world_trait!(KeyCode, GgezKeyCode);
impl_world_trait!(String, Marker);
impl_world_trait!(Text, GgezText);
impl_world_trait!(SoundData, GgezSound);
