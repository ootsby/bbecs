use std::collections::{HashMap};

use eyre::Result;

use crate::errors::BbEcsError;

#[derive(Debug, Default)]
pub struct BitMap {
    component_type_lookup: HashMap<String, u64>,
    entity_map: Vec<u64>,
    length: usize,
}


const MAX_COMPONENT_TYPES:usize = 64;

impl BitMap {
    pub fn new() -> Self {
        Self {
            component_type_lookup: HashMap::new(),
            entity_map: Vec::new(),
            length: 0,
        }
    }

    pub fn max_component_types() -> usize{
        return MAX_COMPONENT_TYPES;
    }

    pub fn register(&mut self, name: String) -> Result<()> {
        if !self.component_type_lookup.contains_key(&name.to_string()){
            if self.component_type_lookup.len() >= MAX_COMPONENT_TYPES {
                return Err(BbEcsError::TooManyComponentTypes(name).into());
            }
            let bitmask = 1 << self.component_type_lookup.len();
            self.component_type_lookup.insert(name.to_string(), bitmask);
        }

        //self.entity_map.insert(name, vec![]);
        Ok(())
    }

    pub fn spawn_entity(&mut self) {
        self.length += 1;
        // for components in &mut self.entity_map.values_mut() {
        //     components.push(false);
        // }
        self.entity_map.push(0);
    }

    ///Add component of named type to indexed entity record
    pub fn insert(&mut self, index:usize, name: &str) -> Result<()> {
        // if let Some(components) = self.entity_map.get_mut(name) {
        //     components[self.length - 1] = true;
        if self.component_type_lookup.contains_key(name){
            if index == self.entity_map.len(){
                self.entity_map.push(0);
            }
            
            self.entity_map[index] |= self.component_type_lookup.get(name).unwrap();
        } else {
            return Err(BbEcsError::BitMapInsertBeforeRegister.into());
        }

        Ok(())
    }

    ///return ids of entities that contain the requested component types
    pub fn query(&self, names: Vec<&str>) -> Result<Vec<usize>> {
        //let mut results = BTreeMap::new();

        //convert list of names to bitfield match
        let mut bitfield_match:u64 = 0;
        for name in names{
            bitfield_match |= self.component_type_lookup.get(name).unwrap();
        }

        //get indices of components that match the bitfield component set
        let ret_ids = self.entity_map
                                    .iter().enumerate()
                                    .filter( |(_, &x)| (x & bitfield_match) == bitfield_match )
                                    .map(|(i, _)| i )
                                    .collect();

        // for name in names {
        //     if let Some(map) = self.entity_map.get(name) {
        //         results.insert(name.to_owned(), map);
        //     } else {
        //         return Err(BbEcsError::BitMapComponentNotFound(name.to_owned()).into());
        //     }
        // }

        Ok(ret_ids)
    }

    // pub fn calculate_component_indexes_to_delete(
    //     &self,
    //     entity_indexes: &[usize],
    // ) -> Result<HashMap<String, Vec<usize>>> {
    //     let mut component_indexes_to_delete = HashMap::new();

    //     for (component_name, bitmap) in &self.entity_map {
    //         let mut indexes_to_delete = vec![];

    //         for entity_index in entity_indexes {
    //             if !bitmap[*entity_index] {
    //                 continue;
    //             }

    //             indexes_to_delete
    //                 .push(entity_index - self.count_falses_before_index(bitmap, *entity_index)?);
    //         }

    //         component_indexes_to_delete.insert(component_name.to_owned(), indexes_to_delete);
    //     }

    //     Ok(component_indexes_to_delete)
    // }

    pub fn delete_entities_by_index(&mut self, entity_indexes: &Vec<usize>) -> Result<()> {

        for &index in entity_indexes{
            self.entity_map[index] = 0;
        }
        // entity_indexes.reverse();
        // for components in self.entity_map.values_mut() {
        //     for entity_index in &entity_indexes {
        //         components.remove(*entity_index);
        //     }
        // }
        // self.length -= entity_indexes.len();
        Ok(())
    }

    // fn count_falses_before_index(&self, components: &[bool], index: usize) -> Result<usize> {
    //     if index >= components.len() {
    //         return Err(BbEcsError::OutOfRangeInVector.into());
    //     }

    //     let total_falses: &Vec<bool> = &components[0..index]
    //         .iter()
    //         .filter(|current| !*current)
    //         .cloned()
    //         .collect();

    //     Ok(total_falses.len())
    // }
}
