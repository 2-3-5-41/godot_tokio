use std::sync::Arc;

use godot::prelude::*;
use tokio::runtime::{Handle, Runtime};

#[derive(GodotClass)]
#[class(base = Node)]
pub struct TokioRuntime {
    pub runtime: Option<Arc<Runtime>>,
}

#[godot_api]
impl NodeVirtual for TokioRuntime {
    fn init(_base: Base<Node>) -> Self {
        match Runtime::new() {
            Ok(rt) => Self {
                runtime: Some(Arc::new(rt)),
            },
            Err(e) => {
                godot_error!("{e}");
                Self { runtime: None }
            }
        }
    }
}

#[godot_api]
impl TokioRuntime {
    /// May return the handle to the tokio runtime, or `None` if no runtime handle is obtainable.
    pub fn try_get_handle(&self) -> Option<&Handle> {
        match self.runtime.as_ref() {
            Some(rt) => Some(rt.handle()),
            None => None,
        }
    }

    /// Panics if handle is not found!
    pub fn get_handle(&self) -> &Handle {
        self.try_get_handle().expect("Failed to get handle!")
    }
}
