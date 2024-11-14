use crate::controller::Controller;
use crate::utils::*;
use glam::Mat4;
use std::f32::consts::PI;
use std::ops::Add;
use std::sync::atomic::Ordering;

pub struct Camera {
    pub aspect_ratio: f32,
    pub position: (f32, f32, f32),
    pub desired_pos: (f32, f32, f32),
    pub tween_vel: f32, // units per second
    pub direction: (f32, f32, f32),
    pub rotation: (f32, f32),
    pub WIDTH: f32,
    pub HEIGHT: f32,
    // pub model: glam::Mat4,
    pub view: glam::Mat4,
    pub projection: glam::Mat4,
    // pub PV: glam::Mat4,
    // pub PVM: glam::Mat4,
    pub pv_changed: bool,
    pub camera_speed: f32,
    pub force_camera_update_flag: bool
}

fn b2f(b: bool) -> f32 {
    (b as i8) as f32
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Camera {
        // let mut model = glam::Mat4::IDENTITY;
        let mut view = glam::Mat4::IDENTITY;
        let mut projection = glam::Mat4::perspective_infinite_lh(90f32.to_radians(), (width / height) as f32, 0.0);
        // let mut PV = projection * view;
        // let mut PVM = PV * model;
        Camera {
            aspect_ratio: width / height,
            position: (0.0, 0.0, 0.0),
            desired_pos: (0.0, 0.0, 0.0),
            tween_vel: 1.0,
            direction: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0),
            WIDTH: width,
            HEIGHT: height,
            // model,
            view,
            projection,
            // PV,
            // PVM,
            pv_changed: false,
            camera_speed: 0.2f32,
            force_camera_update_flag: true
        }
    }

    pub fn set_camera_speed(&mut self, speed: f32){
        self.camera_speed = speed;
    }

    pub fn set_pos(&mut self, pos: (f32, f32, f32)) {
        self.desired_pos = pos;
    }
    pub fn hard_set_pos(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
        self.desired_pos = pos;
    }

    pub fn set_dir(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }
    pub fn set_rot(&mut self, rot: (f32, f32)) {
        self.rotation = rot;
    }

    pub fn get_pos(&mut self) -> (f32, f32, f32) {
        self.position
    }

    pub fn get_dir(&mut self) -> (f32, f32, f32) {
        self.direction
    }

    pub fn get_rot(&mut self) -> (f32, f32) {
        self.rotation
    }

    /// Directly controls camera with controller
    pub fn update_vals(&mut self, mmod: f32, ctrl: &Controller) {
        //directly hooks controls to camera

        //rotation angle
        self.rotation.0 = -map(ctrl.mousex as f32, (0f32, self.WIDTH), (0f32, 360f32)).to_radians();
        //elevation angle
        self.rotation.1 = -clamp(
            map(ctrl.mousey as f32, (0f32, self.HEIGHT), (0f32, 360f32)),
            -90f32,
            90f32,
        )
        .to_radians();

        self.direction.0 = -self.rotation.0.cos();
        self.direction.2 = -self.rotation.0.sin();

        self.position.0 += ((-self.rotation.0.cos() * (b2f(ctrl.forward) - b2f(ctrl.backward)))
            + ((self.rotation.0 + (90f32.to_radians())).cos())
                * (b2f(ctrl.right) - b2f(ctrl.left)))
            * mmod;
        self.position.1 += (b2f(ctrl.down) - b2f(ctrl.up)) * mmod;
        self.position.2 += ((-self.rotation.0.sin() * (b2f(ctrl.forward) - b2f(ctrl.backward)))
            + ((self.rotation.0 + (90f32.to_radians())).sin())
                * (b2f(ctrl.right) - b2f(ctrl.left)))
            * mmod;

        self.view = self.lookat_upd8();
    }

    pub fn get_view_mat4(&self) -> Mat4 {
        self.view
    }

    pub fn get_proj_mat4(&self) -> Mat4 {
        self.projection
    }

    // pub fn get_model_mat4(&self) -> Mat4 {
    //     self.model
    // }

    pub fn set_view_mat4(&mut self, new_view: glam::Mat4) {
        self.view = new_view;
        self.pv_changed = true;
    }

    pub fn set_proj_mat4(&mut self, new_proj: glam::Mat4) {
        self.projection = new_proj;
        self.pv_changed = true;
    }

    /// Sets the model mat4 directly and does nothing else.
    // pub fn set_model_mat4(&mut self, new_model: glam::Mat4) {
    //     self.model = new_model;
    // }

    pub fn rotation_to_direction(&self) -> (f32, f32, f32) {
        (
            self.rotation.0.cos() * self.rotation.1.cos(),
            self.rotation.1.sin(),
            self.rotation.0.sin() * self.rotation.1.cos(),
        )
    }

    pub fn magnitudinal_camera_direction(&self, (observer_mag_LtRt, observer_mag_UpDw, observer_mag_FwBw): (f32,f32,f32)) -> (f32, f32, f32) {
        let cam_dir = self.rotation_to_direction();

        // Create orthogonal vectors for left/right and up/down movement
        let cam_right = (cam_dir.2, 0.0, -cam_dir.0);
        let cam_up = (-cam_dir.0 * cam_dir.1, cam_dir.0.powi(2) + cam_dir.2.powi(2), -cam_dir.1 * cam_dir.2);

        // Scale the direction and orthogonal vectors by the magnitudes
        let move_dir = (
            cam_dir.0 * observer_mag_FwBw + cam_right.0 * observer_mag_LtRt + cam_up.0 * observer_mag_UpDw,
            cam_dir.1 * observer_mag_FwBw + cam_right.1 * observer_mag_LtRt + cam_up.1 * observer_mag_UpDw,
            cam_dir.2 * observer_mag_FwBw + cam_right.2 * observer_mag_LtRt + cam_up.2 * observer_mag_UpDw,
        );

        move_dir
    }

    /// Sets the model mat4 and updates the PVM matrix.
    /// Should be used when rendering models
    // pub fn update_model(&mut self, new_model: glam::Mat4) {
    //     self.model = new_model;
    //     self.check_recalc_pv();
    //     self.PVM = self.PV * self.model;
    // }

    //TODO: Add a calc_pv function that uses an internal PV for pre-calculating before PVM

    pub fn calc_pvm(&self, model: &glam::Mat4) -> glam::Mat4 {
        self.projection * self.view * *model
    }

    pub fn lookat_upd8(&mut self) -> Mat4 {
        let res = glam::Mat4::look_at_lh(
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(
                self.rotation.0.cos() as f32 * self.rotation.1.cos() as f32,
                self.rotation.1.sin() as f32,
                self.rotation.0.sin() as f32 * self.rotation.1.cos() as f32,
            ),
            glam::Vec3::new(0f32, 1f32, 0f32),
        ).mul_mat4(&glam::Mat4::from_translation(glam::Vec3::new(self.position.0 as f32, self.position.1 as f32, self.position.2 as f32)));

        res
    }

    // pub fn check_recalc_pv(&mut self){
    //     if self.pv_changed {
    //         self.PV = self.projection * self.view;
    //     }
    // }

    pub fn upd8(&mut self, force_upd8_matrix: bool, delta_time: u128){

        let dt = delta_time as f32;
        // tween
        if self.position != self.desired_pos {
            let delta = subv3f(self.desired_pos,self.position);
            //let movement = mulv3f((delta.0.sqrt(),delta.1.sqrt(),delta.2.sqrt()), (dt/1000.0,dt/1000.0,dt/1000.0));
            // TODO: Camera speed should be time relative (e.g: units per nanosecond )
            let movement = mulv3f(delta,(self.camera_speed,self.camera_speed,self.camera_speed));
            self.position = addv3f(self.position,movement);
            self.view = self.lookat_upd8();
            // FORCE_CAMERA_UPDATE.store(true, Ordering::Relaxed);
            self.force_camera_update_flag = true;
        }else if force_upd8_matrix {
            self.view = self.lookat_upd8();
            // FORCE_CAMERA_UPDATE.store(false, Ordering::Relaxed);
            self.force_camera_update_flag = false;
        }
    }

}

const zoom_help: f32 = 0.1;

/// Calculates a position on a line from an origin to subject
pub fn subject_zoom_pos(origin: (f32,f32,f32), subject: (f32,f32,f32), zoom: f32) -> (f32, f32, f32) {
    (-subject.0,(origin.1+((subject.1-origin.1))*(zoom*zoom_help)),-subject.2)
}

/// Calculates the zoomed position as well as counter-rotation 
pub fn subject_zoom_pos_rot(origin: (f32,f32,f32), subject: (f32,f32,f32), rot: f32, zoom: f32, zoom_to_radius_scale: f32) -> ((f32, f32, f32), (f32,f32)) {
    let radius = zoom_to_radius_scale*(zoom*zoom_help);
    let pos = subject_zoom_pos(origin, subject, zoom);

    // cos(angle), sin(rot)

    let x = f32::cos(rot)*radius;
    let z = f32::sin(rot)*radius;

    let elev_rot = zoom*zoom_help*(PI/2f32);

    ((pos.0+x,pos.1,pos.2+z), (rot,elev_rot))
}