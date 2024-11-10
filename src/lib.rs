use godot::prelude::*;
use tokio::runtime::{self, Runtime};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct AsyncRuntime {
    base: Base<Object>,
    runtime: Runtime,
}

#[godot_api]
impl IObject for AsyncRuntime {
    fn init(base: Base<Object>) -> Self {
        #[cfg(feature = "single-thread")]
        let runtime = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        #[cfg(feature = "multi-thread")]
        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Self { base, runtime }
    }
}

#[godot_api]
impl AsyncRuntime {
    pub const SINGLETON: &'static str = "Tokio";
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }
}
