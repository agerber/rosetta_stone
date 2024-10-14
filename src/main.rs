use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use std::sync::{Arc, Mutex};
use std::f32::consts::PI;
use winit::window::{Window, WindowId};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};


#[derive(Default)]
struct App {
    window: Option<Window>,
    pixels: Option<Pixels>,
    angle: f32, 
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        self.window = Some(window);

        let surface_texture = SurfaceTexture::new(800, 600, self.window.as_ref().unwrap());
        self.pixels = Some(Pixels::new(800, 600, surface_texture).unwrap());

        println!("Window has resumed and pixel buffer initialized.");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button pressed; exiting.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw requested.");
                self.update_and_render();
                self.pixels.as_mut().unwrap().render().unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl App {
    fn update_and_render(&mut self) {
        self.angle += 0.05;
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }

        self.draw_polygon();
    }

    fn draw_polygon(&mut self) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        frame.fill(0x00); // Clear the frame.

        let (cx, cy) = (400.0, 300.0); // Center of the window.
        let radius = 100.0;

        // Define a hexagon with six vertices around the origin (0, 0).
        let hexagon_points: Vec<(f32, f32)> = (0..6)
            .map(|i| {
                let theta = i as f32 * PI / 3.0; // 60-degree increments.
                (radius * theta.cos(), radius * theta.sin())
            })
            .collect();

         // Rotate and translate each point to the window center.
        let rotated_points: Vec<(usize, usize)> = hexagon_points
        .iter()
        .map(|(x, y)| Self::rotate_point(*x, *y, self.angle, cx, cy))
        .collect();

        // Draw the polygon by connecting each pair of points.
        for i in 0..6 {
            let (x1, y1) = rotated_points[i];
            let (x2, y2) = rotated_points[(i + 1) % 6]; // Connect to the next point.
            Self::draw_line(x1, y1, x2, y2, frame);
        }
    }
    fn rotate_point(x: f32, y: f32, angle: f32, cx: f32, cy: f32) -> (usize, usize) {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        let rotated_x = cos_theta * x - sin_theta * y;
        let rotated_y = sin_theta * x + cos_theta * y;

        let final_x = rotated_x + cx;
        let final_y = rotated_y + cy;

        (final_x as usize, final_y as usize)
    }

    // Bresenham's line drawing algorithm
    fn draw_line(x1: usize, y1: usize, x2: usize, y2: usize, frame: &mut [u8]) {
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = -(y2 as isize - y1 as isize).abs();
        let mut sx = if x1 < x2 { 1 } else { -1 };
        let mut sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1 as isize;
        let mut y = y1 as isize;

        while x != x2 as isize || y != y2 as isize {
            let index = (y as usize * 800 + x as usize) * 4;
            if index < frame.len() - 4 {
                frame[index] = 0xFF;     // Red
                frame[index + 1] = 0x00; // Green
                frame[index + 2] = 0x00; // Blue
                frame[index + 3] = 0xFF; // Alpha
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
}

}
fn main() {
    let event_loop = EventLoop::new().unwrap();
    let app = Arc::new(Mutex::new(App::default())); 
    
    event_loop.run_app(&mut *app.lock().unwrap());
}
