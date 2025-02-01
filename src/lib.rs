use std::{future::Future, rc::Rc};

use godot::{classes::Engine, prelude::*};
use tokio::{
    runtime::{self, Runtime},
    task::JoinHandle,
};

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

    /// This function has no real use for the user, only to make it easier
    /// for this crate to access the singleton object.
    fn singleton() -> Option<Gd<AsyncRuntime>> {
        match Engine::singleton().get_singleton(Self::SINGLETON) {
            Some(singleton) => Some(singleton.cast::<Self>()),
            None => None,
        }
    }

    /// Get direct access to the ref counted tokio `Runtime` if you need extra control
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

    /// A wrapper function for the [`tokio::spawn`] function.
    pub fn spawn<F>(task: F) -> Option<tokio::task::JoinHandle<<F>::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        match Self::runtime() {
            Some(rt) => Some(rt.spawn(task)),
            None => None,
        }
    }

    /// A wrapper function for the [`tokio::block_on`] function.
    pub fn block_on<F>(task: F) -> Option<F::Output>
    where
        F: Future,
    {
        match Self::runtime() {
            Some(rt) => Some(rt.block_on(task)),
            None => None,
        }
    }

    /// A wrapper function for the [`tokio::spawn_blocking`] function.
    pub fn spawn_blocking<F, R>(&self, func: F) -> Option<JoinHandle<R>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        match Self::runtime() {
            Some(rt) => Some(rt.spawn_blocking(func)),
            None => None,
        }
    }
}
