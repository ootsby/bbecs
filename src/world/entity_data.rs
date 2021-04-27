use std::collections::{HashMap};

use eyre::Result;

use crate::{errors::BbEcsError};
use crate::{components::ComponentData};

const CAPACITY_STEP: usize = 256;

// pub trait EntityDataTraits<T> {
//     fn insert(&mut entity_data:EntityData, name: &str, index:usize, data: T) -> Result<()>{
//         if let Some(components) = entity_data.components.get_mut(name) {
//             let insert_val = get_inserted_value(data);
//             if index < components.len() {
//                 components[index] = insert_val;
//             }else{
//                 components.push(insert_val);
//             }
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
//     fn get_inserted_value(data: T) -> ComponentData;
// }

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
        //components.reserve(self.entity_capacity);
        self.components.insert(name, components);
        Ok(())
    }

    ///Get component data from given list of names
    pub fn query(
        &self,
        names: Vec<&str>,
    //    bitmap: BTreeMap<String, &Vec<bool>>,
        bitmap: Vec<usize>,
    ) -> Result<HashMap<String, Vec<&ComponentData>>> {
        let mut results = HashMap::new();

        for name in names{
            if let Some(components_list) = self.components.get(name) {
                let mut c_vec = Vec::new();
                for &index in bitmap.iter(){
                    c_vec.push(&components_list[index]);
                }                
                results.insert(name.to_string(), c_vec);
            }else{                
                return Err(BbEcsError::ComponentNotFound(name.to_owned()).into());
            }
        }

        // for (name, map) in bitmap.iter() {
        //     if let Some(components_list) = self.components.get(name) {
        //         let mut missing_components_count = 0;
        //         let mut component_results = vec![];
        //         for (index, component_map) in map.iter().enumerate() {
        //             if *component_map {
        //                 if self.does_entity_have_all_components(&bitmap, index) {
        //                     component_results
        //                         .push(&components_list[index - missing_components_count]);
        //                 }
        //             } else {
        //                 missing_components_count += 1;
        //             }
        //         }
        //         results.insert(name.to_owned(), component_results);
        //     } else {
        //         return Err(BbEcsError::ComponentNotFound(name.to_owned()).into());
        //     }
        // }

        Ok(results)
    }

    // fn does_entity_have_all_components(
    //     &self,
    //     bitmap: &BTreeMap<String, &Vec<bool>>,
    //     entity_index: usize,
    // ) -> bool {
    //     for components in bitmap.values() {
    //         if !components[entity_index] {
    //             return false;
    //         }
    //     }

    //     true
    // }

    pub fn delete_entities_by_index(&mut self, _indexes_to_delete: &Vec<usize>) -> Result<()> {
        
        // for (component_name, mut indexes_to_delete) in bitmap {

        //     indexes_to_delete.reverse();
        //     if let Some(components) = self.components.get_mut(&component_name) {
        //         for index in indexes_to_delete {
        //             components.remove(index);
        //         }
        //     }
        // }

        Ok(())
    }

    pub fn insert(&mut self, name: &str, index:usize, data: ComponentData) -> Result<()>{
        if index == self.entity_capacity {
            self.entity_capacity += CAPACITY_STEP;
            for c_vec in self.components.values_mut() {
                let mut blanks = vec![ComponentData::Blank; CAPACITY_STEP];
                c_vec.append(&mut blanks);
            }
        }
        if let Some(components) = self.components.get_mut(name) {
            components[index] = data;
        } else {
            return Err(BbEcsError::NeedToRegister.into());
        }
        Ok(())
    }

    
}


// impl EntityDataTraits<Point> for EntityData {
//     fn get_inserted_value(data: Point) -> ComponentData{
//         ComponentData::Point(Rc::new(RefCell::new(data)))
//     }
// }

// impl EntityDataTraits<Color> for EntityData {
//     fn insert(&mut self, name: &str, data: Color) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::Color(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<f32> for EntityData {
//     fn insert(&mut self, name: &str, data: f32) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::F32(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<Mesh> for EntityData {
//     fn insert(&mut self, name: &str, data: Mesh) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::Mesh(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<u32> for EntityData {
//     fn insert(&mut self, name: &str, data: u32) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::U32(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<usize> for EntityData {
//     fn insert(&mut self, name: &str, data: usize) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::Usize(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<bool> for EntityData {
//     fn insert(&mut self, name: &str, data: bool) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::Bool(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<KeyCode> for EntityData {
//     fn insert(&mut self, name: &str, data: KeyCode) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::GgezKeyCode(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<String> for EntityData {
//     fn insert(&mut self, name: &str, data: String) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::Marker(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<Text> for EntityData {
//     fn insert(&mut self, name: &str, data: Text) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::GgezText(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }

// impl EntityDataTraits<ggez::audio::SoundData> for EntityData {
//     fn insert(&mut self, name: &str, data: ggez::audio::SoundData) -> Result<()> {
//         if let Some(components) = self.components.get_mut(name) {
//             components.push(ComponentData::GgezSound(Rc::new(RefCell::new(data))));
//         } else {
//             return Err(BbEcsError::NeedToRegister.into());
//         }
//         Ok(())
//     }
// }
