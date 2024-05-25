use std::{collections::HashMap, sync::Arc};

use obj::{raw::parse_obj, Position};
use safehouse_render::vertex_type::AdvVertex;

pub fn build_obj_adv(objfile_data: &'static [u8]) -> Vec<AdvVertex> {

    let bunny = parse_obj(objfile_data).unwrap();

    bunny.material_libraries.iter().for_each(|x| println!("\tMaterial Library: \"{}\"", x));

    let size = 0usize; 

    let posns = &bunny.positions;
    let tcoords = &bunny.tex_coords;
    let normals = &bunny.normals;

    let mut vertices = vec![];

    for (gid, (group_name, group)) in bunny.groups.iter().enumerate() {
        for range in &group.polygons {

            for poly in &bunny.polygons[range.start..range.end] {

                let mut polygon = vec![];

                match poly {
                    obj::raw::object::Polygon::P(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.clone()];
                            polygon.push(AdvVertex{
                                pos: [pos.0, pos.1, pos.2, pos.3],
                                normal: [0.0, 0.0, 0.0],
                                texcoord: [0.0, 0.0, 0.0],
                                group_id: gid as u32,
                                bone_id: 0,
                            });
                        }                    
                    },
                    obj::raw::object::Polygon::PT(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.0];
                            let tex = &tcoords[pi.1];
                            polygon.push(AdvVertex{
                                pos: [pos.0, pos.1, pos.2, pos.3],
                                normal: [0.0, 0.0, 0.0],
                                texcoord: [tex.0, tex.1, tex.2],
                                group_id: gid as u32,
                                bone_id: 0,
                            });
                        }   
                    },
                    obj::raw::object::Polygon::PN(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.0];
                            let nor = &normals[pi.1];
                            polygon.push(AdvVertex{
                                pos: [pos.0, pos.1, pos.2, pos.3],
                                normal: [nor.0, nor.1, nor.2],
                                texcoord: [0.0, 0.0, 0.0],
                                group_id: gid as u32,
                                bone_id: 0,
                            });
                        }   

                    },
                    obj::raw::object::Polygon::PTN(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.0];
                            let tex = &tcoords[pi.1];
                            let nor = &normals[pi.2];
                            polygon.push(AdvVertex{
                                pos: [pos.0, pos.1, pos.2, pos.3],
                                normal: [nor.0, nor.1, nor.2],
                                texcoord: [tex.0, tex.1, tex.2],
                                group_id: gid as u32,
                                bone_id: 0,
                            });
                        }   


                    },
                }

                vertices.append(&mut polygon);

            }

        }
    }

    vertices

}

pub fn build_obj<V>(
    objfile_data: &'static [u8],
    polygon_f: &dyn Fn(
        &(f32,f32,f32,f32),
        Option<&(f32,f32,f32)>,
        Option<&(f32,f32,f32)>,
    ) -> V
) -> Vec<V> {

    let bunny = parse_obj(objfile_data).unwrap();

    bunny.material_libraries.iter().for_each(|x| println!("\tMaterial Library: \"{}\"", x));

    let size = 0usize; 

    let posns = &bunny.positions;
    let tcoords = &bunny.tex_coords;
    let normals = &bunny.normals;

    let mut vertices = vec![];

    for (gid, (group_name, group)) in bunny.groups.iter().enumerate() {
        for range in &group.polygons {

            for poly in &bunny.polygons[range.start..range.end] {

                let mut polygon = vec![];

                match poly {
                    obj::raw::object::Polygon::P(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.clone()];
                            polygon.push(polygon_f(
                                pos,
                                None,
                                None
                            ));
                        }                    
                    },
                    obj::raw::object::Polygon::PT(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.0];
                            let tex = &tcoords[pi.1];
                            polygon.push(polygon_f(
                                pos,
                                Some(tex),
                                None
                            ));
                        }   
                    },
                    obj::raw::object::Polygon::PN(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.0];
                            let nor = &normals[pi.1];
                            polygon.push(polygon_f(
                                pos,
                                None,
                                Some(nor)
                            ));
                        }   

                    },
                    obj::raw::object::Polygon::PTN(p) => {
                        for pi in p.as_slice() {
                            let pos = &posns[pi.0];
                            let tex = &tcoords[pi.1];
                            let nor = &normals[pi.2];
                            polygon.push(polygon_f(
                                pos,
                                Some(tex),
                                Some(nor)
                            ));
                        }   


                    },
                }

                vertices.append(&mut polygon);

            }

        }
    }

    vertices

}