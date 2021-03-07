use thiserror::Error;

use crate::components::Components;
use crate::resources::resource::Resource;

#[derive(Debug, Error)]
pub enum BbEcsError {
    #[error("attempted to cast component from {from:?} to {to:?}")]
    CastingComponents { from: Components, to: Components },
    #[error("attempted to cast resource from {from:?} to {to:?}")]
    CastingResource { from: Resource, to: &'static str },
}
