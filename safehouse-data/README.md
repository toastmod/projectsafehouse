# Safehouse Engine - Asset Data Builder

This crate is for packing assets into data files at build-time to be loaded at runtime.

# Usage

An example `build.rs` is as follows, from the `pong` crate.

```rust
use safehouse_data::{create_file, model::obj::build_obj};
use safehouse_render::vertex_type::ColorVertex;
use slicebytes::cast_bytes;

fn main() {

    // This function will create/overwrite a file with the provided bytes.
    create_file(

        // Define the output asset data file.
        "src/model/paddle.dat",

        // Build an OBJ file into vertices data.
        build_obj(

            // Pass the file as bytes.
            include_bytes!("res/paddle/paddle.obj"),

            // Configure how parsed data is placed in your vertex struct.
            // The parameters for this function are:
            // `position, tex_coord, normal`
            &|p, _, n| {
                ColorVertex {
                    pos: p.clone().into(),
                    color: n.unwrap_or(&(1.0,0.0,1.0)).clone().into(),
                }
            }
        ).as_slice()
    ).expect("Could not write to file!");
}
```
