const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1500;
const FACTOR: u32 = 4;
const EXPORT_FACTOR: u32 = 2;

#[derive(Clone, Copy)]
struct Complex {
    pub real: f64,
    pub im: f64,
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            real: self.real + rhs.real,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            real: self.real * rhs.real - self.im * rhs.im,
            im: self.real * rhs.im + self.im * rhs.real,
        }
    }
}

impl Complex {
    fn arg_sq(self) -> f64 {
        self.real * self.real + self.im * self.im
    }
}

fn color(t: f64) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}

fn get_pixel(x: f64, y: f64) -> f64 {
    let mut z = Complex { real: 0.0, im: 0.0 };
    let c = Complex { real: x, im: y };
    let max = 1000;
    let mut i = 0;
    while i < max && z.arg_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    return (i as f64 - z.arg_sq().log2().log2()) / (max as f64);
}

use speedy2d::Window;

struct StargazerWindow {
    pos_x: f64,
    pos_y: f64,
    zoom: f64,
}

use speedy2d::color::Color;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

impl WindowHandler for StargazerWindow {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        let mut data: Vec<u8> = Vec::new();

        for y in 0..HEIGHT / FACTOR {
            for x in 0..WIDTH / FACTOR {
                let u = y as f64 / (HEIGHT / FACTOR) as f64;
                let v = x as f64 / (HEIGHT / FACTOR) as f64;
                let t = get_pixel(
                    self.zoom * (v - 0.5) - self.pos_x,
                    self.zoom * (u - 0.5) - self.pos_y,
                );
                data.extend(color((2.0 * t + 0.5) % 1.0))
            }
        }

        let res = graphics
            .create_image_from_raw_pixels(
                speedy2d::image::ImageDataType::RGB,
                speedy2d::image::ImageSmoothingMode::Linear,
                (WIDTH / FACTOR, HEIGHT / FACTOR),
                &data,
            )
            .unwrap();
        let surface =
            speedy2d::shape::Rectangle::from_tuples((0.0, 0.0), (WIDTH as f32, HEIGHT as f32));
        graphics.draw_rectangle_image(surface, &res);
    }

    // If desired, on_mouse_move(), on_key_down(), etc...

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        match virtual_key_code {
            Some(VirtualKeyCode::A) => self.pos_x += 0.1 * self.zoom,
            Some(VirtualKeyCode::D) => self.pos_x -= 0.1 * self.zoom,
            Some(VirtualKeyCode::W) => self.pos_y += 0.4 * self.zoom,
            Some(VirtualKeyCode::S) => self.pos_y -= 0.4 * self.zoom,
            Some(VirtualKeyCode::Q) => self.zoom *= 2.0,
            Some(VirtualKeyCode::E) => self.zoom *= 0.5,
            Some(VirtualKeyCode::R) => self.render(),
            None => {}
            _ => {}
        }
        helper.request_redraw();
    }
}

impl StargazerWindow {
    fn render(&mut self) {
        let mut image_buffer =
            image::ImageBuffer::new(WIDTH * EXPORT_FACTOR, HEIGHT * EXPORT_FACTOR);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let u = y as f64 / (HEIGHT * EXPORT_FACTOR) as f64;
            let v = x as f64 / (HEIGHT * EXPORT_FACTOR) as f64;
            let t = get_pixel(
                self.zoom * (v - 0.5) - self.pos_x,
                self.zoom * (u - 0.5) - self.pos_y,
            );
            *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
        }
        image_buffer.save("mandelbrot.png").unwrap();
    }
}

fn main() {
    let window = Window::new_centered("Stargazer", (WIDTH, HEIGHT)).unwrap();
    window.run_loop(StargazerWindow {
        pos_x: 1.4,
        pos_y: 0.0,
        zoom: 2.5,
    });
}
