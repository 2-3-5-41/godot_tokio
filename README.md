# godot_tokio

This was made to prevent re-typing out the boilerplate for creating a tokio runtime godot object.

## Change Notes

Things that changed!

### 0.2.0

- Removed previous functions: `singleton()`, `runtime()` `runtime_please()`
- Added wrapper functions: `spawn()`, `spawn_blocking()`, `block_on()`
- Changed `godot` version to support `0.2.x` instead of individual patches.
- Changed `tokio` version to support `1.32.x` for support of older versions of tokio (if it's needed?).
- Hopefully a better readme.

## Getting started

To start, let's get `godot_tokio` in your crate, there are two options:

1. Add the crate using [crates.io](https://crates.io/crates/godot_tokio)

*OR*

2. Linking to this repo in your `Cargo.toml` -> `godot_tokio = { git = "https://github.com/2-3-5-41/godot_tokio" }`.

Next, you should add a new `GodotTokio.gdextension` , _or what ever you prefer it to be called_, into your Godot project folder, and add this content in to your `GodotTokio.gdextension` file in order to link your extension to the engine:

```txt
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
reloadable = true

[libraries]
linux.debug.x86_64 =     "res://../rust/target/debug/libgodot_tokio.so"
linux.release.x86_64 =   "res://../rust/target/release/libgodot_tokio.so"
windows.debug.x86_64 =   "res://../rust/target/debug/godot_tokio.dll"
windows.release.x86_64 = "res://../rust/target/release/godot_tokio.dll"
macos.debug =            "res://../rust/target/debug/libgodot_tokio.dylib"
macos.release =          "res://../rust/target/release/libgodot_tokio.dylib"
macos.debug.arm64 =      "res://../rust/target/debug/libgodot_tokio.dylib"
macos.release.arm64 =    "res://../rust/target/release/libgodot_tokio.dylib"
```

Read the [GDExtension Docs](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/gdextension_cpp_example.html#using-the-gdextension-module) for more information on the `.gdextension` file.

Read the [gdext Book](https://godot-rust.github.io/book/intro/hello-world.html) for a better understand of how to setup a gdext project (which this crate assumes you follow).

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
