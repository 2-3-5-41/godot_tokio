use godot::prelude::*;

use crate::runtime::TokioRuntime;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct TokioTest {
    #[export]
    pub runtime_node: Option<Gd<TokioRuntime>>
}

#[godot_api]
impl NodeVirtual for TokioTest {
    fn init(_base: Base<Node>) -> Self {
        Self { runtime_node: None }
    }

    fn ready(&mut self) {
        if let Some(rt) = self.runtime_node.as_ref() {
            let bind = rt.bind();
            let handle = bind.get_handle();

            handle.spawn(async {
                godot_print!("Now running on a worker thread, spawned by seperate node!")
            });
        }
    }
}

#[godot_api]
impl TokioTest {}
