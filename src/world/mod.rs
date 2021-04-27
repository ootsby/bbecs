pub mod bitmap;
mod entity_data;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use entity_data::EntityData;
use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::{components::{CastComponents, ComponentData}};
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::ResourcesData;

use self::bitmap::BitMap;
//use self::entity_data::EntityDataTraits;

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

impl WorldMethods<Point> for World {
    fn with_component(&mut self, name: &str, data: Point) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Point) {
        self.resources.insert(name, Resource::Point(data));
    }
}

impl WorldMethods<Color> for World {
    fn with_component(&mut self, name: &str, data: Color) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Color) {
        self.resources.insert(name, Resource::Color(data));
    }
}

impl WorldMethods<Mesh> for World {
    fn with_component(&mut self, name: &str, data: Mesh) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Mesh) {
        self.resources.insert(name, Resource::Mesh(data));
    }
}

impl WorldMethods<u32> for World {
    fn with_component(&mut self, name: &str, data: u32) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: u32) {
        self.resources.insert(name, Resource::U32(data));
    }
}

impl WorldMethods<f32> for World {
    fn with_component(&mut self, name: &str, data: f32) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: f32) {
        self.resources.insert(name, Resource::F32(data));
    }
}

impl WorldMethods<usize> for World {
    fn with_component(&mut self, name: &str, data: usize) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: usize) {
        self.resources.insert(name, Resource::Usize(data));
    }
}

impl WorldMethods<bool> for World {
    fn with_component(&mut self, name: &str, data: bool) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: bool) {
        self.resources.insert(name, Resource::Bool(data));
    }
}

impl WorldMethods<KeyCode> for World {
    fn with_component(&mut self, name: &str, data: KeyCode) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: KeyCode) {
        self.resources.insert(name, Resource::GgezKeyCode(data));
    }
}

impl WorldMethods<String> for World {
    fn with_component(&mut self, name: &str, data: String) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: String) {
        self.resources.insert(name, Resource::Marker(data));
    }
}

impl WorldMethods<Text> for World {
    fn with_component(&mut self, name: &str, data: Text) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Text) {
        self.resources.insert(name, Resource::GgezText(data));
    }
}

impl WorldMethods<ggez::audio::SoundData> for World {
    fn with_component(&mut self, name: &str, data: ggez::audio::SoundData) -> Result<&mut Self> {
        self.entity_data.insert(name, self.last_spawned_id, ComponentData::from_raw_data(data))?;
        self.bitmap.insert(self.last_spawned_id, name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: ggez::audio::SoundData) {
        self.resources.insert(name, Resource::GgezSound(data));
    }
}
