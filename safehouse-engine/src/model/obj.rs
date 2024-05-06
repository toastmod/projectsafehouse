// pub fn build_obj<'a>(name: &str, display: &'a gpu::State, customs: &Fn(&str) -> Option<&'static str> ) -> (VertexBuffer<Vertexd>, ModelGroups<u32>) {
//     println!("Preparing test texture...");

//     let test_texture = load_hardcode_texture(display, &MODEL_BANK.iter().find(|x| x.0 == "test").unwrap().2[0].1);
//     println!("Loading model: \"{}\"", name);
//     let bunny_data = MODEL_BANK.iter().find(|x| x.0 == name).unwrap();
//     let bunnyobj: &[u8] = bunny_data.1.0; 
//     let bunny = parse_obj(bunnyobj).unwrap();

//     println!("{:?}", bunny.param_vertices);

//     let mut ibgroups: Vec<(String, GroupData<u32>)> = vec![];
//     let mut vbv: Vec<Vertexd> = bunny
//         .positions
//         .iter()
//         .map(|x| Vertexd {
//             position: [x.0, x.1, x.2],
//             color_texcoord: [1.0, 1.0, 1.0],
//             normal: [0.0, 0.0, 0.0],
//             groupid: [0]
//         })
//         .collect();

//     let mut vbv_checked = vec![false; vbv.len()];

//     bunny.material_libraries.iter().for_each(|x| println!("\tMaterial Library: \"{}\"", x));

//     for (groupid, (name, group)) in bunny.groups.iter().enumerate() {
//         let groupid = [groupid as u32];
//         let name_parsed = name.replace("_Mesh", "");
//         println!("\tLoading group: \"{}\"", name);
        
//         let mut ibv: Vec<u32> = vec![];

//         for polyrange in &group.polygons {
//             for poly in &bunny.polygons[polyrange.start..polyrange.end] {
//                 let mut pos = (0.0, 0.0, 0.0, 0.0);
//                 let mut tex = (0.0, 0.0, 0.0);
//                 let mut nor = (0.0, 0.0, 0.0);
//                 match poly {
//                     Polygon::P(p) => {
//                         for i in p {
//                             //let pos = bunny.positions[i.clone()];
//                             ibv.push(i.clone() as u32);
//                         }
//                     }
//                     Polygon::PT(pt) => {
//                         for i in pt {
//                             pos = bunny.positions[i.0];
//                             tex = bunny.tex_coords[i.1];

//                             if(!vbv_checked[i.0]) {
//                                 // Hasn't been checked yet, just fill it in
//                                 vbv[i.0].color_texcoord = [tex.0, tex.1, 1.0];
//                                 ibv.push(i.0 as u32);
//                                 vbv_checked[i.0] = true;
//                             }else{
//                                 // Was checked, check if the attributes are the same
//                                 let check_tex = tex == vbv[i.0].color_texcoord.into();
//                                 if(check_tex){
//                                     // Attributes are the same, just add the index
//                                     ibv.push(i.0 as u32);
//                                 }else{
//                                     // Attributes are different, add a new vertex
//                                     let ii = vbv.len();
//                                     vbv.push(Vertexd {
//                                         position: [pos.0, pos.1, pos.2],
//                                         color_texcoord: [tex.0, tex.1, 0.0],
//                                         normal: [nor.0, nor.1, nor.2],
//                                         groupid,
//                                     });
//                                     ibv.push(ii as u32);
//                                 }

//                             }

//                         }
//                     }
//                     Polygon::PN(pn) => {
//                         for i in pn {
//                             pos = bunny.positions[i.0];
//                             nor = bunny.normals[i.1];
                            
//                             if(!vbv_checked[i.0]) {
//                                 // Hasn't been checked yet, just fill it in
//                                 vbv[i.0].color_texcoord = [tex.0, tex.1, 1.0];
//                                 ibv.push(i.0 as u32);
//                                 vbv_checked[i.0] = true;
//                             }else{
//                                 // Was checked, check if the attributes are the same
//                                 let check_nor = nor == vbv[i.0].normal.into();
//                                 if(check_nor){
//                                     // Attributes are the same, just add the index
//                                     ibv.push(i.0 as u32);
//                                 }else{
//                                     // Attributes are different, add a new vertex
//                                     let ii = vbv.len();
//                                     vbv.push(Vertexd {
//                                         position: [pos.0, pos.1, pos.2],
//                                         color_texcoord: [tex.0, tex.1, 0.0],
//                                         normal: [nor.0, nor.1, nor.2],
//                                         groupid,
//                                     });
//                                     ibv.push(ii as u32);
//                                 }

//                             }
//                         }
//                     }
//                     Polygon::PTN(ptn) => {
//                         for i in ptn {
//                             pos = bunny.positions[i.0];
//                             tex = bunny.tex_coords[i.1];
//                             nor = bunny.normals[i.2];
                            
//                             if(!vbv_checked[i.0]) {
//                                 // Hasn't been checked yet, just fill it in
//                                 vbv[i.0].color_texcoord = [tex.0, tex.1, 1.0];
//                                 ibv.push(i.0 as u32);
//                                 vbv_checked[i.0] = true;
//                             }else{
//                                 // Was checked, check if the attributes are the same
//                                 let check_tex = tex == vbv[i.0].color_texcoord.into();
//                                 let check_nor = nor == vbv[i.0].normal.into();
//                                 if(check_tex && check_nor){
//                                     // Attributes are the same, just add the index
//                                     ibv.push(i.0 as u32);
//                                 }else{
//                                     // Attributes are different, add a new vertex
//                                     let ii = vbv.len();
//                                     vbv.push(Vertexd {
//                                         position: [pos.0, pos.1, pos.2],
//                                         color_texcoord: [tex.0, tex.1, 0.0],
//                                         normal: [nor.0, nor.1, nor.2],
//                                         groupid,
//                                     });
//                                     ibv.push(ii as u32);
//                                 }

//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         let pipeline = match customs(name.clone().as_str()) {
//             Some(pipeline_name) => display.get_render_pipeline(pipeline_name),
//             None => None, 
//         };

//         let mut group = GroupData {
//             texture: Some(
//                 match bunny_data.2.iter().find(|x| x.0 == name_parsed) {
//                     Some(x) => gpu::texture::load_hardcode_texture(display, &x.1),
//                     None => {
//                         println!("\tWarning: No texture \"{}\" found for group \"{}\" in model \"{}\"", name_parsed, name, bunny_data.0);
//                         test_texture.clone()
//                     }
//                 }
//             ),
//             pipeline,
//             vind: Rc::new(IndexBuffer::new(display, &ibv)),
//         };


//         ibgroups.push((String::from(name), group));
//         // create bindgroup entry
        

//     }

//     if ibgroups.len() == 0 {
//         return (VertexBuffer::new(display, &vbv), ModelGroups::NoData);
//     }else if ibgroups.len() == 1 {
//         return ibgroups.drain(0..).next().map(|(name, data)| (VertexBuffer::new(display, &vbv), ModelGroups::Single{name: name, gdata: data})).unwrap();
//     }

//     (VertexBuffer::new(display, &vbv), ModelGroups::Multiple(ibgroups))
// }