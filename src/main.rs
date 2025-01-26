use winit::event_loop::{EventLoop};
use winit::event::{Event, WindowEvent};
use winit::window::{WindowBuilder};
use winit::dpi::LogicalSize;
use pixels::{Pixels, SurfaceTexture};
use winit::keyboard::KeyCode;

use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Custom Shader")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let window_size = window.inner_size();
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
    //let mut world = World::new();
    //app.world = world;
    let window_size = window.inner_size();
    let mut world = World::new();
    let mut input = WinitInputHelper::new();
    let mut x_offset = 0;
    let mut y_offset = 0;
    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
           //x_offset = x_offset +1; 
            world.draw(pixels.frame_mut(), x_offset, y_offset);
            let render_result = pixels.render();
            if x_offset < 255 {
                x_offset += 1;
            }
         
        }
       // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            window.request_redraw();
        }
    });
}

struct World {}

impl World {
    fn new() -> Self {
        Self {
        }
    }
    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`pixels::wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&self, frame: &mut [u8], x_offset: u8, y_offset: u8) {
            println!("x_offset: {x_offset:?}");
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as u8;
            let y = (i / WIDTH as usize) as u8;
            //println!("blue: {blue:?}");
            let red = x;
            let green = y;

            let mut blue = x;
            match x.checked_add(x_offset) {
                Some(v) => {
                    blue = v;
                }
                None => {
                    blue = x;
                }
            };

            //println!("blue : {blue:?}");
            let rgba = [x, y, blue, 255];

            pixel.copy_from_slice(&rgba);
        }
    }
}
