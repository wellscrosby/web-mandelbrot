//use cgmath::*;
use instant::Duration;
//use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::window::Window;

const LATERAL_SENSITIVITY: f32 = 0.5;
const ZOOM_SENSITIVITY: f32 = 0.5;

#[derive(Debug)]
pub struct CameraController {
    x_speed: f32,
    y_speed: f32,
    zoom_speed: f32,
    is_multisampling: i32,
    coloring_scheme: i32,
    is_reset: bool,
    max_iterations: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            x_speed: 0.0,
            y_speed: 0.0,
            zoom_speed: 0.0,
            is_multisampling: 0,
            coloring_scheme: 0,
            is_reset: false,
            max_iterations: 1000.0,
        }
    }

    pub fn apply_controller(&mut self, camera: &mut Camera, dt: f32) {
        camera.position[0] += self.x_speed * camera.vertical_scale * dt * LATERAL_SENSITIVITY;
        camera.position[1] += self.y_speed * camera.vertical_scale * dt * LATERAL_SENSITIVITY;
        camera.vertical_scale += camera.vertical_scale * self.zoom_speed * dt * ZOOM_SENSITIVITY;
        camera.is_multisampling = self.is_multisampling;
        camera.coloring_scheme = self.coloring_scheme;
        if self.is_reset {
            camera.position = [0.0, 0.0];
            camera.vertical_scale = 1.0;
            self.is_reset = false
        }
        camera.max_iterations = self.max_iterations as i32;

        println!("max iterations: {}", camera.max_iterations);
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, element_state: ElementState) -> bool {
        println!("{:?}", self);
        let is_pressed = element_state == ElementState::Pressed;
        match key {
            VirtualKeyCode::W => {
                //self.position[1] += self.vertical_scale * LATERAL_SENSITIVITY * dt;
                if is_pressed {
                    self.y_speed = 1.0;
                } else {
                    self.y_speed = 0.0;
                }

                true
            }
            VirtualKeyCode::S => {
                if is_pressed {
                    self.y_speed = -1.0;
                } else {
                    self.y_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::A => {
                if is_pressed {
                    self.x_speed = -1.0;
                } else {
                    self.x_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::D => {
                if is_pressed {
                    self.x_speed = 1.0;
                } else {
                    self.x_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::Up => {
                if is_pressed {
                    self.zoom_speed = -1.0;
                } else {
                    self.zoom_speed = 0.0;
                }
                true
            }
            VirtualKeyCode::Down => {
                if is_pressed {
                    self.zoom_speed = 1.0;
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
            VirtualKeyCode::C => {
                if is_pressed {
                    self.coloring_scheme = (self.coloring_scheme + 1) % 5;
                }
                true
            }
            VirtualKeyCode::R => {
                if is_pressed {
                    self.is_reset = true;
                }
                true
            }
            VirtualKeyCode::Left => {
                if is_pressed {
                    self.max_iterations *= 0.95;
                }
                true
            }
            VirtualKeyCode::Right => {
                if is_pressed {
                    self.max_iterations *= 1.05;
                }
                true
            }
            _ => false,
        }
    }
}

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
    pub coloring_scheme: i32,
    pub max_iterations: i32,
}

impl Camera {
    pub fn new(window: &Window) -> Self {
        Self {
            position: [0.0, 0.0],
            vertical_scale: 1.0,
            scale_factor: window.inner_size().width as f32 / window.inner_size().height as f32,
            vertical_resolution: window.inner_size().height as f32,
            is_multisampling: 0,
            coloring_scheme: 0,
            max_iterations: 1000,
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
