use safehouse_data::{create_file, render::vertex_type::*};

fn main() {
    let obj = safehouse_data::model::obj::build_obj::<TexVertex>(
        include_bytes!("res/obj/bunny/bunny.obj"),
        &|p,t,n, group_id| {
            let tt = t.unwrap();
            TexVertex { pos: p.clone().into(), tex_coord: [tt.0, tt.1]  }
        }
    );

   create_file("src/model/bunny.dat", &obj).expect("Could not create file!");

}