//use cgmath::*;
use instant::Duration;
//use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::window::Window;

const LATERAL_SENSITIVITY: f32 = 1.0;
const ZOOM_SENSITIVITY: f32 = 1.0;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub position: [f32; 2],
    pub vertical_scale: f32,
    pub scale_factor: f32,
    pub vertical_resolution: f32,
    pub is_multisampling: i32,
    pub padding: [i32; 2],
}

#[derive(Debug)]
pub struct CameraController {
    x_speed: f32,
    y_speed: f32,
    zoom_speed: f32,
    pub is_multisampling: i32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            x_speed: 0.0,
            y_speed: 0.0,
            zoom_speed: 0.0,
            is_multisampling: 0,
        }
    }

    pub fn apply_controller(&self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();

        camera.position[0] += self.x_speed * camera.vertical_scale * dt * LATERAL_SENSITIVITY;
        camera.position[1] += self.y_speed * camera.vertical_scale * dt * LATERAL_SENSITIVITY;
        camera.vertical_scale += camera.vertical_scale * self.zoom_speed * dt * ZOOM_SENSITIVITY;
        camera.is_multisampling = self.is_multisampling;
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, element_state: ElementState) -> bool {
        println!("{:?}", self);
        let is_pressed = element_state == ElementState::Pressed;
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                //self.position[1] += self.vertical_scale * LATERAL_SENSITIVITY * dt;
                if is_pressed {
                    self.y_speed = 1.0;
                } else {
                    self.y_speed = 0.0;
                }

                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                if is_pressed {
                    self.y_speed = -1.0;
                } else {
                    self.y_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                if is_pressed {
                    self.x_speed = -1.0;
                } else {
                    self.x_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                if is_pressed {
                    self.x_speed = 1.0;
                } else {
                    self.x_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::C => {
                if is_pressed {
                    self.zoom_speed = 1.0;
                } else {
                    self.zoom_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::V => {
                if is_pressed {
                    self.zoom_speed = -1.0;
                } else {
                    self.zoom_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::M => {
                if is_pressed {
                    if self.is_multisampling == 0 {
                        self.is_multisampling = 1
                    } else {
                        self.is_multisampling = 0
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl Camera {
    pub fn new(window: &Window) -> Self {
        Self {
            position: [0.0, 0.0],
            vertical_scale: 1.0,
            scale_factor: window.inner_size().width as f32 / window.inner_size().height as f32,
            vertical_resolution: window.inner_size().height as f32,
            is_multisampling: 0,
            padding: [0, 0],
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
