pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use std::{collections::HashMap, sync::Arc, alloc::Allocator};
use obj::{raw::{object::Polygon, parse_obj, RawObj}, ObjResult};


#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Vertexd {
    pub position: [f32; 3],
    pub color_texcoord: [f32; 3],
    pub normal: [f32; 3],
}

pub const fn build_obj(name: &'static str, data: &'static [u8]) -> &'static [u8] {
    let bunny = parse_obj(data).unwrap();
    let mut i = 0;
    while i < bunny.groups.len() {
        let group = bunny.groups.get(bunny.groups.keys().next().unwrap()).unwrap();

        let name_parsed = name.replace("_Mesh", "");

        const polygons: &Vec<Polygon> = &bunny.polygons;
        let positions = &bunny.positions;
        let tex_coords = &bunny.tex_coords;
        let normals = &bunny.normals;

        const lol: usize = polygons.len()*3;        

        let mut vb: [Vertexd; ];

        let mut pos = (0.0, 0.0, 0.0, 0.0);
        let mut tex = (0.0, 0.0, 0.0);
        let mut nor = (0.0, 0.0, 0.0);

        let mut i2 = 0;
        while i2 < polygons.len() {
            match &polygons[i] {
                Polygon::P(p) => {
                    // Process Polygon::P
                    pos = positions[p[0]];
                }
                Polygon::PT(pt) => {
                    // Process Polygon::PT
                    pos = positions[pt[0]];
                    tex = tex_coords[pt[1]];

                }
                Polygon::PN(pn) => {
                    // Process Polygon::PN
                    pos = positions[pn[0]];
                    nor = normals[pn[2]];

                }
                Polygon::PTN(ptn) => {
                    // Process Polygon::PTN
                    pos = positions[ptn[0]];
                    tex = tex_coords[ptn[1]];
                    nor = normals[ptn[2]];

                }
            }
            i += 1;
        }
        i += 1;
    }
    &[]
}
