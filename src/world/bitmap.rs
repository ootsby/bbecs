use std::collections::HashMap;

use eyre::Result;

use crate::errors::BbEcsError;

#[derive(Debug, Default)]
pub struct BitMap {
    component_type_lookup: HashMap<String, u64>,
    entity_map: Vec<u64>,
}

const MAX_COMPONENT_TYPES: usize = 64;

impl BitMap {
    pub fn new() -> Self {
        Self {
            component_type_lookup: HashMap::new(),
            entity_map: Vec::new(),
        }
    }

    pub fn max_component_types() -> usize {
        return MAX_COMPONENT_TYPES;
    }

    pub fn register(&mut self, name: String) -> Result<()> {
        if !self.component_type_lookup.contains_key(&name.to_string()) {
            if self.component_type_lookup.len() >= MAX_COMPONENT_TYPES {
                return Err(BbEcsError::TooManyComponentTypes(name).into());
            }
            let bitmask = 1 << self.component_type_lookup.len();
            self.component_type_lookup.insert(name.to_string(), bitmask);
        }
        Ok(())
    }

    pub fn spawn_entity(&mut self, index: usize) -> Result<()> {
        if index == self.entity_map.len() {
            self.entity_map.push(0);
        }

        Ok(())
    }

    ///Add component of named type to indexed entity record
    pub fn insert(&mut self, index: usize, name: &str) -> Result<()> {
        if let Some(&c_flag) = self.component_type_lookup.get(name) {
            self.entity_map[index] |= c_flag;
        } else {
            return Err(BbEcsError::BitMapInsertBeforeRegister.into());
        }

        Ok(())
    }

    ///Add component of named type to indexed entity record
    pub fn insert_for_many(&mut self, ids: &[usize], name: &str) -> Result<()> {
        if let Some(&c_flag) = self.component_type_lookup.get(name) {
            for &id in ids {
                self.entity_map[id] |= c_flag;
            }
        } else {
            return Err(BbEcsError::BitMapInsertBeforeRegister.into());
        }

        Ok(())
    }

    ///return ids of entities that contain the requested component types
    pub fn query(&self, names: Vec<&str>) -> Result<Vec<usize>> {
        //convert list of names to bitfield match
        let mut bitfield_match: u64 = 0;
        for name in names {
            bitfield_match |= self.component_type_lookup.get(name).unwrap();
        }

        //get indices of components that match the bitfield component set
        let ret_ids = self
            .entity_map
            .iter()
            .enumerate()
            .filter(|(_, &x)| (x & bitfield_match) == bitfield_match)
            .map(|(i, _)| i)
            .collect();

        Ok(ret_ids)
    }

    pub fn delete_entities_by_index(&mut self, entity_indexes: &[usize]) -> Result<()> {
        for &index in entity_indexes {
            self.entity_map[index] = 0;
        }
        Ok(())
    }
}
