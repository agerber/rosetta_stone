use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use std::sync::{Arc, Mutex};
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
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let app = Arc::new(Mutex::new(App::default())); 
    
    event_loop.run_app(&mut *app.lock().unwrap());
}
