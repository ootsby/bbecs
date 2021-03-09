mod entity_data;

use std::cell::RefCell;
use std::rc::Rc;

use entity_data::EntityData;
use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh};

use crate::components::{Component, Components};
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::{ResourceDataLens, ResourcesData};

use self::entity_data::EntityDataTraits;

pub trait WorldMethods<T> {
    fn with_component<S: Into<String>>(&mut self, name: S, data: T) -> Result<&mut Self>;
    fn add_resource<S: Into<String>>(&mut self, name: S, data: T);
    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&T>;
    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut T>;
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

    pub fn query_one<S: Into<String>>(&self, name: S) -> Option<&Rc<RefCell<Components>>> {
        self.entity_data.query_one(&name.into())
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            entity_data: EntityData::new(),
            resources: ResourcesData::new(),
        }
    }
}

impl WorldMethods<Point> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Point) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Point) {
        self.resources.insert(name.into(), Resource::Point(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&Point> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut Point> {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<Color> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Color) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Color) {
        self.resources.insert(name.into(), Resource::Color(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&Color> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut Color> {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<Mesh> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Mesh) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Mesh) {
        self.resources.insert(name.into(), Resource::Mesh(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&Mesh> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut Mesh> {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<u32> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: u32) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: u32) {
        self.resources.insert(name.into(), Resource::U32(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&u32> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut u32> {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<f32> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: f32) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: f32) {
        self.resources.insert(name.into(), Resource::F32(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&f32> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut f32> {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<usize> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: usize) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: usize) {
        self.resources.insert(name.into(), Resource::Usize(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&usize> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut usize> {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<bool> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: bool) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: bool) {
        self.resources.insert(name.into(), Resource::Bool(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&bool> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut bool> {
        self.resources.get_mut(&name.into())
    }
}

impl WorldMethods<KeyCode> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: KeyCode) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: KeyCode) {
        self.resources
            .insert(name.into(), Resource::GgezKeyCode(data));
    }

    fn get_resource<S: Into<String>>(&self, name: S) -> Result<&KeyCode> {
        self.resources.get(&name.into())
    }

    fn get_resource_mut<S: Into<String>>(&mut self, name: S) -> Result<&mut KeyCode> {
        self.resources.get_mut(&name.into())
    }
}

#[cfg(test)]
mod tests {

    use super::{Component, Result, World, WorldMethods};
    use crate::components::CastComponents;
    use ggez::event::KeyCode;

    #[test]
    fn should_get_key_code_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("keycode", KeyCode::A);
        let keycode: &KeyCode = world.get_resource("keycode")?;
        assert_eq!(*keycode, KeyCode::A);
        Ok(())
    }

    #[test]
    fn should_get_key_code_component() -> Result<()> {
        let mut world = World::new();
        world.register("keycode", Component::GgezKeyCode);
        world.spawn_entity().with_component("keycode", KeyCode::A)?;
        let wrapped_keycodes = world.query_one("keycode").unwrap().borrow();
        let keycodes: &Vec<KeyCode> = wrapped_keycodes.cast()?;
        assert_eq!(keycodes[0], KeyCode::A);
        Ok(())
    }

    #[test]
    fn should_mutably_get_key_code_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("keycode", KeyCode::A);
        let keycode: &mut KeyCode = world.get_resource_mut("keycode")?;
        *keycode = KeyCode::B;
        let keycode: &KeyCode = world.get_resource("keycode")?;
        assert_eq!(*keycode, KeyCode::B);
        Ok(())
    }

    #[test]
    fn should_mutably_get_key_code_component() -> Result<()> {
        let mut world = World::new();
        world.register("keycode", Component::GgezKeyCode);
        world.spawn_entity().with_component("keycode", KeyCode::A)?;
        let mut wrapped_keycodes = world.query_one("keycode").unwrap().borrow_mut();
        let keycodes: &mut Vec<KeyCode> = wrapped_keycodes.cast_mut()?;
        keycodes[0] = KeyCode::B;
        drop(wrapped_keycodes);
        let wrapped_keycodes = world.query_one("keycode").unwrap().borrow();
        let keycodes: &Vec<KeyCode> = wrapped_keycodes.cast()?;
        assert_eq!(keycodes[0], KeyCode::B);
        Ok(())
    }
}
