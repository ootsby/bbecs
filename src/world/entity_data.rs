use std::collections::HashMap;

use eyre::Result;

use crate::components::ComponentData;
use crate::errors::BbEcsError;

const CAPACITY_STEP: usize = 256;

#[derive(Debug, Default)]
pub struct EntityData {
    pub components: HashMap<String, Vec<ComponentData>>,
    entity_capacity: usize,
}

impl EntityData {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            entity_capacity: CAPACITY_STEP,
        }
    }

    pub fn register(&mut self, name: String) -> Result<()> {
        if self.components.contains_key(&name) {
            return Err(BbEcsError::ComponentAlreadyRegistered(name).into());
        }
        let components = vec![ComponentData::Blank; self.entity_capacity];
        self.components.insert(name, components);
        Ok(())
    }

    ///Get component data from given list of names
    pub fn query(
        &self,
        names: Vec<&str>,
        bitmap: &[usize],
    ) -> Result<HashMap<String, Vec<&ComponentData>>> {
        let mut results = HashMap::new();

        for name in names {
            if let Some(components_list) = self.components.get(name) {
                let mut c_vec = Vec::new();
                for &index in bitmap.iter() {
                    c_vec.push(&components_list[index]);
                }
                results.insert(name.to_string(), c_vec);
            } else {
                return Err(BbEcsError::ComponentNotFound(name.to_owned()).into());
            }
        }

        Ok(results)
    }

    pub fn delete_entities_by_index(&mut self, _indexes_to_delete: &[usize]) -> Result<()> {
        Ok(())
    }

    pub fn spawn_entity(&mut self, index: usize) -> Result<()> {
        if index == self.entity_capacity {
            self.entity_capacity += CAPACITY_STEP;
            for c_vec in self.components.values_mut() {
                let mut blanks = vec![ComponentData::Blank; CAPACITY_STEP];
                c_vec.append(&mut blanks);
            }
        }
        Ok(())
    }

    pub fn insert(&mut self, name: &str, index: usize, data: ComponentData) -> Result<()> {
        if let Some(components) = self.components.get_mut(name) {
            components[index] = data;
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }
}
