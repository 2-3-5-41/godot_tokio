use godot::{prelude::*, init::EditorRunBehavior};
pub use godot_tokio::runtime;

struct HelloTokio;

#[gdextension]
unsafe impl ExtensionLibrary for HelloTokio {
    fn editor_run_behavior() -> EditorRunBehavior {
        EditorRunBehavior::ToolClassesOnly
    }
}