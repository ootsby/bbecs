use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use ggez::{audio::SoundData, event::KeyCode};
use ggez::graphics::{Color, Mesh, Text};

use crate::data_types::point::Point;
use crate::errors::BbEcsError;

pub trait CastComponents<T> {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<T>>>;
    fn cast(&self) -> Result<&Rc<RefCell<T>>>;
    fn from_raw_data(data: T) -> ComponentData;
}

/// These components are used to store data into the world. Each of the components contains
/// a vector of the appropriate data. Generally consumers of this library will not need to
/// call Components directly. However the methods attached to components will be used to
/// extract the data.
#[derive(Debug, Clone)]
pub enum ComponentData {
    Point(Rc<RefCell<Point>>),
    F32(Rc<RefCell<f32>>),
    Color(Rc<RefCell<Color>>),
    Mesh(Rc<RefCell<Mesh>>),
    U32(Rc<RefCell<u32>>),
    U64(Rc<RefCell<u64>>),
    Usize(Rc<RefCell<usize>>),
    Bool(Rc<RefCell<bool>>),
    GgezKeyCode(Rc<RefCell<KeyCode>>),
    Marker(Rc<RefCell<String>>),
    GgezText(Rc<RefCell<Text>>),
    GgezSound(Rc<RefCell<ggez::audio::SoundData>>),
    Blank,
}

impl CastComponents<Point> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Point>>> {
        if let Self::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents("Point").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Point>>> {
        if let Self::Point(points) = self {
            Ok(points)
        } else {
            Err(BbEcsError::CastingComponents("Point").into())
        }
    }

    fn from_raw_data(data: Point) -> ComponentData{
        ComponentData::Point(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<f32> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<f32>>> {
        if let Self::F32(numbers) = self {
            Ok(numbers)
        } else {
            Err(BbEcsError::CastingComponents("F32").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<f32>>> {
        if let Self::F32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("F32").into())
        }
    }

    fn from_raw_data(data: f32) -> ComponentData{
        ComponentData::F32(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<Color> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Color>>> {
        if let Self::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingComponents("Color").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Color>>> {
        if let Self::Color(color) = self {
            Ok(color)
        } else {
            Err(BbEcsError::CastingComponents("Color").into())
        }
    }

    fn from_raw_data(data: Color) -> ComponentData{
        ComponentData::Color(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<Mesh> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Mesh>>> {
        if let Self::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingComponents("Mesh").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Mesh>>> {
        if let Self::Mesh(mesh) = self {
            Ok(mesh)
        } else {
            Err(BbEcsError::CastingComponents("Mesh").into())
        }
    }

    fn from_raw_data(data: Mesh) -> ComponentData{
        ComponentData::Mesh(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<u32> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<u32>>> {
        if let Self::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U32").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<u32>>> {
        if let Self::U32(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U32").into())
        }
    }

    fn from_raw_data(data: u32) -> ComponentData{
        ComponentData::U32(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<u64> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<u64>>> {
        if let Self::U64(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U64").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<u64>>> {
        if let Self::U64(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("U64").into())
        }
    }

    fn from_raw_data(data: u64) -> ComponentData{
        ComponentData::U64(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<usize> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<usize>>> {
        if let Self::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("Usize").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<usize>>> {
        if let Self::Usize(number) = self {
            Ok(number)
        } else {
            Err(BbEcsError::CastingComponents("Usize").into())
        }
    }

    fn from_raw_data(data: usize) -> ComponentData{
        ComponentData::Usize(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<bool> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<bool>>> {
        if let Self::Bool(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("Bool").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<bool>>> {
        if let Self::Bool(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("Bool").into())
        }
    }

    fn from_raw_data(data: bool) -> ComponentData{
        ComponentData::Bool(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<KeyCode> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<KeyCode>>> {
        if let Self::GgezKeyCode(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("GgezKeyCode").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<KeyCode>>> {
        if let Self::GgezKeyCode(value) = self {
            Ok(value)
        } else {
            Err(BbEcsError::CastingComponents("GgezKeyCode").into())
        }
    }

    fn from_raw_data(data: KeyCode) -> ComponentData{
        ComponentData::GgezKeyCode(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<String> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<String>>> {
        if let Self::Marker(string) = self {
            Ok(string)
        } else {
            Err(BbEcsError::CastingComponents("Marker").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<String>>> {
        if let Self::Marker(string) = self {
            Ok(string)
        } else {
            Err(BbEcsError::CastingComponents("Marker").into())
        }
    }

    fn from_raw_data(data: String) -> ComponentData{
        ComponentData::Marker(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<Text> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<Text>>> {
        if let Self::GgezText(text) = self {
            Ok(text)
        } else {
            Err(BbEcsError::CastingComponents("GgezText").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<Text>>> {
        if let Self::GgezText(text) = self {
            Ok(text)
        } else {
            Err(BbEcsError::CastingComponents("GgezText").into())
        }
    }

    fn from_raw_data(data: Text) -> ComponentData{
        ComponentData::GgezText(Rc::new(RefCell::new(data)))
    }
}

impl CastComponents<ggez::audio::SoundData> for ComponentData {
    fn cast_mut(&mut self) -> Result<&mut Rc<RefCell<ggez::audio::SoundData>>> {
        if let Self::GgezSound(data) = self {
            Ok(data)
        } else {
            Err(BbEcsError::CastingComponents("GgezSound").into())
        }
    }

    fn cast(&self) -> Result<&Rc<RefCell<ggez::audio::SoundData>>> {
        if let Self::GgezSound(data) = self {
            Ok(data)
        } else {
            Err(BbEcsError::CastingComponents("GgezSound").into())
        }
    }

    fn from_raw_data(data: SoundData) -> ComponentData{
        ComponentData::GgezSound(Rc::new(RefCell::new(data)))
    }
}
pub enum Component {
    Point,
    F32,
    Color,
    Mesh,
    U32,
    U64,
    Usize,
    Bool,
    GgezKeyCode,
    Marker,
    GgezText,
}
