use fltk::{
    app::{self, App},
    prelude::*,
    window::Window,
};
use pixels::{Pixels, SurfaceTexture};
use std::{cell::RefCell, rc::Rc};

use super::{Chip, Wire, U32};

pub struct Screen {
    pub input: Wire<U32>,
    pub address: Wire<U32>,
    app: App,
    pixels: Pixels,
}

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;

impl Screen {
    pub fn new(input: Wire<U32>, address: Wire<U32>) -> Self {
        let app = app::App::default();
        let mut win = Window::default()
            .with_size(WIDTH as i32, HEIGHT as i32)
            .with_label("Hello Pixels");
        win.make_resizable(true);
        win.end();
        win.show();

        // Handle resize events
        let surface_size = Rc::new(RefCell::new(None));
        let surface_resize = surface_size.clone();
        win.resize_callback(move |win, _x, _y, width, height| {
            let scale_factor = win.pixels_per_unit();
            let width = (width as f32 * scale_factor) as u32;
            let height = (height as f32 * scale_factor) as u32;

            surface_resize.borrow_mut().replace((width, height));
        });

        let pixels = {
            let pixel_width = win.pixel_w() as u32;
            let pixel_height = win.pixel_h() as u32;
            let surface_texture = SurfaceTexture::new(pixel_width, pixel_height, &win);

            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };

        Screen {
            input,
            address,
            app,
            pixels,
        }
    }
}

impl Chip for Screen {
    fn compute(&mut self) {
        let addr = self.address.borrow().clone();
        self.pixels.frame_mut()[addr.0 as usize >> 2] = self.input.borrow().clone().0 as u8;
        self.pixels.frame_mut()[addr.0 as usize >> 2 + 1] =
            (self.input.borrow().clone().0 >> 8) as u8;
        self.pixels.frame_mut()[addr.0 as usize >> 2 + 2] =
            (self.input.borrow().clone().0 >> 16) as u8;
        self.pixels.frame_mut()[addr.0 as usize >> 2 + 3] =
            (self.input.borrow().clone().0 >> 24) as u8;
    }

    fn clk(&mut self) {
        let surface_size = Rc::new(RefCell::new(None));

        if self.app.wait() {
            // Update internal state
            // world.update();

            // Resize the window
            if let Some((width, height)) = surface_size.borrow_mut().take() {
                if let Err(err) = self.pixels.resize_surface(width, height) {
                    println!("pixels.resize_surface {}", err);
                    self.app.quit();
                }
            }

            // Draw the current frame
            // world.draw(pixels.frame_mut());
            if let Err(err) = self.pixels.render() {
                println!("pixels.render {}", err);
                self.app.quit();
            }

            app::flush();
            app::awake();
        }
    }
}
