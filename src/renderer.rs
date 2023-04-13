use image::{self};

use crate::scene::Scene;

pub struct Renderer {
    surface: image::GrayImage,
}

impl Renderer {
    pub fn new(dimensions: (u32, u32)) -> Renderer {
        Renderer {
            surface: image::GrayImage::new(dimensions.0, dimensions.1),
        }
    }
    pub fn draw_scene(&mut self, scene: &Scene) -> image::GrayImage {
        for bond in &scene.bonds {
            let dx = bond.atoms[1].borrow().position.x - bond.atoms[0].borrow().position.x;
            let dy = bond.atoms[1].borrow().position.y - bond.atoms[0].borrow().position.y;
            let inclination = dy / dx;

            for x in
                bond.atoms[0].borrow().position.x as i32..bond.atoms[1].borrow().position.x as i32
            {
                for i in 0..9 {
                    self.set_pixel_brightness(
                        (
                            x as u32,
                            (x as f32 * inclination + i as f32 / 10.0 * inclination).floor() as u32,
                        ),
                        255,
                    );
                    println!("{}", i as f32 / 10.0 * inclination);
                }
            }
        }
        self.surface.clone()
    }
    fn set_pixel_brightness(&mut self, coords: (u32, u32), brightness: u8) {
        self.surface.get_pixel_mut(coords.0, coords.1).0[0] = brightness;
    }
    fn increase_pixel_brightness(&mut self, coords: (u32, u32), delta: u8) {
        self.surface.get_pixel_mut(coords.0, coords.1).0[0] =
            ((self.surface.get_pixel(coords.0, coords.1).0[0] as u32 + delta as u32)
                .clamp(0, u8::MAX as u32) as u8)
                .max(self.surface.get_pixel(coords.0, coords.1).0[0]);
    }
}
