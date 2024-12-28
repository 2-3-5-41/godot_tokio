use std::rc::Rc;

use godot::{classes::Engine, prelude::*};
use tokio::runtime::{self, Runtime};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct AsyncRuntime {
    base: Base<Object>,
    runtime: Rc<Runtime>,
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

        Self {
            base,
            runtime: Rc::new(runtime),
        }
    }
}

#[godot_api]
impl AsyncRuntime {
    pub const SINGLETON: &'static str = "Tokio";

    /// This is here if you need it. If not, just use [`Self::runtime()`] just to get the runtime.
    /// Returns `None` if the singleton has not been properly registered with Godot.
    pub fn singleton() -> Option<Gd<AsyncRuntime>> {
        match Engine::singleton().get_singleton(Self::SINGLETON) {
            Some(singleton) => Some(singleton.cast::<Self>()),
            None => None,
        }
    }

    /// If you want to get streight to the sauce.
    /// Returns `None` if [`Self::singleton()`] returns `None`.
    pub fn runtime() -> Option<Rc<Runtime>> {
        match Self::singleton() {
            Some(singleton) => {
                let bind = singleton.bind();
                let rt = Rc::clone(&bind.runtime);

                Some(rt)
            }
            None => None,
        }
    }

    /// # Panics
    /// Just give me the runtime please!
    pub fn runtime_please() -> Rc<Runtime> {
        Self::runtime().unwrap()
    }
}
