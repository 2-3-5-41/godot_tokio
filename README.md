# godot_tokio

This was made to prevent re-typing out the boilerplate for creating a tokio runtime godot object.

## Example Use snippets

You can simply create an engine singleton that makes the Tokio runtime accessable by all godot objects.

```rs
#[gdextension]
unsafe impl ExtensionLibrary for Metaphy {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                engine.register_singleton(
                    GdAsyncRuntime::SINGLETON,
                    &GdAsyncRuntime::new_alloc(),
                );
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                if let Some(async_singleton) = engine.get_singleton(GdAsyncRuntime::SINGLETON)
                {
                    engine.unregister_singleton(GdAsyncRuntime::SINGLETON);
                    async_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        GdAsyncRuntime::SINGLETON
                    );
                }
            }
            _ => (),
        }
    }
}
```

And retrive the runtime in your Node(s) like so.

```rs
let tokio = match Engine::singleton().get_singleton(GdAsyncRuntime::SINGLETON) {
    Some(object) => object.cast::<GdAsyncRuntime>(),
    None => return godot_error!("Failed to get singleton -> {}", GdAsyncRuntime::SINGLETON),
};
```
