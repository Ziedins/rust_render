use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "We want animations",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() {
        for i in buffer.iter_mut() {
            // set all pixels to black
            *i = 0;
        }
        
        for i in 20000..40000 {
            // set the pixels from 20000 to 40000 to red
            buffer[i] = 0xf44336;
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
