# godot_tokio

This was made to prevent re-typing out the boilerplate for creating a tokio runtime godot object.

## Change Notes

Things that changed!

### 0.2.0

- Removed previous functions: `singleton()`, and `runtime_please()`
- Added wrapper functions: `spawn()`, `spawn_blocking()`, `block_on()`
- Changed `godot` version to support `0.2.x` instead of individual patches.
- Changed `tokio` version to support `1.32.x` for support of older versions of tokio (if it's needed?).
- Hopefully a better readme.

## Getting started

To start, let's get `godot_tokio` in your crate, there are two options:

1. Add the crate using [crates.io](https://crates.io/crates/godot_tokio)

*OR*

2. Linking to this repo in your `Cargo.toml` -> `godot_tokio = { git = "https://github.com/2-3-5-41/godot_tokio" }`.

Next, you'll want to add the `AsyncRuntime` object to your engine singletons like so:

```rs
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                // This is where we register our async runtime singleton.
                engine.register_singleton(AsyncRuntime::SINGLETON, &AsyncRuntime::new_alloc());
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                // Here is where we free our async runtime singleton from memory.
                if let Some(async_singleton) = engine.get_singleton(AsyncRuntime::SINGLETON) {
                    engine.unregister_singleton(AsyncRuntime::SINGLETON);
                    async_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        AsyncRuntime::SINGLETON
                    );
                }
            }
            _ => (),
        }
    }
}

```

All that's left is to use `godot_tokio`, here is a basic example:

```rs
...

impl INode for MyAsyncNode {
    ...
    // You can use the tokio runtime in your base functions (i.e `init`, `enter_tree`, `ready`, etc...)
    fn ready(&mut self) {
        AsyncRuntime::spawn(async { todo!("Hello, world!\n From the tokio async runtime!") });
    }
}

impl MyAsyncNode {
    // You can also use the tokio runtime in gdscript accessible functions like so.
    #[func]
    fn async_hello_world() {
        AsyncRuntime::spawn(async { todo!("Hello again, world!\n From the tokio async runtime!") });
    }

    // Even normal internal rust functions.
    fn another_async_hello_world() {
        AsyncRuntime::spawn(async { todo!("You get the point, I hope.") })
    }
}

...
```

You'll notice that there are three functions provided; you have `spawn()`, `spawn_blocking()`, and `block_on()`, they are wrappers around the regular tokio task functions so you can have a familiar API while using async rust in gdext, without having to setup the tokio runtime yourself.
