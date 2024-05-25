/// Bindings used by all shaders, governed by the render manager.\
/// E.g: time, camera, debug flags, etc.
pub const BINDGROUP_GLOBAL: u32 = 0;


/// Bindings for the SceneObject.
/// E.g: Model matrix
pub const BINDGROUP_SCENEOBJECT: u32 = 1;

/// Bindings to model data.\
/// E.g: textures, skeleton animation variables, etc.
pub const BINDGROUP_MODEL: u32 = 2;

/// Bindings for GPU-side updates of an entity.
pub const BINDGROUP_ENTITY: u32 = 3;

/// Bindings specific to the current shader.
/// E.g: Light sources, options, etc.
pub const BINDGROUP_SHADER: u32 = 4;