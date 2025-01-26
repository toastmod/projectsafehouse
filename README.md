# Safehouse Engine Project

An 3D engine with an incremental development style.

The goal of this project is to make a game engine that can be broken down into separate usable modules.

# Module Hierarchy

Each module builds off of other modules, hence being incremental.

-   `safehouse-engine` (TBD): The object manager with higher, game-engine-level concepts.
    -   `safehouse-render`: The rendering manager.
        -   `safehouse-gpu`: The GPU state.

# External Modules

These are modules that are not part of the main hierarchy, and are to be used to aid development.

-   `safehouse-data`
    -   A build-time asset data packager.
-   `safehouse-debug` (WIP)
    -   A debugger that is familiar with all modules.
-   `safehouse-shader` (WIP)
    -   Tools and macros for building shaders.
-   `slicebytes`
    -   Basically just `cast_bytes` from the `bytemuck` crate.
-   `winit-app-handler`
    -   A helper for handling winit startup to reduce duplicate code.

# Demos

-   `pong`
    -   A 2D demo game made from `safehouse-render`.
    -   `cargo run --bin pong`

# Examples

#### `cargo run --example [name]`

-   `hello-triangle`
    -   A simple triangle with `safehouse-gpu`.
-   `hello-sceneobject`
    -   Similar to `hello-triangle` but made in `safehouse-render`.
    -   (triangle spawns off screen, move around with WASD)
-   `text-example`
    -   Simple text rendering example in `safehouse-gpu`.
-   `text-texture-example`
    -   Demo of using dynamic textures in `safehouse-render`.
