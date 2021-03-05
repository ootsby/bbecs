mod entity_data;

use std::cell::RefCell;
use std::rc::Rc;

use entity_data::EntityData;
use ggez::graphics::{Color, Mesh};

use crate::components::{Component, Components};
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::{ResourceDataLens, ResourcesData};

use self::entity_data::EntityDataTraits;

pub trait WorldMethods<T> {
    fn with_component<S: Into<String>>(&mut self, name: S, data: T) -> &mut Self;
    fn add_resource<S: Into<String>>(&mut self, name: S, data: T);
    fn get_resource<S: Into<String>>(&self, name: S) -> &T;
    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> &mut T;
}

pub struct World {
    pub entity_data: EntityData,
    resources: ResourcesData,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<S: Into<String>>(&mut self, name: S, component_type: Component) {
        self.entity_data.register(name.into(), component_type);
    }

    pub fn spawn_entity(&mut self) -> &mut Self {
        self
    }

    pub fn query_one<S: Into<String>>(&self, name: S) -> &Rc<RefCell<Components>> {
        self.entity_data.query_one(&name.into())
    }
}

// todo implement the following methods
// register component
// insert spawn entity
// with component
// query one
// insert resource
// get resource

impl Default for World {
    fn default() -> Self {
        Self {
            entity_data: EntityData::new(),
            resources: ResourcesData::new(),
        }
    }
}

impl WorldMethods<Point> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Point) -> &mut Self {
        self.entity_data.insert(&name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Point) {
        self.resources.insert(name.into(), Resource::Point(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> &Point {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> &mut Point {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<Color> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Color) -> &mut Self {
        self.entity_data.insert(&name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Color) {
        self.resources.insert(name.into(), Resource::Color(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> &Color {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> &mut Color {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<Mesh> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Mesh) -> &mut Self {
        self.entity_data.insert(&name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Mesh) {
        self.resources.insert(name.into(), Resource::Mesh(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> &Mesh {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> &mut Mesh {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<u32> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: u32) -> &mut Self {
        self.entity_data.insert(&name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: u32) {
        self.resources.insert(name.into(), Resource::U32(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> &u32 {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> &mut u32 {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<f32> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: f32) -> &mut Self {
        self.entity_data.insert(&name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: f32) {
        self.resources.insert(name.into(), Resource::F32(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> &f32 {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> &mut f32 {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<usize> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: usize) -> &mut Self {
        self.entity_data.insert(&name.into(), data);
        self
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: usize) {
        self.resources.insert(name.into(), Resource::Usize(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> &usize {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> &mut usize {
        self.resources.get_mut(&name.into())
    }
}
