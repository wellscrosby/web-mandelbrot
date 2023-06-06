//use cgmath::*;
use instant::Duration;
//use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::{event::*, window::Window};
// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub position: [f32; 2],
    pub vertical_scale: f32,
    pub scale_factor: f32,
    pub vertical_resolution: [f32; 4],
}

const LATERAL_SENSITIVITY: f32 = 1.0;
const ZOOM_SENSITIVITY: f32 = 1.0;

impl Camera {
    pub fn new(window: &Window) -> Self {
        Self {
            position: [0.0, 0.0],
            vertical_scale: 1.0,
            scale_factor: window.inner_size().width as f32 / window.inner_size().height as f32,
            vertical_resolution: [window.inner_size().height as f32, 0.0, 0.0, 0.0],
        }
    }

    pub fn process_keyboard(
        &mut self,
        key: VirtualKeyCode,
        state: ElementState,
        dt: Duration,
    ) -> bool {
        println!("{:?}", self);
        let dt = dt.as_secs_f32();
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.position[1] += LATERAL_SENSITIVITY * self.vertical_scale * dt;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.position[1] -= LATERAL_SENSITIVITY * self.vertical_scale * dt;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.position[0] -=
                    LATERAL_SENSITIVITY * self.vertical_scale * self.scale_factor * dt;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.position[0] +=
                    LATERAL_SENSITIVITY * self.vertical_scale * self.scale_factor * dt;
                true
            }
            VirtualKeyCode::R => {
                self.vertical_scale -= self.vertical_scale * ZOOM_SENSITIVITY * dt;
                true
            }
            VirtualKeyCode::F => {
                self.vertical_scale += self.vertical_scale * ZOOM_SENSITIVITY * dt;
                true
            }
            _ => false,
        }
    }

    /* pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    } */
}
