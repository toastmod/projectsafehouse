use crate::utils::*;
use crate::gpu::winit;
use winit::event::{DeviceEvent, MouseScrollDelta};
use winit::keyboard::{Key, SmolStr};

pub type GamepadId = usize;

pub struct Controller {
    pub gamepad_id: Option<GamepadId>,
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
    pub forward: bool,
    pub backward: bool,
    pub left_stick: (f32,f32),
    pub scrollbuf: f32,
    pub scrollclamp: Option<(f32,f32)>,
    pub lclick: bool,
    pub rclick: bool,
    pub mclick: bool,
    pub mousex: f32,
    pub mousey: f32,
    pub sensitivity: (f32, f32),
}

impl Controller {
    pub fn new(gamepad_id: Option<GamepadId>) -> Controller {
        Controller {
            gamepad_id,
            up: false,
            left: false,
            down: false,
            right: false,
            forward: false,
            backward: false,
            left_stick: (0.0,0.0),
            scrollbuf: 0.0,
            scrollclamp: None,
            lclick: false,
            rclick: false,
            mclick: false,
            mousex: 0f32,
            mousey: 0f32,
            sensitivity: (6.0f32, 6.0f32),
        }
    }

    pub fn set_scroll_clamp(&mut self, set_to: Option<(f32,f32)>){
        self.scrollclamp = set_to;
    }

    pub fn keyboard_input(&mut self, key: Key<SmolStr>, pressed: bool) {
        //TODO: manage keybinds
        match key {
            Key::Named(n) => match n {
                _ => (),
            },
            Key::Character(c) => match c.as_str() {
                "w" => self.forward = pressed,
                "a" => self.left = pressed,
                "s" => self.backward = pressed,
                "d" => self.right = pressed,
                "q" => self.up = pressed,
                "e" => self.down = pressed,
                _ => (),
            },
            _ => ()
        };
        self.left_stick = (1.0,1.0);
    }

    // TODO: Gamepad device implementation
    // pub fn gamepad_input(&mut self, gilrs: &mut Gilrs) {

    //     match self.gamepad_id {
    //         None => {
    //             //do nothing
    //         }
    //         Some(id) => {
    //             match gilrs.gamepad(id).axis_data(Axis::LeftStickX) {
    //                 None => {}
    //                 Some(dat) => {
    //                     self.left_stick.0 = dat.value()
    //                 }
    //             }
    //             match gilrs.gamepad(id).axis_data(Axis::LeftStickY) {
    //                 None => {}
    //                 Some(dat) => {
    //                     self.left_stick.1 = dat.value()
    //                 }
    //             }
    //         }
    //     }

    // }

    pub fn mouse_input(&mut self, btn: winit::event::MouseButton, pressed: bool) {
        match btn {
            winit::event::MouseButton::Left => {
                self.lclick = pressed;
            }
            winit::event::MouseButton::Middle => {
                self.mclick = pressed;
            }
            winit::event::MouseButton::Right => {
                self.rclick = pressed;
            }
            _ => {}
        }
    }

    pub fn device_input(&mut self, event: DeviceEvent, screen_calc: (f32,f32), mmod: f32) {
        match event {

            DeviceEvent::MouseMotion { delta } => {
                self.mousex += delta.0 as f32 * mmod;
                self.mousey += delta.1 as f32 * mmod;
            },

            DeviceEvent::MouseWheel { delta } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.scrollbuf += x+y;

                    },
                    MouseScrollDelta::PixelDelta(ppos) => {
                        self.scrollbuf += (ppos.x as f32/screen_calc.0) as f32 +(ppos.y as f32/screen_calc.1) as f32;
                    }
                };

                match self.scrollclamp {
                    None => {}
                    Some(minmax) => {
                        self.scrollbuf = clamp(self.scrollbuf,minmax.0,minmax.1);
                    }
                };

            },
            _ => {}
        }
    }

    pub fn mouse_move(&mut self, delta: (f32, f32)) {
        self.mousex += delta.0 * self.sensitivity.0;
        self.mousey += delta.1 * self.sensitivity.1;
    }

    pub fn set_cursor(&mut self, pos: (f32, f32)) {
        self.mousex = pos.0;
        self.mousey = pos.1;
    }

    pub fn set_sens(&mut self, horz: f32, vert: f32) {
        self.mousex = horz;
        self.mousey = vert;
    }
}
