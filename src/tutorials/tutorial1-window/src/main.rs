#![allow(clippy::collapsible_match)]
#![allow(clippy::single_match)]

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn run(event_loop: EventLoop<()>, _window: Window, _swapchain_format: wgpu::TextureFormat) {
    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { ref event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => println!("resized {} x {}", size.height, size.width),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } = input
                    {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => {}
            }
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    println!("Hello, world!");
    run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb);
}
