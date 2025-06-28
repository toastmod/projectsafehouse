use safehouse_data::{create_file, model::obj::build_obj};
use safehouse_render::vertex_type::ColorVertex;
use slicebytes::cast_bytes;

fn main() {
    create_file(
        "src/model/paddle.dat",
        build_obj(
        include_bytes!("res/paddle/paddle.obj"),
            &|p, _, n, group_id| {
                let ncol = n.unwrap_or(&(1.0,0.0,1.0)).clone();
                ColorVertex {
                    pos: p.clone().into(),
                    color: [ncol.0, ncol.1, ncol.2, 1.0],
                }
            }
        ).as_slice()
    ).expect("Could not write to file!");
}