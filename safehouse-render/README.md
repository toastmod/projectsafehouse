# Safehouse Engine - Rendering Manager

A manager consisting of medium-level definitions for graphical objects.

It's goal is simply to render a queue of objects into a scene, and manage the shaders and pipelines used to render those objects.

## SceneObjects

Instances of the `SceneObject` struct are objects in the current scene that will be rendered.

Each instance has an associated `SceneObjectHandle` that is passed to the `Entity` that is instantiated alongside it.\
Higher-level concepts such as hierarchy or other relationships between SceneObjects are not managed in this crate.

Each SceneObject can be moved or have it's pipeline managed.

## Entities

A struct that implements `Entity` can be represented on the GPU.

An Entity can generally have some functionality over a `SceneObject`, but the possibility of that functionality is up to the implementation. For example, this could be animation or manipulating the shader.
