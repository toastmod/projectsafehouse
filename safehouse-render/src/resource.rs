use std::rc::Rc;

use crate::{gpu::shaderprogram::Program, model::ModelData};

pub trait ManagerResource {
    // pub fn get_resource<'a>(self, rm: &'a mut crate::RenderManager) -> &'a Rc<T> {
    //     panic!("Resource load type invalid!")
    // }
    fn fetch(name: &str, rm: &crate::RenderManager) -> Rc<Self>;
    fn fetch_default(rm: &crate::RenderManager) -> Rc<Self>;
}

impl ManagerResource for Program {
    fn fetch(name: &str, rm: &crate::RenderManager) -> Rc<Self> {
        Rc::clone(rm.gpu_state.get_shader(name))
    }
    
    fn fetch_default(rm: &crate::RenderManager) -> Rc<Self> {
        Rc::clone(rm.gpu_state.get_shader("default"))
    }
}

impl ManagerResource for ModelData {
    fn fetch(name: &str, rm: &crate::RenderManager) -> Rc<Self> {
        Rc::clone(rm.get_model(name).expect(&format!("Could not find Model: {}", name)))
    }
    
    fn fetch_default(rm: &crate::RenderManager) -> Rc<Self> {
        Rc::clone(rm.get_model("default").expect("No default model found."))
    }
}


